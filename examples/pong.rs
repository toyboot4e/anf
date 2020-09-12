use anf::fna3d;
use anf::sdl2::event::Event;
use anf::{
    game::{
        app::{App, AppConfig},
        input::{Key, Keyboard},
        run_game, GameResult, GameState,
    },
    gfx::{
        geom::{Rect2f, Vec2f},
        prelude::*,
        SubTextureData2D, TextureData2D,
    },
    vfs,
};

// --------------------------------------------------------------------------------
// game data

#[derive(Debug)]
struct PongGameData {
    input: Keyboard,
    entities: Entities,
    textures: Textures,
}

fn new_game(device: &mut fna3d::Device) -> PongGameData {
    let left = Paddle {
        pos: [100.0, 100.0].into(),
        vel: [0.0, 0.0].into(),
    };

    let right = Paddle {
        pos: [1000.0, 100.0].into(),
        vel: [0.0, 0.0].into(),
    };

    let paddle = TextureData2D::from_path(device, vfs::path("pong/paddle.png")).unwrap();
    let paddle = paddle.trim_px([0, 0, 90, 288]);
    let textures = Textures {
        paddle,
        ball: TextureData2D::from_path(device, vfs::path("pong/paddle.png")).unwrap(),
    };

    PongGameData {
        input: Keyboard::new(),
        entities: Entities {
            left,
            right,
            ball: Ball::default(),
        },
        textures,
    }
}

impl PongGameData {
    fn handle_input(&mut self) {
        if self.input.is_key_pressed(Key::D) {
            self.entities.left.vel += Vec2f::new(5.0, 0.0);
            println!("pressed");
        }
    }

    fn handle_physics(&mut self) {
        let dt = 1.0 / 60.0; // FIXME: dt input
        for e in &mut [&mut self.entities.left, &mut self.entities.right] {
            e.pos += e.vel * dt;
        }
    }
}

impl GameState for PongGameData {
    // TODO: delta time
    fn update(&mut self) {
        self.handle_input();
        self.handle_physics();
    }

    fn render(&mut self, dcx: &mut DrawContext) {
        anf::gfx::clear_frame(dcx, fna3d::Color::cornflower_blue());

        let mut pass = dcx.pass();
        pass.cmd()
            .dest_pos_px(&self.entities.left.pos)
            .texture(&self.textures.paddle);

        pass.cmd()
            .dest_pos_px(&self.entities.right.pos)
            .texture(&self.textures.paddle);

        self.input.on_next_frame(); // FIXME:
    }

    fn listen_event(&mut self, ev: &Event) {
        match ev {
            Event::KeyDown {
                keycode: Some(sdl_key),
                ..
            } => {
                self.input.on_key_down(*sdl_key);
            }
            Event::KeyUp {
                keycode: Some(sdl_key),
                ..
            } => {
                self.input.on_key_up(*sdl_key);
            }
            _ => {}
        }
    }
}

#[derive(Debug, Clone)]
struct Textures {
    paddle: SubTextureData2D,
    ball: TextureData2D,
}

#[derive(Debug, Clone, Default)]
struct Entities {
    left: Paddle,
    right: Paddle,
    ball: Ball,
}

// Entities

#[derive(Debug, Clone, Default)]
struct Paddle {
    pos: Vec2f,
    vel: Vec2f,
}

#[derive(Debug, Clone, Default)]
struct Ball {
    pos: Vec2f,
    vel: Vec2f,
}

// -----------------------------------------------------------------------------------------
// run

fn main() -> GameResult {
    anf::env_logger::init();
    let mut app = App::from_cfg(self::app_config());
    let state = new_game(&mut app.device);
    run_game(app, state)
}

fn app_config() -> AppConfig {
    AppConfig {
        title: "ANF pong game".to_string(),
        w: 1280,
        h: 720,
    }
}
