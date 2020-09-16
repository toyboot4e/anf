//! Runs user state

use sdl2::{event::Event, sys::SDL_Window, EventPump};

use crate::{
    framework::time::{GameClock, TimeStep},
    gfx::api::DrawContext,
};

/// User data driven by [`AnfGameLoop`]
///
/// The internal game loop is like this:
///
/// ```c
/// // game: impl AnfLifecycle
/// loop {
///     for ev in poll_sdl_event() {
///         game.listen(ev);
///     }
///     for timestep in clock.tick() {
///         game.update(timestep);
///     }
///     game.render(clock.timestep(), draw_context);
///     game.on_frame_end();
/// }
/// ```
pub trait AnfLifecycle {
    #[allow(unused_variables)]
    fn event(&mut self, ev: &Event) {}
    #[allow(unused_variables)]
    fn update(&mut self, ucx: &UpdateContext) {}
    #[allow(unused_variables)]
    fn draw(&mut self, dcx: &mut DrawContext) {}
    fn on_next_frame(&mut self) {}
}

/// Drives user data ([`AnfLifecycle`])
#[derive(Debug)]
pub struct AnfGameLoop {
    raw_window: *mut SDL_Window,
    pub clock: GameClock,
    pub dcx: DrawContext,
}

/// Dependencies to update game data
#[derive(Debug, Clone)]
pub struct UpdateContext {
    ts: TimeStep,
    screen_size: [u32; 2],
}

impl UpdateContext {
    pub fn ts(&self) -> &TimeStep {
        &self.ts
    }

    pub fn dt_secs_f32(&self) -> f32 {
        self.ts.dt_secs_f32()
    }

    pub fn screen_size(&self) -> [u32; 2] {
        self.screen_size
    }

    pub fn screen_size_f32(&self) -> [f32; 2] {
        [self.screen_size[0] as f32, self.screen_size[1] as f32]
    }
}

/// Device initialization
/// ---
impl AnfGameLoop {
    pub fn new(raw_window: *mut SDL_Window, dcx: DrawContext) -> Self {
        Self {
            raw_window,
            clock: GameClock::new(),
            dcx,
        }
    }
}

impl AsMut<fna3d::Device> for AnfGameLoop {
    fn as_mut(&mut self) -> &mut fna3d::Device {
        self.dcx.as_mut()
    }
}

/// Visitor
impl AnfGameLoop {
    /// The game loop. Returns if `true` not finished
    pub fn tick_one_frame(
        &mut self,
        state: &mut impl AnfLifecycle,
        events: &mut EventPump,
    ) -> bool {
        if !self.pump_events(state, events) {
            return false;
        }
        self.update(state);
        self.render(state);
        state.on_next_frame();

        true
    }

    fn pump_events(&mut self, state: &mut impl AnfLifecycle, events: &mut EventPump) -> bool {
        for ev in events.poll_iter() {
            match ev {
                Event::Quit { .. } => return false,
                ev => {
                    state.event(&ev);
                }
            }
        }
        true
    }

    fn update(&mut self, state: &mut impl AnfLifecycle) {
        let screen_size = self.dcx.screen_size();
        for ts in self.clock.tick() {
            let ucx = UpdateContext { ts, screen_size };
            state.update(&ucx);
        }
    }

    fn render(&mut self, state: &mut impl AnfLifecycle) {
        self.dcx.time_step = self.clock.timestep();
        state.draw(&mut self.dcx);
        self.dcx
            .as_mut()
            .swap_buffers(None, None, self.raw_window as *mut _);
    }
}
