//! Creates [`TimeStep`]s

use std::time::Duration;

/// Delta time
#[derive(Debug, Clone, Default)]
pub struct TimeStep {
    // public in crate so that it can be set by framework
    pub(crate) elapsed: Duration,
}

impl TimeStep {
    pub fn new() -> Self {
        Self {
            elapsed: Duration::new(0, 0),
        }
    }

    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }

    pub fn dt_secs_f32(&self) -> f32 {
        self.elapsed.as_secs_f32()
    }
}
