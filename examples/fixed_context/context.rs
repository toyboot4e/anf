//! Framework bulilt on top FNA for sample games
//!
//! Modify the [`Context`] for your own game. Then it becomes a specific framework for you!

use anf::{
    game::{app::*, draw::*, time::TimeStep, utils::FpsCounter, AnfLifecycle},
    input::Keyboard,
};

use fna3d::Color;
use sdl2::event::Event;

/// Set of global objects
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

impl AnfLifecycle for Context {
    fn event(&mut self, ev: &Event) {
        self.kbd.event(ev);
    }

    fn update(&mut self, time_step: TimeStep) {
        // TODO: should it be called on render, too?
        if let Some(fps) = self.fps.update(time_step.elapsed()) {
            let title = format!("{} - {} FPS", self.win_title, fps);
            self.win.set_title(&title).unwrap();
        }
    }

    fn render(&mut self, time_step: TimeStep) {
        // FIXME: we should not be responsible for this actually
        self.dcx.set_time_step(time_step);
        anf::gfx::clear_frame(&mut self.dcx, Color::cornflower_blue());
    }

    fn on_end_frame(&mut self) {
        let win = self.dcx.raw_window();
        self.dcx.as_mut().swap_buffers(None, None, win as *mut _);

        self.kbd.on_end_frame();
    }
}
