/*! Creates lifecycle

# Example

```no_run
use {
    anf::engine::time::GameClock,
    sdl2::EventPump,
};

fn tick_one_frame(
    clock: &mut GameClock,
    events: &mut EventPump,
) {
   for ev in events.poll_iter() {
        // handle events
   }

    for dt in clock.tick() {
        // update your game
    }

    let time_step = clock.timestep_draw();
    // draw your game

    // end of the frame
}
```
!*/

use std::time::{Duration, Instant};

/// TODO: use this
pub enum TargetFps {
    Fixed(u32),
    Variable(u32),
}

/// Creates frames
#[derive(Debug, Clone)]
pub struct GameClock {
    /// Update/draw timestep duration
    time_step: Duration,
    /// Accumulated duration for update/render calls
    accum: Duration,
    /// Total duration passed since the clock is created
    total: Duration,
    /// Temporary value to accumulate time
    pub(crate) last_time: Instant,
    /// [Fixed timestep only]
    ///
    /// The value is incremented by `n_updates - 1` on every tick
    lag: u32,
    /// If the lag is too big, this is true (though the value is not provided to user for now)
    is_slow: bool,
    /// Configuration
    is_fixed_timestep: bool,
    updates_per_sec: f64,
}

impl GameClock {
    pub fn new() -> Self {
        Self {
            time_step: Duration::new(0, 0),
            accum: Duration::new(0, 0),
            total: Duration::new(0, 0),
            last_time: Instant::now(),
            lag: 0,
            is_slow: false,
            is_fixed_timestep: true,
            updates_per_sec: 60.0,
        }
    }

    /// TODO: is this accurate?
    fn target_elapsed(&self) -> Duration {
        Duration::from_secs_f64(1.0 / self.updates_per_sec)
    }

    const fn max_elapsed() -> Duration {
        Duration::from_millis(500)
    }

    /// Returns way to tick one frame
    pub fn tick(&mut self) -> GameClockOneFrameTick {
        self.accum = {
            // Do not allow any update to take longer than our maximum.
            let elapsed = self.wait_for_next_frame(self.accum);
            if elapsed > Self::max_elapsed() {
                Self::max_elapsed()
            } else {
                elapsed
            }
        };

        GameClockOneFrameTick::new(self)
    }

    pub fn timestep_draw(&self) -> Duration {
        self.time_step.clone()
    }

    fn wait_for_next_frame(&mut self, accum: Duration) -> Duration {
        let mut elapsed = accum;
        loop {
            // Advance the accumulated elapsed time.
            let now = Instant::now();
            elapsed += now.duration_since(self.last_time);
            self.last_time = now;

            if !self.is_fixed_timestep {
                return elapsed;
            }

            let target_elapsed = self.target_elapsed();
            if elapsed > target_elapsed {
                break elapsed;
            } else {
                // sleep (inaccurate but enough for making frames)
                let remaining = target_elapsed - elapsed;
                std::thread::sleep(remaining);
            }
        }
    }
}

/// Iterator of one frame
pub struct GameClockOneFrameTick<'a> {
    clock: &'a mut GameClock,
    n_updates: u32,
}

impl<'a> GameClockOneFrameTick<'a> {
    fn new(clock: &'a mut GameClock) -> Self {
        clock.time_step = clock.target_elapsed();
        GameClockOneFrameTick {
            clock,
            n_updates: 0,
        }
    }
}

impl<'a> Iterator for GameClockOneFrameTick<'a> {
    type Item = Duration;

    fn next(&mut self) -> Option<Self::Item> {
        if self.clock.is_fixed_timestep {
            self.next_fixed()
        } else {
            self.next_variable()
        }
    }
}

/// Internals
impl<'a> GameClockOneFrameTick<'a> {
    fn next_fixed(&mut self) -> Option<Duration> {
        let target_elapsed = self.clock.target_elapsed();

        // Perform as many full fixed length time steps as we can
        if self.clock.accum >= target_elapsed {
            self.clock.total += target_elapsed;
            self.clock.accum -= target_elapsed;
            self.n_updates += 1;

            if self.n_updates > 1 {
                // FIXME:
                log::trace!(
                    "lag: Update more than once a frame: {} | {:?}",
                    self.n_updates,
                    self.clock.accum
                );
            }

            // update
            return Some(self.clock.time_step.clone());
        }

        // Every update after the first accumulates lag
        if self.n_updates > 0 {
            self.clock.lag += self.n_updates - 1;
        }

        // Update slow/normal consideration
        match (self.clock.is_slow, self.clock.lag) {
            (true, 0) => self.clock.is_slow = false,
            (false, lag) if lag >= 5 => self.clock.is_slow = true,
            _ => {}
        };

        // Every time we just do one update and one draw,
        // then we are not running slowly, so decrease the lag.
        if self.n_updates == 1 && self.clock.lag > 0 {
            self.clock.lag -= 1;
        }

        // Set timestep for drawing
        self.clock.time_step = target_elapsed * self.n_updates;

        None
    }

    fn next_variable(&mut self) -> Option<Duration> {
        if self.n_updates > 0 {
            return None;
        }

        // Perform a single variable length update.
        // if forceElapsedTimeToZero {
        //     // When ResetElapsedTime is called, Elapsed is forced to zero and Total is ignored entirely
        //     time.elapsed = Duration::new(0, 0.0);
        //     forceElapsedTimeToZero = false;
        // } else {
        self.clock.time_step = self.clock.accum;
        self.clock.total += self.clock.accum;
        // }

        self.clock.time_step = Duration::new(0, 0);
        // AssertNotDisposed();
        self.n_updates = 1;

        Some(self.clock.time_step.clone())

        // On next call this method returns `None`
    }
}
