use sdl2::event::Event;

use anf::game::{
    app::{WindowConfig, WindowHandle},
    draw::*,
    lifecycle::{AnfFramework, AnfGameResult, AnfLifecycle},
    time::TimeStep,
};

/// Entry point of an ANF game
///
/// The context/user_data pattern where context is also provided by user.
pub struct SampleGame<T: SampleGameState<U>, U: AnfLifecycle> {
    user: T,
    cx: U,
}

impl<T: SampleGameState<U>, U: AnfLifecycle> SampleGame<T, U> {
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

impl<T: SampleGameState<U>, U: AnfLifecycle> AnfLifecycle for SampleGame<T, U> {
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
pub trait SampleGameState<T: AnfLifecycle> {
    fn update(&mut self, cx: &mut T);
    fn render(&mut self, cx: &mut T);
}
