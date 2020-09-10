use anf::fna3d;
use anf::sdl2::event::Event;
use anf::{
    game::{
        app::{App, AppConfig},
        input::Input,
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
    input: Input,
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
        input: Input::new(),
        entities: Entities {
            left,
            right,
            ball: Ball::default(),
        },
        textures,
    }
}

impl GameState for PongGameData {
    fn update(&mut self) {
        //
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
    }

    fn listen_event(&mut self, ev: &Event) {
        //
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
