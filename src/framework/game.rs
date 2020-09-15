//! Runs user state

use crate::{
    framework::time::{GameClock, TimeStep},
    gfx::api::DrawContext,
    vfs,
};
use sdl2::{event::Event, sys::SDL_Window, EventPump};

/// User data driven by the ANF game loop
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
    fn update(&mut self, ts: TimeStep) {}
    #[allow(unused_variables)]
    fn render(&mut self, ts: TimeStep, dcx: &mut DrawContext) {}
    fn on_next_frame(&mut self) {}
}

/// Drives user data ([`AnfLifecycle`])
pub struct AnfLifecycleLoop {
    raw_window: *mut SDL_Window,
    pub clock: GameClock,
    pub dcx: DrawContext,
}

/// Device initialization
/// ---
impl AnfLifecycleLoop {
    pub fn new(raw_window: *mut SDL_Window, device: fna3d::Device) -> Self {
        Self {
            raw_window,
            clock: GameClock::new(),
            dcx: DrawContext::new(device, vfs::default_shader()),
        }
    }
}

impl AsMut<fna3d::Device> for AnfLifecycleLoop {
    fn as_mut(&mut self) -> &mut fna3d::Device {
        self.dcx.as_mut()
    }
}

/// Visitor
impl AnfLifecycleLoop {
    /// Returns if we continue next frame
    pub fn tick_one_frame(
        &mut self,
        state: &mut impl AnfLifecycle,
        events: &mut EventPump,
    ) -> bool {
        if !self.pump_events(state, events) {
            return false;
        }
        for ts in self.clock.tick() {
            state.update(ts);
        }
        self.render(state, self.clock.timestep());
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

    fn render(&mut self, state: &mut impl AnfLifecycle, ts: TimeStep) {
        state.render(ts, &mut self.dcx);
        self.dcx
            .as_mut()
            .swap_buffers(None, None, self.raw_window as *mut _);
    }
}
