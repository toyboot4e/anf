//! Pong example

// main.rs side

use anf::app::{framework::*, prelude::*};

fn main() -> AnfResult {
    env_logger::init();
    AnfFramework::with_cfg(self::config()).run(pong::new_game)
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

    use sdl2::event::Event;

    use anf::{
        gfx::prelude::*,
        input::{Key, Keyboard},
        prelude::*,
        utils::FpsCounter,
        vfs,
    };

    // --------------------------------------------------------------------------------
    // the game

    pub struct PongGameData {
        window: WindowHandle,
        game_title: String,
        fps: FpsCounter,
        input: Keyboard,
        entities: Vec<Entity>,
    }

    /// Lifecycle
    impl AnfLifecycle for PongGameData {
        fn event(&mut self, ev: &Event) {
            self.input.listen_sdl_event(ev);
        }

        fn update(&mut self, ucx: &UpdateContext) {
            if let Some(fps) = self.fps.update(ucx.time_step().elapsed()) {
                let name = format!("{} - {} FPS", &self.game_title, fps);
                self.window.set_title(&name).unwrap();
            }
            self.handle_input();
            self.handle_physics(ucx);
            self.post_physics(ucx);
        }

        fn render(&mut self, dcx: &mut DrawContext) {
            anf::gfx::clear_frame(dcx, fna3d::Color::cornflower_blue());
            self.render_scene(dcx);
        }

        fn on_next_frame(&mut self) {
            self.input.on_next_frame();
        }
    }

    /// Logic
    impl PongGameData {
        fn handle_input(&mut self) {
            if self.input.is_key_pressed(Key::D) {
                self.entities[0].vel += Vec2f::new(100.0, 0.0);
            }
            if self.input.is_key_pressed(Key::S) {
                self.entities[0].vel += Vec2f::new(0.0, 100.0);
            }
        }

        fn handle_physics(&mut self, ucx: &UpdateContext) {
            let dt = ucx.dt_secs_f32();
            for e in self.entities.iter_mut() {
                e.rect.translate(e.vel * dt);
            }
        }

        fn post_physics(&mut self, ucx: &UpdateContext) {
            let size = ucx.screen().size();
            for e in self.entities.iter_mut() {
                // TODO: handle velocity
                e.rect.clamp_x(0.0, size.x);
                e.rect.clamp_y(0.0, size.y);
            }
        }

        fn render_scene(&mut self, dcx: &mut DrawContext) {
            let mut pass = dcx.pass();
            for e in &self.entities {
                pass.sprite(&e.sprite).dest_pos_px(e.rect.left_up());
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

    /// Initializes the `PongGameData`] with two paddles and one ball
    pub fn new_game(
        window: WindowHandle,
        cfg: &WindowConfig,
        dcx: &mut DrawContext,
    ) -> PongGameData {
        let atlas = TextureData2D::from_path(dcx, vfs::path("ikachan.png")).unwrap();
        let atlas_size_px: Vec2f = atlas.size().into();

        // uv, I mean, normalized
        let paddle_size_uv = Vec2f::new(1.0 / 3.0, 3.0 * 1.0 / 4.0);
        let ball_size_uv = Vec2f::new(1.0 / 3.0, 1.0 * 1.0 / 4.0);

        let paddle_sprite = SpriteData {
            texture: atlas.clone(),
            uv_rect: [(0.0, 0.0), paddle_size_uv.into()].into(),
            origin: Vec2f::new(0.5, 0.5),
            ..Default::default()
        };

        let ball_sprite = SpriteData {
            texture: atlas.clone(),
            uv_rect: [(2.0 / 3.0, 0.0), ball_size_uv.into()].into(),
            origin: Vec2f::new(0.5, 0.5),
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
            window,
            game_title: cfg.title.clone(),
            fps: FpsCounter::default(),
            input: Keyboard::new(),
            entities: vec![left, right, ball],
        }
    }
}
