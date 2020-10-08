//! Framework utilities provided to user

use ::std::time::Duration;

/// Frame rate counter
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FpsCounter {
    fps: usize,
    /// For updating view when necessary
    elapsed: Duration,
}

impl FpsCounter {
    /// Returns Some(fps) every second
    pub fn update(&mut self, elapsed: Duration) -> Option<usize> {
        self.fps += 1;
        self.elapsed += elapsed;
        if self.elapsed >= Duration::from_secs(1) {
            let fps = self.fps;
            self.fps = 0;
            self.elapsed = Duration::new(0, 0);
            Some(fps - 1) // FIXME: over 1 sec so
        } else {
            None
        }
    }
}
