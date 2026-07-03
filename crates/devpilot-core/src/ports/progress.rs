use crate::entities::AnalysisProgress;

/// Receives progress events from long-running operations.
///
/// The desktop shell forwards events to the UI; tests record them.
/// Implementations must be cheap and non-blocking: `report` is called on
/// the hot path of the analysis pipeline.
pub trait ProgressReporter: Send + Sync {
    /// Handles one progress event.
    fn report(&self, progress: AnalysisProgress);
}
