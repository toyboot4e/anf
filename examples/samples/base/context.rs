//! Modify the [`Context`] for your own game. Then it becomes a specific framework for you!

use std::time::Duration;

use imgui_fna3d::Fna3dImgui;

use anf::engine::prelude::*;
use anf::{engine::utils::FpsCounter, gfx::TextureData2d, input::Keyboard, vfs};

use fna3d::Color;
use sdl2::event::Event;

use crate::base::framework::SampleContextLifecycle;

/// Set of fundamental global objects
///
/// Because Rust doesn't have inheritance, it's recommended to copy & modify this struct to build
/// your own (static) context.
pub struct Context {
    pub win: WindowHandle,
    pub dcx: DrawContext,
    pub fps: FpsCounter,
    pub kbd: Keyboard,
    time_step: Duration,
    // debug
    win_title: String,
    pub imgui: Fna3dImgui,
}

impl Context {
    pub fn init(win: WindowHandle, cfg: &WindowConfig, mut dcx: DrawContext) -> Self {
        let size = win.screen_size();
        let size = [size.0 as f32, size.1 as f32];
        let font_size = 13.0;
        let dpi = 1.0; // TODO:

        let mut imgui =
            Fna3dImgui::quick_start(dcx.as_mut(), &win.win, size, font_size, dpi).unwrap();
        let textures = imgui.textures_mut();
        let ika = TextureData2d::from_path(dcx.as_mut(), vfs::path("ika-chan.png")).unwrap();
        let _id = textures.insert(imgui_fna3d::RcTexture2d::new(
            ika.raw(),
            dcx.as_mut().raw(),
            ika.w() as u32,
            ika.h() as u32,
        ));

        Self {
            win,
            dcx,
            fps: FpsCounter::default(),
            kbd: Keyboard::default(),
            time_step: Duration::new(0, 0),
            // debug
            win_title: cfg.title.clone(),
            imgui,
        }
    }

    pub fn time_step(&self) -> Duration {
        self.time_step
    }
}

impl SampleContextLifecycle for Context {
    fn event(&mut self, ev: &Event) -> AnfResult<()> {
        if self.imgui.handle_event(ev) {
            return Ok(());
        }
        self.kbd.event(ev)?;

        Ok(())
    }

    fn update(&mut self, time_step: TimeStep) -> AnfResult<()> {
        self.time_step = time_step.elapsed();

        // TODO: should it be called on render, too?
        if let Some(fps) = self.fps.update(time_step.elapsed()) {
            let title = format!("{} - {} FPS", self.win_title, fps);
            self.win.set_title(&title).unwrap();
        }

        Ok(())
    }

    fn render(&mut self, time_step: TimeStep) -> AnfResult<()> {
        self.time_step = time_step.elapsed();

        // FIXME: we should not be responsible for this actually
        self.dcx.set_time_step(time_step);

        Ok(())
    }

    fn on_end_frame(&mut self) -> AnfResult<()> {
        let win = self.dcx.raw_window();
        self.dcx.as_mut().swap_buffers(None, None, win as *mut _);

        self.kbd.on_end_frame()?;

        Ok(())
    }

    fn debug_render(&mut self) {}
}