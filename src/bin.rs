//! Sampler game loop

use anf::{
    batch::{self, Batcher}, //{device::GraphicsDevice, device_utils::DeviceStatus},
    gfx::texture::Texture2D,
};

use fna3d::Device;
use sdl2::{event::Event, keyboard::Keycode};
use std::{ffi::c_void, path::PathBuf};

// TODO: &self -> &mut T

fn setup() {
    env_logger::init();
    log::info!("FNA version {}", fna3d::linked_version());

    let _flags = fna3d::prepare_window_attributes();
    fna3d::hook_log_functions_default();
}

fn main() {
    self::setup();

    // Create a window using SDL2
    let cfg = anf::WindowConfig::default();
    let (mut scx, canvas) = anf::create(&cfg);

    // Set up FNA3D for rendering
    let win = canvas.window().raw() as *mut _; // FIXME: do not use Canvas
    let params = fna3d::utils::params_from_window_handle(win);
    let device = Device::from_params(params, true);

    // Run the game loop
    let mut state = MainState::new(device, win);
    match anf::run_loop(&mut state, &mut scx) {
        Ok(()) => {}
        Err(why) => println!("Error occured: {}", why),
    };
}

pub struct MainState {
    device: Device,
    batcher: Batcher,
    texture: Texture2D,
}

impl MainState {
    pub fn new(mut device: Device, win: *mut c_void) -> Self {
        let texture = {
            let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/assets");
            let path = root.join("a.png");
            Texture2D::from_path(&mut device, &path).expect("failed to load texture")
        };

        let batcher = Batcher::new(&mut device, win);
        Self {
            device,
            batcher,
            texture,
        }
    }
}

#[inline]
fn clear_color() -> fna3d::Color {
    fna3d::Color {
        r: 100,
        g: 149,
        b: 237,
        a: 0,
    }
}

impl MainState {
    fn render_scene(&mut self) {
        use anf::batch::DrawPolicy;
        let policy = DrawPolicy { do_round: false };

        let mut push = batch::push();
        push.color = fna3d::Color {
            r: 128,
            g: 128,
            b: 128,
            a: 128,
        };
        push.src_rect(0f32, 0f32, 576f32, 384f32);
        push.dest_pos(0f32, 0f32);
        push.is_dest_size_in_pixels = false;
        push.dest_size(1f32, 1f32);
        // push.dest_size(100f32, 100f32);

        push.run(&mut self.batcher.batch, &self.texture, policy, 0);
    }
}

impl anf::State for MainState {
    fn update(&mut self) {
        // do something
    }

    fn render(&mut self) {
        Batcher::clear(&mut self.device, clear_color());
        self.batcher.begin(&mut self.device);
        self.render_scene();
        self.batcher.end(&mut self.device);
    }

    fn handle_event(&mut self, ev: &sdl2::event::Event) -> anf::StateUpdateResult {
        match ev {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => anf::StateUpdateResult::Quit,
            _ => anf::StateUpdateResult::Continue,
        }
    }
}
