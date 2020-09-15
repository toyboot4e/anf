//! Pong example

use anf::framework::*;

fn main() -> AnfResult {
    env_logger::init();
    anf_run_game(&self::config(), pong::new_game)
}

pub fn config() -> AnfConfig {
    AnfConfig {
        title: "Pong".to_string(),
        w: 1280,
        h: 720,
    }
}

mod pong {
    use anf::prelude::*;
    use fna3d::Color;
    use sdl2::event::Event;

    use anf::{
        gfx::{
            geom::{Rect2f, Vec2f},
            SubTextureData2D, TextureData2D,
        },
        input::{Key, Keyboard},
        vfs,
    };

    // --------------------------------------------------------------------------------
    // game data

    #[derive(Debug)]
    pub struct PongGameData {
        input: Keyboard,
        textures: Textures,
        entities: Entities,
    }

    pub fn new_game(device: &mut fna3d::Device) -> PongGameData {
        let left = Paddle {
            pos: [100.0, 100.0].into(),
            vel: [0.0, 0.0].into(),
        };

        let right = Paddle {
            pos: [1000.0, 100.0].into(),
            vel: [0.0, 0.0].into(),
        };

        let textures = {
            let paddle = TextureData2D::from_path(device, vfs::path("pong/paddle.png")).unwrap();
            let paddle = paddle.trim_px([0, 0, 90, 288]);
            Textures {
                paddle,
                ball: TextureData2D::from_path(device, vfs::path("pong/paddle.png")).unwrap(),
            }
        };

        PongGameData {
            input: Keyboard::new(),
            textures,
            entities: Entities {
                left,
                right,
                ball: Ball::default(),
            },
        }
    }

    /// Logic
    impl PongGameData {
        fn handle_input(&mut self) {
            if self.input.is_key_pressed(Key::D) {
                self.entities.left.vel += Vec2f::new(100.0, 0.0);
                println!("D");
            }
            if self.input.is_key_pressed(Key::S) {
                self.entities.left.vel += Vec2f::new(0.0, 100.0);
                println!("S");
            }
        }

        fn handle_physics(&mut self, ts: TimeStep) {
            let dt = ts.dt_secs_f32();
            // wow, ECS looks simpler than this
            for e in &mut [&mut self.entities.left, &mut self.entities.right] {
                e.pos += e.vel * dt;
            }
        }

        fn render_scene(&mut self, dcx: &mut DrawContext) {
            let mut pass = dcx.pass();
            pass.cmd()
                .dest_pos_px(&self.entities.left.pos)
                .texture(&self.textures.paddle);

            pass.cmd()
                .dest_pos_px(&self.entities.right.pos)
                .texture(&self.textures.paddle);
        }
    }

    /// Lifecycle
    impl AnfLifecycle for PongGameData {
        fn event(&mut self, ev: &Event) {
            self.input.listen_sdl_event(ev);
        }

        fn update(&mut self, ts: TimeStep) {
            self.handle_input();
            self.handle_physics(ts);
        }

        fn render(&mut self, dcx: &mut DrawContext) {
            anf::gfx::clear_frame(dcx, fna3d::Color::cornflower_blue());
            self.render_scene(dcx);
        }

        fn on_next_frame(&mut self) {
            self.input.on_next_frame();
        }
    }

    // --------------------------------------------------------------------------------
    // World

    // (Not) generic resources

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
}
