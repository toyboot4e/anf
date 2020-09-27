//! Example tiled game

use std::fs;

use anf::{engine::prelude::*, gfx::prelude::*, input::Key, vfs};
use sdl2::event::Event;

pub use tiled::{Image, Layer, Map, Tile, Tileset};

use crate::{
    base::{context::Context, framework::SampleUserDataLifecycle},
    grid2d::{Rect2i, Vec2i},
    render::tiled_render,
};

pub struct TiledGameData {
    map: Map,
    texture: TextureData2d,
    world: World,
}

impl SampleUserDataLifecycle<Context> for TiledGameData {
    fn update(&mut self, cx: &mut Context) -> AnfResult<()> {
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

        Ok(())
    }

    fn render(&mut self, cx: &mut Context) -> AnfResult<()> {
        tiled_render::render_tiled(
            &mut cx.dcx,
            &self.map,
            &self.texture,
            (self.world.camera.pos, [1280.0, 720.0]),
        );
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

#[derive(Debug, Clone)]
pub struct Player {
    sprite: SpriteData,
    pos: Vec2i,
}

pub fn new_game(win: &WindowHandle, dcx: &mut DrawContext) -> TiledGameData {
    let path = vfs::path("map/tmx/1.tmx");
    let file = fs::File::open(&path).unwrap();

    let tiles = TextureData2d::from_path(dcx.as_mut(), vfs::path("map/images/nekura_1/m_mura.png"))
        .unwrap();

    let atlas = TextureData2d::from_path(dcx.as_mut(), vfs::path("ika-chan.png")).unwrap();
    let sprite = SpriteData {
        texture: atlas,
        uv_rect: [(2.0 / 3.0, 0.0), (1.0 / 3.0, 1.0 / 4.0)].into(),
        ..Default::default()
    };

    let world = World {
        camera: Camera::default(),
        player: Player {
            sprite,
            pos: Vec2i::default(),
        },
    };

    TiledGameData {
        map: tiled::parse_with_path(file, &path).unwrap(),
        texture: tiles,
        world,
    }
}
