use std::sync::Mutex;

use devpilot_core::entities::AnalysisProgress;
use devpilot_core::ports::ProgressReporter;

/// [`ProgressReporter`] that records every event for later assertions.
#[derive(Default)]
pub struct RecordingProgressReporter {
    events: Mutex<Vec<AnalysisProgress>>,
}

impl RecordingProgressReporter {
    /// Creates a reporter with an empty event log.
    pub fn new() -> Self {
        Self::default()
    }

    /// Events received so far, in order.
    pub fn events(&self) -> Vec<AnalysisProgress> {
        self.events.lock().expect("mock mutex poisoned").clone()
    }
}

impl ProgressReporter for RecordingProgressReporter {
    fn report(&self, progress: AnalysisProgress) {
        self.events
            .lock()
            .expect("mock mutex poisoned")
            .push(progress);
    }
}
