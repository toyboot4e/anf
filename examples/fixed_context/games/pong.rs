//! Pong example
//!
//! Note that all the sprites have origin at (0, 0), i.e. top left. Or else, our `Rect2f` methods
//! do not make sense.

use anf::prelude::*;

use anf::{
    gfx::prelude::*,
    input::{Key, Keyboard},
    vfs,
};

use crate::{context::Context, framework::SampleUserDataLifecycle};

pub struct PongGameData {
    entities: Vec<Entity>,
}

impl PongGameData {
    pub fn from_cx(cx: &mut Context) -> Self {
        new_game(&cx.win, &mut cx.dcx)
    }
}

impl SampleUserDataLifecycle<Context> for PongGameData {
    fn update(&mut self, cx: &mut Context) -> AnfResult<()> {
        let dt = cx.dcx.dt_secs_f32();
        let size = cx.dcx.screen().size();

        self.handle_input(&cx.kbd);
        self.handle_physics(dt);
        self.post_physics(dt, size);

        Ok(())
    }

    fn render(&mut self, cx: &mut Context) -> AnfResult<()> {
        let mut pass = cx.dcx.pass();
        for e in &self.entities {
            pass.sprite(&e.sprite).dest_pos_px(e.rect.left_up());
        }

        Ok(())
    }
}

/// Updating logic
impl PongGameData {
    fn handle_input(&mut self, kbd: &Keyboard) {
        if kbd.is_key_pressed(Key::W) {
            self.entities[0].vel = Vec2f::new(0.0, -400.0);
        }
        if kbd.is_key_pressed(Key::S) {
            self.entities[0].vel = Vec2f::new(0.0, 400.0);
        }
        if kbd.is_key_pressed(Key::A) || kbd.is_key_pressed(Key::D) {
            self.entities[0].vel = Vec2f::zero();
        }
    }

    fn handle_physics(&mut self, dt: f32) {
        for e in &mut self.entities {
            e.rect.translate(e.vel * dt);
        }
    }

    fn post_physics(&mut self, dt: f32, screen_size: Vec2f) {
        self.apply_bouce(screen_size);

        for e in self.entities.iter_mut() {
            // TODO: top
            e.rect.clamp_x(0.0, screen_size.x);
            e.rect.clamp_y(0.0, screen_size.y);
        }
    }
}

impl PongGameData {
    fn apply_bouce(&mut self, screen_size: Vec2f) {
        let screen: Rect2f = [(0.0, 0.0).into(), screen_size].into();

        // this is unfortunate nature of Rust
        let (left, right, ball) = unsafe {
            (
                &self.entities[0],
                &self.entities[1],
                &mut *(&self.entities[2] as *const Entity as *mut Entity),
            )
        };

        // bounce on left/right edge of screen
        if ball.rect.left() < screen.left() || screen.right() < ball.rect.right() {
            // TODO: delete the ball
            ball.vel.x = -ball.vel.x;
        }

        // bounce on top/bottom edge of screen
        if ball.rect.top() < screen.top() || screen.bottom() < ball.rect.bottom() {
            ball.vel.y = -ball.vel.y;
        }

        // bounce on paddle
        for paddle in &[left, right] {
            if !paddle.rect.intersects(&ball.rect) {
                continue;
            }

            let dx_1 = (ball.rect.left() - paddle.rect.right()).abs();
            let dx_2 = (paddle.rect.left() - ball.rect.right()).abs();
            if dx_1 < 5.0 || dx_2 < 5.0 {
                // on left/right edge
                ball.vel.x = -ball.vel.x;
            } else {
                // on top/bottom edge
                ball.vel.y = -ball.vel.y;
            }
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
    let atlas = TextureData2d::from_path(dcx.as_mut(), vfs::path("ika-chan.png")).unwrap();
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
        rect: ([1100.0, 100.0], paddle_size_px).into(),
        vel: Vec2f::zero(),
        sprite: paddle_sprite.clone(),
    };

    let ball_size_px = ball_size_uv * atlas_size_px;
    let vel = Vec2f::new(-480.0, 480.0);
    let ball = Entity {
        rect: (dcx.screen().center(), ball_size_px).into(),
        // vel: [100.0, (rand::random::<u32>() % 100) as f32].into(),
        vel,
        sprite: ball_sprite.clone(),
    };

    PongGameData {
        entities: vec![left, right, ball],
    }
}
