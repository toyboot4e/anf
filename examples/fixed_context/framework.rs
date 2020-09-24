use sdl2::event::Event;

use anf::game::{
    app::{WindowConfig, WindowHandle},
    draw::*,
    lifecycle::{AnfFramework, AnfLifecycle, AnfResult},
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
    ) -> AnfResult<()> {
        AnfFramework::from_cfg(cfg).run(|win, cfg, dcx| {
            let mut cx = cx(win, cfg, dcx);
            let user = state(&mut cx);
            Self { user, cx }
        })
    }
}

impl<T: SampleGameState<U>, U: AnfLifecycle> AnfLifecycle for SampleGame<T, U> {
    fn event(&mut self, ev: &Event) -> AnfResult<()> {
        self.cx.event(ev)?;
        Ok(())
    }

    fn update(&mut self, time_step: TimeStep) -> AnfResult<()> {
        self.cx.update(time_step)?;
        self.user.update(&mut self.cx)?;
        Ok(())
    }

    fn render(&mut self, time_step: TimeStep) -> AnfResult<()> {
        self.cx.render(time_step)?;
        self.user.render(&mut self.cx)?;
        Ok(())
    }

    fn on_end_frame(&mut self) -> AnfResult<()> {
        self.cx.on_end_frame()?;
        Ok(())
    }
}

/// Where we manage user game data
pub trait SampleGameState<T: AnfLifecycle> {
    fn update(&mut self, cx: &mut T) -> AnfResult<()>;
    fn render(&mut self, cx: &mut T) -> AnfResult<()>;
}
