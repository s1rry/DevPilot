//! # devpilot-scan
//!
//! Project scanner for DevPilot. Implements the [`ProjectScanner`] port from
//! `devpilot-core` by reading a repository's manifest files and recognizing
//! frameworks and dependencies.
//!
//! Detection is split from I/O: [`detectors`] holds pure functions over
//! manifest text (easy to unit-test), while [`FsProjectScanner`] reads the
//! working copy and feeds those functions.
//!
//! ## Rules
//!
//! - Depends only on `devpilot-core`; never on other adapter crates.
//! - A missing or malformed manifest is skipped, not an error.

use std::path::{Path, PathBuf};

use async_trait::async_trait;
use devpilot_core::entities::{Detection, Repository};
use devpilot_core::errors::ScanError;
use devpilot_core::ports::ProjectScanner;

pub mod detectors;

/// Manifest files recognized at the repository root, paired with the detector
/// that parses each.
type Detector = fn(&str) -> Detection;
const MANIFESTS: &[(&str, Detector)] = &[
    ("package.json", detectors::detect_npm),
    ("Cargo.toml", detectors::detect_cargo),
    ("requirements.txt", detectors::detect_requirements),
    ("pyproject.toml", detectors::detect_pyproject),
    ("go.mod", detectors::detect_gomod),
];

/// A [`ProjectScanner`] that reads manifests from the working copy on disk.
#[derive(Default)]
pub struct FsProjectScanner;

impl FsProjectScanner {
    /// Creates a scanner.
    pub fn new() -> Self {
        Self
    }
}

/// Reads and detects from every recognized manifest at `root`, merging the
/// results. Missing files are skipped; a read error on a present file is
/// reported.
fn scan_root(root: &Path) -> Result<Detection, ScanError> {
    let mut detection = Detection::default();
    for (file_name, detect) in MANIFESTS {
        let path = root.join(file_name);
        match std::fs::read_to_string(&path) {
            Ok(content) => {
                let found = detect(&content);
                detection.frameworks.extend(found.frameworks);
                detection.dependencies.extend(found.dependencies);
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(ScanError::Backend(format!(
                    "reading {}: {error}",
                    path.display()
                )));
            }
        }
    }
    Ok(detection)
}

#[async_trait]
impl ProjectScanner for FsProjectScanner {
    async fn detect(&self, repository: &Repository) -> Result<Detection, ScanError> {
        let root: PathBuf = repository.local_path.clone();
        tokio::task::spawn_blocking(move || scan_root(&root))
            .await
            .map_err(|join| ScanError::Backend(format!("scan task failed: {join}")))?
    }
}
