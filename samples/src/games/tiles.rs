//! Example tiled game

use anf::{engine::prelude::*, gfx::prelude::*, input::Key, vfs};
pub use tiled::{Image, Layer, Map, Tile, Tileset};

use crate::{
    base::{context::Context, framework::SampleUserDataLifecycle},
    rl::{
        self,
        grid2d::{Dir8, Vec2i},
        rlmap::TiledRlMap,
    },
    utils::anim::SpriteAnimState,
};

pub struct TiledGameData {
    rlmap: TiledRlMap,
    camera: Camera,
    player: Player,
}

impl SampleUserDataLifecycle<Context> for TiledGameData {
    fn update(&mut self, cx: &mut Context) -> AnfResult<()> {
        // update animation stat
        self.player.anim.tick(cx.time_step());

        // update physics
        let dt = cx.dcx.dt_secs_f32();

        let v = 640.0;
        if cx.kbd.is_key_down(Key::A) {
            self.camera.pos.x -= v * dt;
        }
        if cx.kbd.is_key_down(Key::D) {
            self.camera.pos.x += v * dt;
        }
        if cx.kbd.is_key_down(Key::W) {
            self.camera.pos.y -= v * dt;
        }
        if cx.kbd.is_key_down(Key::S) {
            self.camera.pos.y += v * dt;
        }

        if cx.kbd.is_key_pressed(Key::Left) {
            self.player.dir = Dir8::W;
            self.player.anim.set_pattern(Dir8::W, false);
            self.player.pos.x -= 1;
        }
        if cx.kbd.is_key_pressed(Key::Right) {
            self.player.dir = Dir8::E;
            self.player.anim.set_pattern(Dir8::E, false);
            self.player.pos.x += 1;
        }
        if cx.kbd.is_key_pressed(Key::Up) {
            self.player.dir = Dir8::N;
            self.player.anim.set_pattern(Dir8::N, false);
            self.player.pos.y -= 1;
        }
        if cx.kbd.is_key_pressed(Key::Down) {
            self.player.dir = Dir8::S;
            self.player.anim.set_pattern(Dir8::S, false);
            self.player.pos.y += 1;
        }

        Ok(())
    }

    fn render(&mut self, cx: &mut Context) -> AnfResult<()> {
        anf::gfx::clear_frame(&mut cx.dcx, fna3d::Color::rgb(21, 164, 185));

        self.rlmap
            .render(&mut cx.dcx, (self.camera.pos, [1280.0, 720.0]));

        let mut pass = cx.dcx.pass();

        let pos = self.player.pos * 32;
        let pos = Vec2f::new(pos.x as f32, pos.y as f32) + Vec2f::new(16.0, 16.0);
        let pos = pos - self.camera.pos;
        let sprite = self.player.anim.current_frame();
        pass.sprite(sprite).dest_pos_px(pos);

        Ok(())
    }
}

pub struct World {}

#[derive(Debug, Clone, Default)]
pub struct Camera {
    /// Top-left point
    pos: Vec2f,
}

#[derive(Debug)]
pub struct Player {
    anim: SpriteAnimState<Dir8>,
    pos: Vec2i,
    dir: Dir8,
}

pub fn new_game(win: &WindowHandle, dcx: &mut DrawContext) -> TiledGameData {
    let path = vfs::path("map/tmx/tiles.tmx");
    let rlmap = rl::rlmap::TiledRlMap::from_tiled_path(&path, dcx.device_mut());

    let ika_atlas = TextureData2d::from_path(dcx.as_mut(), vfs::path("ika-chan.png")).unwrap();
    let ika_anim = {
        let patterns = rl::view::gen_anim4(&ika_atlas, 4.0);
        SpriteAnimState::new(patterns, Dir8::S)
    };

    TiledGameData {
        rlmap,
        camera: Camera::default(),
        player: Player {
            anim: ika_anim,
            pos: Vec2i::default(),
            dir: Dir8::S,
        },
    }
}
