//! Pong example

// main.rs side

use anf::{
    game::{AnfGame, AnfGameState, Context},
    prelude::*,
};

fn main() -> AnfAppResult {
    env_logger::init();
    AnfGame::run(config(), pong::PongGameData::from_cx)
}

pub fn config() -> WindowConfig {
    WindowConfig {
        title: "Pong".to_string(),
        w: 1280,
        h: 720,
        ..Default::default()
    }
}

mod pong {
    //! lib.rs side

    use anf::{
        game::{AnfGame, AnfGameState, Context},
        prelude::*,
    };
    use sdl2::event::Event;

    use anf::{
        gfx::prelude::*,
        input::{Key, Keyboard},
        vfs,
    };

    // --------------------------------------------------------------------------------
    // the game

    pub struct PongGameData {
        entities: Vec<Entity>,
    }

    impl PongGameData {
        pub fn from_cx(cx: &mut Context) -> Self {
            new_game(&cx.win, &mut cx.dcx)
        }
    }

    impl AnfGameState for PongGameData {
        fn update(&mut self, cx: &mut Context) {
            let dt = cx.dcx.dt_secs_f32();
            let size = cx.dcx.screen().size();

            self.handle_input(&cx.kbd);
            self.handle_physics(dt);
            self.post_physics(dt, size);
        }

        fn render(&mut self, cx: &mut Context) {
            let mut pass = cx.dcx.pass();
            for e in &self.entities {
                pass.sprite(&e.sprite).dest_pos_px(e.rect.left_up());
            }
        }
    }

    //     fn update(&mut self, ucx: &UpdateContext) {
    //         if let Some(fps) = self.fps.update(ucx.time_step().elapsed()) {
    //             let name = format!("{} - {} FPS", &self.game_title, fps);
    //             self.window.set_title(&name).unwrap();
    //         }
    //         self.handle_input();
    //         self.handle_physics(ucx);
    //         self.post_physics(ucx);
    //     }

    //     fn render(&mut self, dcx: &mut DrawContext) {
    //         anf::gfx::clear_frame(dcx, fna3d::Color::cornflower_blue());

    //         let mut pass = dcx.pass();
    //         for e in &self.entities {
    //             pass.sprite(&e.sprite).dest_pos_px(e.rect.left_up());
    //         }
    //     }

    //     fn on_next_frame(&mut self) {
    //         self.input.on_next_frame();
    //     }
    // }

    /// Updating logic
    impl PongGameData {
        fn handle_input(&mut self, kbd: &Keyboard) {
            if kbd.is_key_pressed(Key::D) {
                self.entities[0].vel += Vec2f::new(100.0, 0.0);
            }
            if kbd.is_key_pressed(Key::S) {
                self.entities[0].vel += Vec2f::new(0.0, 100.0);
            }
        }

        fn handle_physics(&mut self, dt: f32) {
            for e in self.entities.iter_mut() {
                e.rect.translate(e.vel * dt);
            }
        }

        fn post_physics(&mut self, dt: f32, screen_size: Vec2f) {
            for e in self.entities.iter_mut() {
                // TODO: top
                e.rect.clamp_x(0.0, screen_size.x);
                e.rect.clamp_y(0.0, screen_size.y);
            }
        }
    }

    // --------------------------------------------------------------------------------
    // World

    #[derive(Debug, Clone, Default)]
    struct Entity {
        rect: Rect2f,
        vel: Vec2f,
        sprite: SpriteData,
    }

    /// Initializes the [`PongGameData`] with two paddles and one ball
    pub fn new_game(win: &WindowHandle, dcx: &mut DrawContext) -> PongGameData {
        let atlas = TextureData2D::from_path(dcx, vfs::path("ikachan.png")).unwrap();
        let atlas_size_px: Vec2f = atlas.size().into();

        // uv, I mean, normalized
        let paddle_size_uv = Vec2f::new(1.0 / 3.0, 3.0 * 1.0 / 4.0);
        let ball_size_uv = Vec2f::new(1.0 / 3.0, 1.0 * 1.0 / 4.0);

        // TODO: center origin
        // TODO: rotation and bounds (use matrix?)
        let origin = Vec2f::new(0.0, 0.0);

        let paddle_sprite = SpriteData {
            texture: atlas.clone(),
            uv_rect: [(0.0, 0.0), paddle_size_uv.into()].into(),
            origin,
            ..Default::default()
        };

        let ball_sprite = SpriteData {
            texture: atlas.clone(),
            uv_rect: [(2.0 / 3.0, 0.0), ball_size_uv.into()].into(),
            origin,
            ..Default::default()
        };

        let paddle_size_px = paddle_size_uv * atlas_size_px;
        let left = Entity {
            rect: ([100.0, 100.0], paddle_size_px).into(),
            vel: Vec2f::zero(),
            sprite: paddle_sprite.clone(),
        };

        let right = Entity {
            rect: ([1000.0, 100.0], paddle_size_px).into(),
            vel: Vec2f::zero(),
            sprite: paddle_sprite.clone(),
        };

        let ball_size_px = ball_size_uv * atlas_size_px;
        let ball = Entity {
            rect: (dcx.screen().center(), ball_size_px).into(),
            vel: Vec2f::zero(),
            sprite: ball_sprite.clone(),
        };

        PongGameData {
            entities: vec![left, right, ball],
        }
    }
}
