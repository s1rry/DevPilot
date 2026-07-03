//! Behavior tests for the shared mocks: every port double must act like a
//! faithful, configurable stand-in for the real adapter.

use std::path::{Path, PathBuf};

use devpilot_core::entities::{AnalysisProgress, Language, RepositorySource};
use devpilot_core::errors::{AnalysisError, CacheError, GitError};
use devpilot_core::ports::{AnalysisCache, CodeAnalyzer, GitReader, ProgressReporter};
use devpilot_testing::fixtures;
use devpilot_testing::mocks::{
    MockAnalysisCache, MockCodeAnalyzer, MockGitReader, RecordingProgressReporter,
};

fn local_source() -> RepositorySource {
    RepositorySource::LocalPath(PathBuf::from("/tmp/fixture-sample"))
}

#[tokio::test]
async fn git_reader_returns_fixtures_by_default() {
    let reader = MockGitReader::new();

    let repository = reader.open(&local_source()).await.expect("open");
    assert_eq!(repository, fixtures::sample_repository());

    let tree = reader.file_tree(&repository).await.expect("tree");
    assert_eq!(tree.file_count(), 3);

    let content = reader
        .read_file(&repository, Path::new("src/main.rs"))
        .await
        .expect("read_file");
    assert!(content.contains("fn main"));

    assert_eq!(reader.calls(), vec!["open", "file_tree", "read_file"]);
}

#[tokio::test]
async fn git_reader_respects_configured_error_and_history_limit() {
    let reader = MockGitReader::new().with_open_error(GitError::EmptyRepository);
    assert_eq!(
        reader.open(&local_source()).await,
        Err(GitError::EmptyRepository)
    );

    let reader = MockGitReader::new();
    let repository = fixtures::sample_repository();
    let history = reader.history(&repository, 1).await.expect("history");
    assert_eq!(history.len(), 1);
    assert_eq!(history[0].summary, "Add library module");
}

#[tokio::test]
async fn git_reader_reports_missing_files() {
    let reader = MockGitReader::new().without_files();
    let repository = fixtures::sample_repository();

    let result = reader
        .read_file(&repository, Path::new("src/main.rs"))
        .await;
    assert_eq!(
        result,
        Err(GitError::FileNotFound {
            path: PathBuf::from("src/main.rs")
        })
    );
}

#[tokio::test]
async fn analyzer_returns_configured_analysis_and_records_calls() {
    let analysis = fixtures::sample_file_analysis("src/lib.rs");
    let analyzer = MockCodeAnalyzer::new().with_analysis(analysis.clone());

    let mut file = fixtures::sample_source_file();
    file.path = PathBuf::from("src/lib.rs");

    let produced = analyzer.analyze_file(&file).await.expect("analysis");
    assert_eq!(produced, analysis);
    assert_eq!(analyzer.analyzed_paths(), vec![PathBuf::from("src/lib.rs")]);
}

#[tokio::test]
async fn analyzer_fails_unconfigured_files_as_unsupported() {
    let analyzer = MockCodeAnalyzer::new();
    let file = fixtures::sample_source_file();

    let result = analyzer.analyze_file(&file).await;
    assert_eq!(
        result,
        Err(AnalysisError::UnsupportedLanguage {
            path: file.path.clone()
        })
    );
}

#[test]
fn analyzer_detects_language_from_path() {
    let analyzer = MockCodeAnalyzer::new();
    assert_eq!(
        analyzer.detect_language(Path::new("src/app.tsx")),
        Language::TypeScript
    );
    assert_eq!(
        analyzer.detect_language(Path::new("Makefile")),
        Language::Unknown
    );
}

#[tokio::test]
async fn cache_roundtrips_results() {
    let cache = MockAnalysisCache::new();
    let result = fixtures::sample_analysis_result();

    assert!(cache.is_empty());
    let missing = cache
        .get(&result.repository, &result.commit)
        .await
        .expect("get");
    assert_eq!(missing, None);

    cache.put(&result).await.expect("put");
    assert_eq!(cache.len(), 1);

    let stored = cache
        .get(&result.repository, &result.commit)
        .await
        .expect("get");
    assert_eq!(stored, Some(result));
}

#[tokio::test]
async fn failing_cache_returns_configured_error() {
    let cache = MockAnalysisCache::failing(CacheError::Backend("disk full".into()));
    let result = fixtures::sample_analysis_result();

    assert_eq!(
        cache.put(&result).await,
        Err(CacheError::Backend("disk full".into()))
    );
    assert_eq!(
        cache.get(&result.repository, &result.commit).await,
        Err(CacheError::Backend("disk full".into()))
    );
}

#[test]
fn progress_reporter_records_events_in_order() {
    let reporter = RecordingProgressReporter::new();

    reporter.report(AnalysisProgress::Started { total_files: 2 });
    reporter.report(AnalysisProgress::FileAnalyzed {
        path: PathBuf::from("src/lib.rs"),
        analyzed: 1,
        total: 2,
    });
    reporter.report(AnalysisProgress::Finished);

    let events = reporter.events();
    assert_eq!(events.len(), 3);
    assert_eq!(events[0], AnalysisProgress::Started { total_files: 2 });
    assert_eq!(events[2], AnalysisProgress::Finished);
}
