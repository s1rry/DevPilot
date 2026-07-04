//! Synchronous git2 conversions from libgit2 objects into `devpilot-core`
//! entities. Every function here runs inside `spawn_blocking`.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use devpilot_core::entities::{CommitInfo, FileChurn, FileNode, FileTree, Repository};
use devpilot_core::errors::GitError;

use crate::{commit_hash, file_node, repository_id, repository_name};

/// Maps a git2 error to a generic backend error.
fn backend(error: git2::Error) -> GitError {
    GitError::Backend(error.message().to_string())
}

/// Returns the commit HEAD points to, mapping an unborn branch (a repository
/// with no commits) to [`GitError::EmptyRepository`].
fn head_commit(repo: &git2::Repository) -> Result<git2::Commit<'_>, GitError> {
    match repo.head() {
        Ok(reference) => reference.peel_to_commit().map_err(backend),
        Err(error)
            if error.code() == git2::ErrorCode::UnbornBranch
                || error.code() == git2::ErrorCode::NotFound =>
        {
            Err(GitError::EmptyRepository)
        }
        Err(error) => Err(backend(error)),
    }
}

/// Builds the [`Repository`] descriptor for an opened git repository.
pub fn describe_repository(repo: &git2::Repository, path: &Path) -> Result<Repository, GitError> {
    let commit = head_commit(repo)?;
    Ok(Repository {
        id: repository_id(path),
        name: repository_name(path),
        local_path: path.to_path_buf(),
        head: commit_hash(commit.id()),
    })
}

/// Returns the short branch name for HEAD, or a detached-HEAD marker.
pub fn short_branch(repo: &git2::Repository) -> Result<String, GitError> {
    let head = repo.head().map_err(backend)?;
    if head.is_branch() {
        Ok(head.shorthand().unwrap_or("HEAD").to_string())
    } else {
        Ok("HEAD (detached)".to_string())
    }
}

/// Builds the full file tree at HEAD.
pub fn build_file_tree(repo: &git2::Repository) -> Result<FileTree, GitError> {
    let tree = head_commit(repo)?.tree().map_err(backend)?;
    let children = walk_tree(repo, &tree, Path::new(""))?;
    Ok(FileTree {
        root: FileNode::Directory {
            path: PathBuf::new(),
            children,
        },
    })
}

/// Recursively converts a git tree into file nodes rooted at `prefix`.
fn walk_tree(
    repo: &git2::Repository,
    tree: &git2::Tree<'_>,
    prefix: &Path,
) -> Result<Vec<FileNode>, GitError> {
    let mut nodes = Vec::with_capacity(tree.len());
    for entry in tree.iter() {
        let name = entry.name().unwrap_or_default();
        let path = prefix.join(name);
        match entry.kind() {
            Some(git2::ObjectType::Blob) => {
                let blob = repo.find_blob(entry.id()).map_err(backend)?;
                nodes.push(file_node(path, blob.size() as u64));
            }
            Some(git2::ObjectType::Tree) => {
                let subtree = repo.find_tree(entry.id()).map_err(backend)?;
                let children = walk_tree(repo, &subtree, &path)?;
                nodes.push(FileNode::Directory { path, children });
            }
            // Submodule commits and other entry kinds are ignored.
            _ => {}
        }
    }
    Ok(nodes)
}

/// Returns up to `limit` commits reachable from HEAD, newest first.
pub fn build_history(repo: &git2::Repository, limit: usize) -> Result<Vec<CommitInfo>, GitError> {
    let mut revwalk = repo.revwalk().map_err(backend)?;
    revwalk.push_head().map_err(backend)?;
    revwalk.set_sorting(git2::Sort::TIME).map_err(backend)?;

    let mut commits = Vec::new();
    for oid in revwalk.take(limit) {
        let oid = oid.map_err(backend)?;
        let commit = repo.find_commit(oid).map_err(backend)?;
        let author = commit.author();
        commits.push(CommitInfo {
            hash: commit_hash(commit.id()),
            author_name: author.name().unwrap_or_default().to_string(),
            author_email: author.email().unwrap_or_default().to_string(),
            timestamp: commit.time().seconds(),
            summary: commit.summary().unwrap_or_default().to_string(),
        });
    }
    Ok(commits)
}

/// Counts all commits reachable from HEAD.
pub fn count_commits(repo: &git2::Repository) -> Result<usize, GitError> {
    let mut revwalk = repo.revwalk().map_err(backend)?;
    revwalk.push_head().map_err(backend)?;
    Ok(revwalk.count())
}

/// Computes per-file change counts across the whole history.
pub fn collect_churn(repo: &git2::Repository) -> Result<Vec<FileChurn>, GitError> {
    let mut revwalk = repo.revwalk().map_err(backend)?;
    revwalk.push_head().map_err(backend)?;

    let mut counts: HashMap<PathBuf, (usize, i64)> = HashMap::new();
    for oid in revwalk {
        let oid = oid.map_err(backend)?;
        let commit = repo.find_commit(oid).map_err(backend)?;
        let time = commit.time().seconds();
        let tree = commit.tree().map_err(backend)?;
        let parent_tree = match commit.parent(0) {
            Ok(parent) => Some(parent.tree().map_err(backend)?),
            Err(_) => None,
        };
        let diff = repo
            .diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), None)
            .map_err(backend)?;

        for delta in diff.deltas() {
            if let Some(path) = delta.new_file().path().or_else(|| delta.old_file().path()) {
                let entry = counts.entry(path.to_path_buf()).or_insert((0, time));
                entry.0 += 1;
                entry.1 = entry.1.max(time);
            }
        }
    }

    let mut churn: Vec<FileChurn> = counts
        .into_iter()
        .map(|(path, (commit_count, last_modified))| FileChurn {
            path,
            commit_count,
            last_modified,
        })
        .collect();
    churn.sort_by(|a, b| {
        b.commit_count
            .cmp(&a.commit_count)
            .then_with(|| a.path.cmp(&b.path))
    });
    Ok(churn)
}

/// Reads the UTF-8 content of a file at HEAD.
pub fn read_file_at_head(repo: &git2::Repository, path: &Path) -> Result<String, GitError> {
    let tree = head_commit(repo)?.tree().map_err(backend)?;
    let entry = tree.get_path(path).map_err(|_| GitError::FileNotFound {
        path: path.to_path_buf(),
    })?;
    let blob = repo.find_blob(entry.id()).map_err(backend)?;
    Ok(String::from_utf8_lossy(blob.content()).to_string())
}
