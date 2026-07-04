use async_trait::async_trait;
use devpilot_core::entities::{Detection, Repository};
use devpilot_core::errors::ScanError;
use devpilot_core::ports::ProjectScanner;

/// Configurable [`ProjectScanner`] for tests.
///
/// Returns an empty detection by default; configure a result with
/// [`with_detection`](MockProjectScanner::with_detection) or a failure with
/// [`failing`](MockProjectScanner::failing).
pub struct MockProjectScanner {
    result: Result<Detection, ScanError>,
}

impl MockProjectScanner {
    /// Creates a scanner returning an empty detection.
    pub fn new() -> Self {
        Self {
            result: Ok(Detection::default()),
        }
    }

    /// Configures the detection to return.
    pub fn with_detection(mut self, detection: Detection) -> Self {
        self.result = Ok(detection);
        self
    }

    /// Configures a failure.
    pub fn failing(error: ScanError) -> Self {
        Self { result: Err(error) }
    }
}

impl Default for MockProjectScanner {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ProjectScanner for MockProjectScanner {
    async fn detect(&self, _repository: &Repository) -> Result<Detection, ScanError> {
        self.result.clone()
    }
}
