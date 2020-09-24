//! Framework bulilt on top FNA for sample games
//!
//! Modify the [`Context`] for your own game. Then it becomes a specific framework for you!

use imgui::im_str;
use imgui_fna3d::Fna3dImgui;

use anf::{
    game::{app::*, draw::*, lifecycle::AnfLifecycle, time::TimeStep, utils::FpsCounter},
    gfx::TextureData2d,
    input::Keyboard,
    vfs,
};

use fna3d::Color;
use sdl2::event::Event;

pub trait DebugLifecycle: AnfLifecycle {
    fn debug_render(&mut self, cx: &mut Context);
}

/// Set of fundamental global objects
///
/// Because Rust doesn't have inheritance, it's recommended to copy & modify this struct to build
/// your own (static) context.
pub struct Context {
    pub win: WindowHandle,
    pub dcx: DrawContext,
    pub fps: FpsCounter,
    pub kbd: Keyboard,
    // debug
    win_title: String,
    imgui: Fna3dImgui,
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
            kbd: Keyboard::new(),
            // debug
            win_title: cfg.title.clone(),
            imgui,
        }
    }
}

impl AnfLifecycle for Context {
    fn event(&mut self, ev: &Event) {
        if self.imgui.handle_event(ev) {
            return;
        }
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
        // FIXME: we want to do this here
        // state.render()
        // self.debug_render()
    }

    fn on_end_frame(&mut self) {
        self.debug_render();

        let win = self.dcx.raw_window();
        self.dcx.as_mut().swap_buffers(None, None, win as *mut _);

        self.kbd.on_end_frame();
    }
}

impl Context {
    fn debug_render(&mut self) {
        let (ui, fin) = {
            let size = self.win.screen_size();
            let size = [size.0 as f32, size.1 as f32];
            let scale = [1.0, 1.0];
            let dt = 0.016; // FIXME:
            self.imgui.frame(&self.win, size, scale, dt)
        };

        ui.show_demo_window(&mut true);

        fin.render(ui, self.win.as_ref(), self.dcx.as_mut())
            .unwrap();
    }
}
