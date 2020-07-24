//! Window configuration & creation

use sdl2::{keyboard::Keycode, render::WindowCanvas};

/// Initial window settings
pub struct Config {
    pub title: String,
    pub w: u32,
    pub h: u32,
}

/// Creates a window from `WindowConfig` and returns a handle to it
///
/// You can get raw pointer to SDL window from the returned `WindowCanvas`
pub fn create(cfg: &Config) -> (sdl2::Sdl, WindowCanvas) {
    log::info!("FNA version {}", fna3d::linked_version());
    // TODO: should I use the flags
    let _flags = fna3d::prepare_window_attributes();

    let scx = sdl2::init().unwrap();
    let canvas = self::create_canvas(&scx, &cfg);
    (scx, canvas)
}

fn create_canvas(scx: &sdl2::Sdl, cfg: &Config) -> WindowCanvas {
    let video = scx.video().unwrap();
    let win = cfg.create_window(video);
    win.into_canvas().build().unwrap()
}

impl Config {
    pub fn default() -> Self {
        Self {
            title: "† ANF †".to_string(),
            w: 1280,
            h: 720,
        }
    }

    fn create_window(&self, video: sdl2::VideoSubsystem) -> sdl2::video::Window {
        video
            .window(&self.title, self.w, self.h)
            .position_centered()
            .build()
            .unwrap()
    }
}
