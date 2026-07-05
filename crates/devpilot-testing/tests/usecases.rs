//! Tests for the `devpilot-core` project use cases, driven by the shared
//! mocks. These live here so `devpilot-core` needs no dev-dependency on
//! `devpilot-testing`.

use std::path::PathBuf;
use std::sync::Arc;

use devpilot_core::entities::{
    ChatMessage, Dependency, Detection, Ecosystem, EdgeKind, FileAst, Framework, FrameworkCategory,
    FunctionDef, ImportDecl, Language, RepositoryId, Role,
};
use devpilot_core::errors::{GitError, ProjectError, RepoScanError, ScanError, StoreError};
use devpilot_core::ports::RecentProjectsStore;
use devpilot_core::usecases::{
    AnalyzeArchitecture, AnalyzeCodeIntelligence, ChatWithRepository, ListRecentProjects,
    OpenProject, RemoveRecentProject, ScanRepository, SearchCode,
};
use devpilot_testing::fixtures;
use devpilot_testing::mocks::{
    MockCodeAnalyzer, MockGitReader, MockLlmProvider, MockProjectScanner, MockRecentProjectsStore,
};
use futures_util::StreamExt;

#[tokio::test]
async fn open_project_builds_metadata_and_records_recent() {
    let git = Arc::new(MockGitReader::new().with_branch("develop"));
    let store = Arc::new(MockRecentProjectsStore::new());
    let use_case = OpenProject::new(git.clone(), store.clone());

    let metadata = use_case
        .execute(fixtures::sample_local_source())
        .await
        .expect("open should succeed");

    // Metadata assembled from the sample fixtures (3 files: 2 Rust, 1 unknown).
    assert_eq!(metadata.name, "sample");
    assert_eq!(metadata.branch, "develop");
    assert_eq!(metadata.file_count, 3);
    assert_eq!(metadata.commit_count, 2);
    assert_eq!(metadata.total_size_bytes, 230);
    assert_eq!(metadata.languages[0].language, Language::Rust);
    assert_eq!(metadata.languages[0].file_count, 2);

    // The project was recorded exactly once, with a store-stamped timestamp.
    let recent = store.list().await.expect("list");
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0].id, RepositoryId::new("fixture/sample"));
    assert_eq!(recent[0].last_opened, 1);
}

#[tokio::test]
async fn open_project_propagates_git_error_without_recording() {
    let git = Arc::new(MockGitReader::new().with_open_error(GitError::EmptyRepository));
    let store = Arc::new(MockRecentProjectsStore::new());
    let use_case = OpenProject::new(git, store.clone());

    let result = use_case.execute(fixtures::sample_local_source()).await;

    assert_eq!(result, Err(ProjectError::Git(GitError::EmptyRepository)));
    assert!(store.is_empty());
}

#[tokio::test]
async fn open_project_propagates_store_error() {
    let git = Arc::new(MockGitReader::new());
    let store = Arc::new(MockRecentProjectsStore::failing(StoreError::Backend(
        "disk full".into(),
    )));
    let use_case = OpenProject::new(git, store);

    let result = use_case.execute(fixtures::sample_local_source()).await;

    assert_eq!(
        result,
        Err(ProjectError::Store(StoreError::Backend("disk full".into())))
    );
}

#[tokio::test]
async fn list_recent_projects_returns_stored_entries() {
    let store =
        Arc::new(MockRecentProjectsStore::new().with_project(fixtures::sample_recent_project()));
    let use_case = ListRecentProjects::new(store);

    let projects = use_case.execute().await.expect("list");

    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].name, "sample");
}

#[tokio::test]
async fn remove_recent_project_deletes_entry() {
    let store =
        Arc::new(MockRecentProjectsStore::new().with_project(fixtures::sample_recent_project()));
    let use_case = RemoveRecentProject::new(store.clone());

    use_case
        .execute(&RepositoryId::new("fixture/sample"))
        .await
        .expect("remove");

    assert!(store.is_empty());
}

#[tokio::test]
async fn scan_repository_assembles_full_report() {
    let git = Arc::new(MockGitReader::new().with_branch("main"));
    let detection = Detection {
        frameworks: vec![Framework {
            name: "React".into(),
            category: FrameworkCategory::Frontend,
            source: "package.json".into(),
        }],
        dependencies: vec![Dependency {
            name: "react".into(),
            version: Some("^18".into()),
            ecosystem: Ecosystem::Npm,
        }],
    };
    let scanner = Arc::new(MockProjectScanner::new().with_detection(detection));
    let use_case = ScanRepository::new(git, scanner);

    let report = use_case
        .execute(fixtures::sample_local_source())
        .await
        .expect("scan should succeed");

    // Languages from the sample tree: 2 Rust, 1 unknown.
    assert_eq!(report.languages[0].language, Language::Rust);
    assert_eq!(report.languages[0].file_count, 2);

    // Structure: sample tree has one top-level dir `src`, three files.
    assert_eq!(report.structure.total_files, 3);
    assert_eq!(report.structure.top_level_dirs, vec!["src".to_string()]);
    assert_eq!(report.structure.notable, vec!["src".to_string()]);

    // Git: two commits, two contributors, newest commit is last_commit.
    assert_eq!(report.git.branch, "main");
    assert_eq!(report.git.commit_count, 2);
    assert_eq!(report.git.contributors.len(), 2);
    assert_eq!(
        report.git.last_commit.as_ref().unwrap().summary,
        "Add library module"
    );

    // Detection passed through.
    assert_eq!(report.frameworks[0].name, "React");
    assert_eq!(report.dependencies[0].name, "react");
}

#[tokio::test]
async fn scan_repository_propagates_git_error() {
    let git = Arc::new(MockGitReader::new().with_open_error(GitError::EmptyRepository));
    let scanner = Arc::new(MockProjectScanner::new());
    let use_case = ScanRepository::new(git, scanner);

    let result = use_case.execute(fixtures::sample_local_source()).await;

    assert_eq!(result, Err(RepoScanError::Git(GitError::EmptyRepository)));
}

#[tokio::test]
async fn scan_repository_propagates_scan_error() {
    let git = Arc::new(MockGitReader::new());
    let scanner = Arc::new(MockProjectScanner::failing(ScanError::Backend("io".into())));
    let use_case = ScanRepository::new(git, scanner);

    let result = use_case.execute(fixtures::sample_local_source()).await;

    assert_eq!(
        result,
        Err(RepoScanError::Scan(ScanError::Backend("io".into())))
    );
}

#[tokio::test]
async fn analyze_architecture_builds_graphs_from_parsed_files() {
    // The sample tree has src/lib.rs and src/main.rs (Rust) plus README.md.
    let git = Arc::new(MockGitReader::new());

    let lib = FileAst {
        path: PathBuf::from("src/lib.rs"),
        language: Language::Rust,
        functions: vec![FunctionDef {
            name: "a".into(),
            start_line: 1,
            end_line: 2,
            is_async: false,
            calls: vec!["b".into()],
        }],
        imports: vec![ImportDecl {
            source: "crate::main".into(),
            line: 1,
        }],
        ..FileAst::default()
    };
    let main = FileAst {
        path: PathBuf::from("src/main.rs"),
        language: Language::Rust,
        functions: vec![FunctionDef {
            name: "b".into(),
            start_line: 1,
            end_line: 2,
            is_async: false,
            calls: vec![],
        }],
        ..FileAst::default()
    };
    let analyzer = Arc::new(MockCodeAnalyzer::new().with_ast(lib).with_ast(main));
    let use_case = AnalyzeArchitecture::new(git, analyzer);

    let model = use_case
        .execute(fixtures::sample_local_source())
        .await
        .expect("analyze");

    // Dependency edge lib.rs -> main.rs (import "crate::main" resolves by stem).
    assert!(model
        .dependency_graph
        .edges
        .iter()
        .any(|e| e.from == "src/lib.rs" && e.to == "src/main.rs" && e.kind == EdgeKind::Imports));

    // Call edge a -> b across files.
    assert!(model
        .call_graph
        .edges
        .iter()
        .any(|e| e.from == "src/lib.rs::a"
            && e.to == "src/main.rs::b"
            && e.kind == EdgeKind::Calls));

    // Folder graph contains the src directory.
    assert!(model.folder_graph.nodes.iter().any(|n| n.id == "src"));
}

#[tokio::test]
async fn chat_with_repository_streams_provider_tokens() {
    let git = Arc::new(MockGitReader::new());
    let provider = Arc::new(MockLlmProvider::new(["Hi", " there"]));
    let use_case = ChatWithRepository::new(git, provider);

    let history = vec![ChatMessage::new(Role::User, "what does lib.rs do?")];
    let mut stream = use_case
        .execute(fixtures::sample_local_source(), "llama3".into(), history)
        .await
        .expect("chat should start");

    let mut tokens = Vec::new();
    while let Some(item) = stream.next().await {
        tokens.push(item.expect("token"));
    }
    assert_eq!(tokens, vec!["Hi", " there"]);
}

#[tokio::test]
async fn chat_with_repository_surfaces_git_error() {
    let git = Arc::new(MockGitReader::new().with_open_error(GitError::EmptyRepository));
    let provider = Arc::new(MockLlmProvider::new(["ignored"]));
    let use_case = ChatWithRepository::new(git, provider);

    let history = vec![ChatMessage::new(Role::User, "hello")];
    let result = use_case
        .execute(fixtures::sample_local_source(), "llama3".into(), history)
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn code_intelligence_finds_dead_code_and_search() {
    // lib.rs: orphan (uncalled, private) + used (called by main). main.rs: main -> used.
    let lib = FileAst {
        path: PathBuf::from("src/lib.rs"),
        language: Language::Rust,
        functions: vec![
            FunctionDef {
                name: "orphan".into(),
                start_line: 1,
                end_line: 2,
                is_async: false,
                calls: vec![],
            },
            FunctionDef {
                name: "used".into(),
                start_line: 5,
                end_line: 6,
                is_async: false,
                calls: vec![],
            },
        ],
        ..FileAst::default()
    };
    let main = FileAst {
        path: PathBuf::from("src/main.rs"),
        language: Language::Rust,
        functions: vec![FunctionDef {
            name: "main".into(),
            start_line: 1,
            end_line: 3,
            is_async: false,
            calls: vec!["used".into()],
        }],
        ..FileAst::default()
    };

    let git = Arc::new(MockGitReader::new());
    let analyzer = Arc::new(MockCodeAnalyzer::new().with_ast(lib).with_ast(main));

    let report = AnalyzeCodeIntelligence::new(git.clone(), analyzer.clone())
        .execute(fixtures::sample_local_source())
        .await
        .expect("analyze");
    assert!(report.dead_code.iter().any(|d| d.name == "orphan"));
    assert!(!report
        .dead_code
        .iter()
        .any(|d| d.name == "used" || d.name == "main"));

    let hits = SearchCode::new(git, analyzer)
        .execute(fixtures::sample_local_source(), "where is orphan".into())
        .await
        .expect("search");
    assert!(hits.iter().any(|h| h.symbol.as_deref() == Some("orphan")));
}
