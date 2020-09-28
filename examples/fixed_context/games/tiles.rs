//! Example tiled game

use std::{collections::HashMap, fs};

use anf::{engine::prelude::*, gfx::prelude::*, input::Key, vfs};
use sdl2::event::Event;
pub use tiled::{Image, Layer, Map, Tile, Tileset};

use crate::{
    base::{context::Context, framework::SampleUserDataLifecycle},
    render::tiled_render,
    utils::{
        anim::{LoopMode, SpriteAnimPattern, SpriteAnimState},
        grid2d::{Dir4, Dir8, Rect2i, Vec2i},
    },
};

pub struct TiledGameData {
    map: Map,
    texture: TextureData2d,
    world: World,
}

impl SampleUserDataLifecycle<Context> for TiledGameData {
    fn update(&mut self, cx: &mut Context) -> AnfResult<()> {
        // update animation stat
        self.world.player.anim.tick(cx.time_step());

        // update physics
        let dt = cx.dcx.dt_secs_f32();

        let v = 640.0;
        if cx.kbd.is_key_down(Key::A) {
            self.world.camera.pos.x -= v * dt;
        }
        if cx.kbd.is_key_down(Key::D) {
            self.world.camera.pos.x += v * dt;
        }
        if cx.kbd.is_key_down(Key::W) {
            self.world.camera.pos.y -= v * dt;
        }
        if cx.kbd.is_key_down(Key::S) {
            self.world.camera.pos.y += v * dt;
        }

        if cx.kbd.is_key_pressed(Key::Left) {
            self.world.player.dir = Dir8::W;
            self.world.player.anim.set_pattern(Dir8::W, false);
            self.world.player.pos.x -= 1;
        }
        if cx.kbd.is_key_pressed(Key::Right) {
            self.world.player.dir = Dir8::E;
            self.world.player.anim.set_pattern(Dir8::E, false);
            self.world.player.pos.x += 1;
        }
        if cx.kbd.is_key_pressed(Key::Up) {
            self.world.player.dir = Dir8::N;
            self.world.player.anim.set_pattern(Dir8::N, false);
            self.world.player.pos.y -= 1;
        }
        if cx.kbd.is_key_pressed(Key::Down) {
            self.world.player.dir = Dir8::S;
            self.world.player.anim.set_pattern(Dir8::S, false);
            self.world.player.pos.y += 1;
        }

        Ok(())
    }

    fn render(&mut self, cx: &mut Context) -> AnfResult<()> {
        tiled_render::render_tiled(
            &mut cx.dcx,
            &self.map,
            &self.texture,
            (self.world.camera.pos, [1280.0, 720.0]),
        );

        let mut pass = cx.dcx.pass();

        let pos = self.world.player.pos * 32;
        let pos = Vec2f::new(pos.x as f32, pos.y as f32) + Vec2f::new(16.0, 16.0);
        let pos = pos - self.world.camera.pos;
        let sprite = self.world.player.anim.current_frame();
        pass.sprite(sprite).dest_pos_px(pos);

        Ok(())
    }
}

pub struct World {
    camera: Camera,
    player: Player,
}

#[derive(Debug, Clone, Default)]
pub struct Camera {
    /// Top-left point
    pos: Vec2f,
}

#[derive(Debug)]
pub struct Player {
    // sprite: SpriteData,
    anim: SpriteAnimState<Dir8>,
    pos: Vec2i,
    dir: Dir8,
}

pub fn new_game(win: &WindowHandle, dcx: &mut DrawContext) -> TiledGameData {
    let path = vfs::path("map/tmx/1.tmx");
    let file = fs::File::open(&path).unwrap();

    let tiles = TextureData2d::from_path(dcx.as_mut(), vfs::path("map/images/nekura_1/m_mura.png"))
        .unwrap();

    let atlas = TextureData2d::from_path(dcx.as_mut(), vfs::path("ika-chan.png")).unwrap();
    let anim = {
        let patterns = gen_anim4(&atlas, 4.0);
        SpriteAnimState::new(patterns, Dir8::S)
    };

    // let sprite = SpriteData {
    //     texture: atlas,
    //     uv_rect: [(2.0 / 3.0, 0.0), (1.0 / 3.0, 1.0 / 4.0)].into(),
    //     origin: [0.5, 0.5].into(),
    //     ..Default::default()
    // };

    let world = World {
        camera: Camera::default(),
        player: Player {
            // sprite,
            anim,
            pos: Vec2i::default(),
            dir: Dir8::S,
        },
    };

    TiledGameData {
        map: tiled::parse_with_path(file, &path).unwrap(),
        texture: tiles,
        world,
    }
}

/// Creates walking animation from 4x3 character image
fn gen_anim4(texture: &TextureData2d, fps: f32) -> HashMap<Dir8, SpriteAnimPattern> {
    [
        (Dir8::E, [6, 7, 8]),
        (Dir8::W, [3, 4, 5]),
        (Dir8::S, [0, 1, 2]),
        (Dir8::SE, [0, 1, 2]),
        (Dir8::SW, [0, 1, 2]),
        (Dir8::N, [9, 10, 11]),
        (Dir8::NE, [9, 10, 11]),
        (Dir8::NW, [9, 10, 11]),
    ]
    .iter()
    .map(|(dir, ixs)| {
        (
            dir.clone(),
            SpriteAnimPattern::new(
                ixs.iter()
                    .map(|ix| {
                        let row = ix / 3;
                        let col = ix % 3;
                        let uv = (col as f32 / 3.0, row as f32 / 4.0);
                        SpriteData {
                            texture: texture.clone(),
                            uv_rect: Rect2f::from([uv, (1.0 / 3.0, 1.0 / 4.0)]),
                            ..Default::default()
                        }
                    })
                    .collect::<Vec<_>>(),
                fps,
                LoopMode::PingPong,
            ),
        )
    })
    .collect()
}
