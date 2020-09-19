//! Framework bulilt on top FNA for sample games
//!
//! Modify the [`Context`] for your own game. Then it becomes a specific framework for you!

use imgui::*;
use imgui_fna3d::Fna3dImgui;

use anf::{
    game::{app::*, draw::*, time::TimeStep, utils::FpsCounter, AnfLifecycle},
    input::Keyboard,
};

use fna3d::Color;
use sdl2::event::Event;

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
        let dpi = 2.0; // TODO:
        let imgui = Fna3dImgui::quick_start(&mut dcx, &win, size, 13.0, dpi).unwrap();

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
        let mut io = self.imgui.icx.io_mut();
        io.display_size = [1280.0, 720.0];
        io.display_framebuffer_scale = [1.0, 1.0];
        io.delta_time = 0.016; // FIXME:

        self.imgui.prepare_frame(&self.win);
        let ui = self.imgui.icx.frame();

        ui.show_demo_window(&mut true);

        // Window::new(im_str!("Hello world"))
        //     .size([300.0, 600.0], Condition::FirstUseEver)
        //     .position([100.0, 100.0], Condition::FirstUseEver)
        //     .build(&ui, || {
        //         ui.text(im_str!("Hello world!"));
        //         ui.text(im_str!("こんにちは世界！"));
        //         ui.text(im_str!("This...is...imgui-rs!"));
        //         ui.separator();
        //         let mouse_pos = ui.io().mouse_pos;
        //         ui.text(im_str!(
        //             "Mouse Position: ({:.1},{:.1})",
        //             mouse_pos[0],
        //             mouse_pos[1],
        //         ));

        //         if ui.small_button(im_str!("small button")) {
        //             println!("Small button clicked");
        //         }
        //     });

        // if let Some(tk) = ui.begin_menu_bar() {
        //     if let Some(tk) = ui.begin_menu(im_str!("menu A"), true) {
        //         log::trace!("MENU");
        //         MenuItem::new(im_str!("menu item A")).build(&ui);

        //         tk.end(&ui);
        //     }
        //     // ui.text("BBBB.");
        //     // ui.text("text. これが世界！ help me..");
        //     // ui.text("text. これが世界！ help me..");
        //     // ui.text("text. これが世界！ help me..");

        //     tk.end(&ui);
        // }

        self.imgui
            .part
            .render(ui, self.win.as_ref(), self.dcx.as_mut())
            .unwrap();
    }
}
