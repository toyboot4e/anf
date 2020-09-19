//! The framework
//!
//! Build your own framework on top of it!

pub mod app;
pub mod draw;
pub mod time;
pub mod utils;

// TODO: remove framework module
mod framework;
pub use framework::{AnfGameResult, AnfLifecycle};

use sdl2::event::Event;

use self::{app::*, draw::*, framework::*, time::*};

/// Entry point of an ANF game
///
/// The context/user_data pattern where context is also provided by user.
pub struct AnfGame<T: AnfGameState<U>, U: AnfLifecycle> {
    user: T,
    cx: U,
}

impl<T: AnfGameState<U>, U: AnfLifecycle> AnfGame<T, U> {
    pub fn run(
        cfg: WindowConfig,
        cx: impl FnOnce(WindowHandle, &WindowConfig, DrawContext) -> U,
        state: impl FnOnce(&mut U) -> T,
    ) -> AnfGameResult {
        AnfFramework::from_cfg(cfg).run(|win, cfg, dcx| {
            let mut cx = cx(win, cfg, dcx);
            let user = state(&mut cx);
            Self { user, cx: cx }
        })
    }
}

impl<T: AnfGameState<U>, U: AnfLifecycle> AnfLifecycle for AnfGame<T, U> {
    fn event(&mut self, ev: &Event) {
        self.cx.event(ev);
    }
    fn update(&mut self, time_step: TimeStep) {
        self.cx.update(time_step);
        self.user.update(&mut self.cx);
    }
    fn render(&mut self, time_step: TimeStep) {
        self.cx.render(time_step);
        self.user.render(&mut self.cx);
    }
    fn on_end_frame(&mut self) {
        self.cx.on_end_frame();
    }
}

/// Where we manage user game data
pub trait AnfGameState<T: AnfLifecycle> {
    fn update(&mut self, cx: &mut T);
    fn render(&mut self, cx: &mut T);
}
