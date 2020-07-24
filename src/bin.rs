//! Sample to draw a sprite. Not working somehow...

use anf::{
    gfx::{
        batcher::{self, Batcher},
        Pipeline, Texture2D,
    },
    vfs,
};

use fna3d::Device;
use sdl2::{event::Event, keyboard::Keycode};
use std::ffi::c_void;

// --------------------------------------------------------------------------------
// State & callbacks for the game loop

pub struct MainState {
    // TODO: where
    params: fna3d::PresentationParameters,
    device: Device,
    pipeline: Pipeline,
    batcher: Batcher,
    // ----------------------------------------
    // temporary fields for debugging
    texture: Texture2D,
    is_first_frame: bool,
}

impl anf::State for MainState {
    /// Does nothing for now
    fn update(&mut self) {
        // do something
    }

    /// Runs the rendering pipeline
    fn render(&mut self) {
        // stop the game on the first frame (for debug purpose for now)
        if self.is_first_frame {
            return;
        }
        // self.is_first_frame = true;

        anf::gfx::begin_frame(&mut self.device);
        anf::gfx::clear(&mut self.device);

        self.batcher.begin(&mut self.device);
        self.render_scene(); // defined below
        self.batcher.end(&mut self.device, &mut self.pipeline);

        anf::gfx::end_frame(&mut self.device, &mut self.pipeline, &mut self.batcher);
    }

    /// Just quits on `Escape` key down
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

impl MainState {
    /// Renders `Self::texture`
    fn render_scene(&mut self) {
        let policy = batcher::DrawPolicy { do_round: false };

        let mut push = batcher::push();
        push.color = fna3d::Color {
            r: 128,
            g: 128,
            b: 128,
            a: 128,
        };

        // in pixels. will be normalzied
        let w = self.texture.w as f32;
        let h = self.texture.h as f32;
        push.src_rect(0f32, 0f32, w, h);

        push.is_dest_size_in_pixels = true;
        push.dest_size(w, h);

        push.run(&mut self.batcher.batch, &self.texture, policy, 0);
    }
}

// --------------------------------------------------------------------------------
// Main

impl MainState {
    pub fn new(
        mut device: Device,
        win: *mut c_void,
        params: fna3d::PresentationParameters,
    ) -> Self {
        anf::gfx::init(&mut device, &params);

        let p = Pipeline::from_device(&mut device);
        let batcher = Batcher::new(&mut device, win);

        let texture = {
            let path = vfs::get("a.png");
            Texture2D::from_path(&mut device, &path).expect("failed to load texture")
        };

        Self {
            params,
            device,
            pipeline: p,
            batcher,
            texture,
            is_first_frame: false,
        }
    }
}

fn main() {
    env_logger::init();
    fna3d::hook_log_functions_default();

    // Create a window using SDL2
    let cfg = anf::window::Config::default();
    let (mut scx, canvas) = anf::window::create(&cfg);

    // Set up FNA3D for rendering
    let win = canvas.window().raw() as *mut _;
    let params = {
        let mut params = fna3d::utils::params_from_window_handle(win);
        params.backBufferWidth = cfg.w as i32;
        params.backBufferHeight = cfg.h as i32;
        params
    };
    let device = Device::from_params(params, true);

    // Run the game loop
    let mut state = self::MainState::new(device, win, params);
    match anf::run_loop(&mut state, &mut scx) {
        Ok(()) => {}
        Err(why) => println!("Error occured: {}", why),
    };
}
