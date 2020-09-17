use sdl2::event::Event;

use crate::{app::prelude::*, gfx::geom::Rect2f};

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
    fn render(&mut self, dcx: &mut DrawContext) {}
    fn on_next_frame(&mut self) {}
}

/// Dependencies to update game data
#[derive(Debug, Clone)]
pub struct UpdateContext {
    pub(crate) time_step: TimeStep,
    pub(crate) screen: Rect2f,
}

impl UpdateContext {
    pub fn time_step(&self) -> &TimeStep {
        &self.time_step
    }

    pub fn dt_secs_f32(&self) -> f32 {
        self.time_step.dt_secs_f32()
    }

    pub fn screen(&self) -> &Rect2f {
        &self.screen
    }
}
