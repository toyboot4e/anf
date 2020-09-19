//! Frames

use std::time::{Duration, Instant};

/// TODO: use this
pub enum TargetFps {
    Fixed(u32),
    Variable(u32),
}

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

/// Creates frames
#[derive(Debug, Clone)]
pub struct GameClock {
    // states
    time_step: TimeStep,
    accum: Duration,
    total: Duration,
    last_time: Instant,
    lag: u32,
    is_slow: bool,
    // configuration
    is_fixed_timestep: bool,
    updates_per_sec: f64,
}

impl GameClock {
    pub fn new() -> Self {
        Self {
            time_step: TimeStep::new(),
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
    pub fn tick(&mut self) -> GameClockTick {
        let elapsed = {
            let mut elapsed = self.wait_for_next_frame(self.accum);
            // Do not allow any update to take longer than our maximum.
            if elapsed > Self::max_elapsed() {
                elapsed = Self::max_elapsed();
            }
            elapsed
        };
        self.accum = elapsed;

        GameClockTick::new(self)
    }

    pub fn timestep(&self) -> TimeStep {
        self.time_step.clone()
    }

    fn wait_for_next_frame(&mut self, mut elapsed: Duration) -> Duration {
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
            }

            // sleep (inaccurate but enough for making frames)
            let remaining = target_elapsed - elapsed;
            std::thread::sleep(remaining);
        }
    }
}

/// Iterator of one frame
pub struct GameClockTick<'a> {
    clock: &'a mut GameClock,
    n_updates: u32,
}

impl<'a> GameClockTick<'a> {
    fn new(clock: &'a mut GameClock) -> Self {
        clock.time_step.elapsed = clock.target_elapsed();
        GameClockTick {
            clock,
            n_updates: 0,
        }
    }
}

impl<'a> Iterator for GameClockTick<'a> {
    type Item = TimeStep;

    fn next(&mut self) -> Option<Self::Item> {
        if self.clock.is_fixed_timestep {
            self.next_fixed()
        } else {
            self.next_variable()
        }
    }
}

/// Internals
impl<'a> GameClockTick<'a> {
    fn next_fixed(&mut self) -> Option<TimeStep> {
        let target_elapsed = self.clock.target_elapsed();

        // Perform as many full fixed length time steps as we can
        if self.clock.accum >= target_elapsed {
            self.clock.total += target_elapsed;
            self.clock.accum -= target_elapsed;
            self.n_updates += 1;
            return Some(self.clock.time_step.clone());
        }

        // Every update after the first accumulates lag
        if self.n_updates > 0 {
            self.clock.lag += self.n_updates - 1;
        }

        // If we think we are running slowly, wait
        // until the lag clears before resetting it
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

        self.clock.time_step.elapsed = target_elapsed * self.n_updates;

        None
    }

    fn next_variable(&mut self) -> Option<TimeStep> {
        if self.n_updates > 0 {
            return None;
        }

        // Perform a single variable length update.
        // if forceElapsedTimeToZero {
        //     // When ResetElapsedTime is called, Elapsed is forced to zero and Total is ignored entirely
        //     time.elapsed = Duration::new(0, 0.0);
        //     forceElapsedTimeToZero = false;
        // } else {
        self.clock.time_step.elapsed = self.clock.accum;
        self.clock.total += self.clock.accum;
        // }

        self.clock.time_step.elapsed = Duration::new(0, 0);
        // AssertNotDisposed();
        self.n_updates = 1;

        Some(self.clock.time_step.clone())
    }
}