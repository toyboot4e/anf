use std::time::{Duration, Instant};

/// Delta time
#[derive(Debug, Clone)]
pub struct TimeStep {
    elapsed: Duration,
    /// Total duration passed in game time
    ///
    /// Because our game can stop, we have to accumulate the elapsed time in this game
    total: Duration,
}

impl TimeStep {
    pub fn new() -> Self {
        Self {
            elapsed: Duration::new(0, 0),
            total: Duration::new(0, 0),
        }
    }

    pub fn dt_secs_f32(&self) -> f32 {
        self.elapsed.as_secs_f32()
    }
}

/// Genetares [`TimeStep`]s
#[derive(Debug, Clone)]
pub struct GameClock {
    time: TimeStep,
    last_time: Instant,
    is_fixed_timestep: bool,
    updates_per_sec: f64,
    // states
    lag: u32,
    is_slow: bool,
}

impl GameClock {
    pub fn new() -> Self {
        Self {
            time: TimeStep::new(),
            last_time: Instant::now(),
            is_fixed_timestep: true,
            updates_per_sec: 60.0,
            lag: 0,
            is_slow: false,
        }
    }

    /// TODO: is this accurate?
    fn target_elapsed(&self) -> Duration {
        Duration::from_secs_f64(1.0 / self.updates_per_sec)
    }

    const fn max_elapsed() -> Duration {
        Duration::from_millis(500)
    }

    pub fn tick(&mut self) -> TimeStepIter {
        let elapsed = {
            let mut elapsed = self.get_elapsed();
            // Do not allow any update to take longer than our maximum.
            if elapsed > Self::max_elapsed() {
                elapsed = Self::max_elapsed();
            }
            elapsed
        };

        TimeStepIter::new(self, elapsed)
    }

    pub fn timestep(&self) -> TimeStep {
        self.time.clone()
    }

    fn get_elapsed(&mut self) -> Duration {
        loop {
            // Advance the accumulated elapsed time.
            let current = Instant::now();
            let elapsed = current.duration_since(self.last_time);
            self.last_time = current;

            if !self.is_fixed_timestep {
                break elapsed;
            }

            let target_elapsed = self.target_elapsed();
            if elapsed > target_elapsed {
                break elapsed;
            }

            let remaining = target_elapsed - elapsed;
            // sleep is inaccurate but enough for making frames
            std::thread::sleep(remaining);

            break elapsed;
        }
    }
}

pub struct TimeStepIter<'a> {
    clock: &'a mut GameClock,
    elapsed: Duration,
    n_updates: u32,
}

impl<'a> TimeStepIter<'a> {
    fn new(clock: &'a mut GameClock, elapsed: Duration) -> Self {
        clock.time.elapsed = clock.target_elapsed();
        Self {
            clock,
            elapsed,
            n_updates: 0,
        }
    }

    fn next_fixed(&mut self) -> Option<TimeStep> {
        // Perform as many full fixed length time steps as we can.
        let target_elapsed = self.clock.target_elapsed();
        let clock = &mut self.clock;

        if self.elapsed >= target_elapsed {
            clock.time.total += target_elapsed;
            self.elapsed -= target_elapsed;
            self.n_updates += 1;
            return Some(clock.time.clone());
        }

        // Every update after the first accumulates lag
        if self.n_updates > 0 {
            clock.lag += self.n_updates - 1;
        }

        // If we think we are running slowly, wait
        // until the lag clears before resetting it
        match (clock.is_slow, clock.lag) {
            (true, 0) => clock.is_slow = false,
            (false, lag) if lag >= 5 => clock.is_slow = true,
            _ => {}
        };

        // Every time we just do one update and one draw,
        // then we are not running slowly, so decrease the lag.
        if self.n_updates == 1 && clock.lag > 0 {
            clock.lag -= 1;
        }

        clock.time.elapsed = target_elapsed * self.n_updates;

        None
    }

    fn next_variable(&mut self) -> Option<TimeStep> {
        if self.n_updates > 0 {
            return None;
        }
        let clock = &mut self.clock;

        // Perform a single variable length update.
        // if forceElapsedTimeToZero {
        //     // When ResetElapsedTime is called, Elapsed is forced to zero and Total is ignored entirely
        //     time.elapsed = Duration::new(0, 0.0);
        //     forceElapsedTimeToZero = false;
        // } else {
        clock.time.elapsed = self.elapsed;
        clock.time.total += self.elapsed;
        // }

        clock.time.elapsed = Duration::new(0, 0);
        // AssertNotDisposed();
        self.n_updates = 1;

        Some(clock.time.clone())
    }
}

impl<'a> Iterator for TimeStepIter<'a> {
    type Item = TimeStep;

    fn next(&mut self) -> Option<Self::Item> {
        if self.clock.is_fixed_timestep {
            self.next_fixed()
        } else {
            self.next_variable()
        }
    }
}
