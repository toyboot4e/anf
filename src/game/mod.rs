//! Game, the ANF framework

pub mod app;
pub mod draw;
mod framework;
pub mod utils;

use fna3d::Color;
use sdl2::event::Event;

use self::{app::*, draw::*};
pub use framework::AnfFramework;

use crate::{game::utils::FpsCounter, input::Keyboard};

/// Game with context/user data pattern
pub struct AnfGame<T: AnfGameState> {
    user: T,
    cx: Context,
}

impl<T: AnfGameState> AnfGame<T> {
    pub fn run(cfg: WindowConfig, user: impl FnOnce(&mut Context) -> T) -> AnfAppResult {
        AnfFramework::from_cfg(cfg).run(|win, cfg, dcx| {
            let mut cx = Context::new(win, cfg, dcx);
            let user = user(&mut cx);
            Self { user, cx: cx }
        })
    }
}

impl<T: AnfGameState> AnfAppLifecycle for AnfGame<T> {
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
    fn on_next_frame(&mut self) {
        self.cx.on_next_frame();
    }
}

pub trait AnfGameState {
    fn update(&mut self, cx: &mut Context);
    fn render(&mut self, cx: &mut Context);
}

/// Fixed set of objects
pub struct Context {
    pub win: WindowHandle,
    pub dcx: DrawContext,
    pub fps: FpsCounter,
    pub kbd: Keyboard,
    /// For debug window title
    pub win_title: String,
}

impl Context {
    pub fn new(win: WindowHandle, cfg: &WindowConfig, dcx: DrawContext) -> Self {
        Self {
            win,
            win_title: cfg.title.clone(),
            dcx,
            fps: FpsCounter::default(),
            kbd: Keyboard::new(),
        }
    }
}

impl AnfAppLifecycle for Context {
    fn event(&mut self, ev: &Event) {
        self.kbd.listen_sdl_event(ev);
    }

    fn update(&mut self, time_step: TimeStep) {
        // TODO: should it be called on render, too?
        if let Some(fps) = self.fps.update(time_step.elapsed()) {
            let title = format!("{} - {} FPS", self.win_title, fps);
            self.win.set_title(&title).unwrap();
        }
    }

    fn render(&mut self, time_step: TimeStep) {
        self.dcx.time_step = time_step;
        crate::gfx::clear_frame(&mut self.dcx, Color::cornflower_blue());
    }

    fn on_next_frame(&mut self) {
        let win = self.dcx.params.deviceWindowHandle;
        self.dcx.as_mut().swap_buffers(None, None, win);

        self.kbd.on_next_frame();
    }
}
