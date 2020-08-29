//! The game application

use crate::gfx::{self, batcher::Batcher, DrawContext, Pipeline};
use fna3d::Device;
use sdl2::{
    render::WindowCanvas,
    {event::Event, keyboard::Keycode},
};
use std::time::Duration;

/// Initial window settings
pub struct WindowConfig {
    pub title: String,
    pub w: u32,
    pub h: u32,
}

/// Window handle created from `WindowConfig`
pub struct WindowHandle {
    pub sdl: sdl2::Sdl,
    pub raw_window: *mut sdl2::sys::SDL_Window,
    pub params: fna3d::PresentationParameters,
    pub canvas: WindowCanvas,
}

enum UpdateResult {
    Continue,
    Quit,
}

/// The core of the application
pub struct App<T: AppState> {
    dcx: DrawContext,
    clear_color: fna3d::Color,
    state: T,
    win: WindowHandle,
}

/// Data injected to `AppCore`
pub trait AppState {
    fn render(&mut self, dcx: &mut DrawContext);
    fn update(&mut self);
}

/// The final notification from the application returned by `run_loop`
pub type AppResult = std::result::Result<(), Box<dyn std::error::Error>>;

impl WindowConfig {
    pub fn default() -> Self {
        Self {
            title: "† ANF game †".to_string(),
            w: 1280,
            h: 720,
        }
    }

    pub fn create(&self) -> (WindowHandle, fna3d::Device) {
        log::info!("FNA version {}", fna3d::linked_version());
        let flags = fna3d::prepare_window_attributes();

        let sdl = sdl2::init().unwrap();
        let canvas = self.canvas(&sdl, flags.0);
        let win = canvas.window().raw();
        let (params, device) = self.device(win as *mut _);

        (
            WindowHandle {
                sdl,
                raw_window: win,
                params,
                canvas,
            },
            device,
        )
    }
}

impl WindowConfig {
    fn canvas(&self, sdl: &sdl2::Sdl, flags: u32) -> WindowCanvas {
        let video = sdl.video().unwrap();
        let win = self.window(video, flags);
        win.into_canvas().build().unwrap()
    }

    fn window(&self, video: sdl2::VideoSubsystem, flags: u32) -> sdl2::video::Window {
        video
            .window(&self.title, self.w, self.h)
            .set_window_flags(flags)
            .position_centered()
            .build()
            .unwrap()
    }

    pub fn device(
        &self,
        win: *mut std::ffi::c_void,
    ) -> (fna3d::PresentationParameters, fna3d::Device) {
        let params = {
            let mut params = fna3d::utils::params_from_window_handle(win);
            params.backBufferWidth = self.w as i32;
            params.backBufferHeight = self.h as i32;
            params
        };
        let device = fna3d::Device::from_params(params, true);
        (params, device)
    }
}

impl<T: AppState> App<T> {
    pub fn new(state: T, win: WindowHandle, mut device: Device) -> Self {
        gfx::init(&mut device, &win.params);

        let pipe = Pipeline::from_device(&mut device);
        let batcher = Batcher::from_device(&mut device);

        App {
            dcx: DrawContext {
                device,
                batcher,
                pipe,
            },
            clear_color: fna3d::Color::cornflower_blue(),
            state,
            win,
        }
    }

    pub fn run(&mut self) -> AppResult {
        let mut events = self.win.sdl.event_pump().unwrap();
        log::trace!("Start ANF game loop");

        'main_loop: loop {
            // pump events
            for ev in events.poll_iter() {
                match self.handle_event(&ev) {
                    UpdateResult::Quit => break 'main_loop,
                    UpdateResult::Continue => {}
                }
            }

            self.update();
            self.render();

            // FIXME: timestep handling & `GameTime`
            let fps = 60;
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / fps));
        }

        Ok(())
    }
}

impl<T: AppState> App<T> {
    /// Does nothing for now
    fn update(&mut self) {
        self.state.update();
    }

    /// Runs the rendering pipeline
    fn render(&mut self) {
        gfx::clear(&mut self.dcx.device, self.clear_color);
        self.state.render(&mut self.dcx);
        gfx::end_frame(&mut self.dcx.device, self.win.raw_window as *mut _);
    }

    /// Just quits on `Escape` key down
    fn handle_event(&mut self, ev: &sdl2::event::Event) -> UpdateResult {
        match ev {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => UpdateResult::Quit,
            _ => UpdateResult::Continue,
        }
    }
}
