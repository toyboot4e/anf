use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use anf::gfx::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LoopMode {
    /// [A][B][C][A][B][C][A][B][C]...
    Loop,
    /// [A][B][C][C][C]...
    ClampForever,
    /// [A][B][C] then pause and set time to 0
    Once,
    /// [A][B][C][B][A][B][C][B]...
    PingPong,
    /// [A][B][C][B][A] then pause and set time to 0
    PingPongOnce,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LoopState {
    Running,
    Paused,
    Stopped,
}

#[derive(Debug, Clone)]
pub struct SpriteAnimPattern {
    frames: Vec<SpriteData>,
    fps: f32,
    loop_mode: LoopMode,
}

impl SpriteAnimPattern {
    pub fn new(frames: Vec<SpriteData>, fps: f32, loop_mode: LoopMode) -> Self {
        Self {
            frames,
            fps,
            loop_mode,
        }
    }

    /// Returns (duration, state) after completion handling
    pub fn on_tick(&mut self, past: Duration) -> (Duration, LoopState) {
        let loop_duration = self.loop_duration();
        if past < loop_duration {
            return (past, LoopState::Running);
        }

        // on end
        match self.loop_mode {
            // finish
            LoopMode::Once | LoopMode::ClampForever => (loop_duration, LoopState::Stopped),
            // loop
            LoopMode::Loop | LoopMode::PingPong | LoopMode::PingPongOnce => {
                (past - loop_duration, LoopState::Running)
            }
        }
    }

    pub fn frame(&self, past: Duration) -> &SpriteData {
        let ix = self.frame_ix(past);
        &self.frames[self.frame_ix(past)]
    }

    fn frame_ix(&self, past: Duration) -> usize {
        let ms_per_frame = 1000.0 * 1.0 / self.fps;
        let ms_past = past.as_millis();
        let frame = (ms_past / ms_per_frame as u128) as usize;

        let len = self.frames.len();
        match self.loop_mode {
            // ping pong loop
            //
            // [A][B][C][D][C][B][A]..
            //  0  1  2  3  4  5  6 :: frame
            //  0  1  2  3  2  1  0 :: 2 * (len - 1) - frame
            //  where the last frame should be omitted so that it's not duplicated
            LoopMode::PingPong | LoopMode::PingPongOnce if frame >= len => 2 * (len - 1) - frame,
            // not ping pong
            _ => frame,
        }
    }

    fn loop_duration(&self) -> Duration {
        let sec = 1.0 / self.fps * self.n_loop_frames() as f32;
        let ms = (1000.0 * sec) as u64;
        Duration::from_millis(ms)
    }

    fn n_loop_frames(&self) -> usize {
        match self.loop_mode {
            // ping pong (omitting the duplicating last frame)
            LoopMode::PingPong | LoopMode::PingPongOnce => self.frames.len() * 2 - 2,
            // not ping pong
            LoopMode::Loop | LoopMode::Once | LoopMode::ClampForever => self.frames.len(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpriteAnimState<T> {
    // pattern settings
    patterns: HashMap<T, SpriteAnimPattern>,
    // states
    cur_key: T,
    accum: Duration,
    state: LoopState,
}

impl<T> SpriteAnimState<T> {
    pub fn new(patterns: HashMap<T, SpriteAnimPattern>, initial_key: T) -> Self {
        Self {
            patterns,
            cur_key: initial_key,
            accum: Duration::new(0, 0),
            state: LoopState::Running,
        }
    }
}

impl<T: Eq + std::hash::Hash> SpriteAnimState<T> {
    pub fn current_frame(&mut self) -> &SpriteData {
        let pattern = self.patterns.get_mut(&self.cur_key).unwrap();
        pattern.frame(self.accum)
    }

    pub fn current_pattern(&mut self) -> Option<&SpriteAnimPattern> {
        self.patterns.get(&self.cur_key)
    }

    pub fn tick(&mut self, past: Duration) {
        if matches!(self.state, LoopState::Stopped | LoopState::Stopped) {
            return;
        }

        let pattern = self.patterns.get_mut(&self.cur_key).unwrap();

        self.accum += past;
        let (next_duration, next_state) = pattern.on_tick(self.accum);
        self.accum = next_duration;
        self.state = next_state;
    }

    pub fn set_pattern(&mut self, key: T, reset_accum: bool) {
        if reset_accum {
            self.accum = Duration::new(0, 0);
        }

        self.cur_key = key;
    }
}
