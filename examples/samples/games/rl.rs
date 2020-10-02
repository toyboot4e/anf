//! Roguelike game example

use anf::{engine::prelude::*, gfx::prelude::*, input::Key, vfs};
pub use tiled::{Image, Layer, Map, Tile, Tileset};

use crate::{
    base::{context::Context, framework::SampleUserDataLifecycle},
    render::tiled_render,
    rl::{self, fov::*, view::TiledRlMap},
    utils::{
        anim::SpriteAnimState,
        grid2d::{Dir8, Vec2i},
    },
};

#[repr(u32)]
enum Gid {
    Floor = 9,
    Ceil = 25,
}

pub struct RlGameData {
    map: TiledRlMap,
    camera: Camera,
    player: Player,
}

impl SampleUserDataLifecycle<Context> for RlGameData {
    fn update(&mut self, cx: &mut Context) -> AnfResult<()> {
        if cx.kbd.is_key_pressed(Key::R) {
            self::gen_cave(&mut self.map.tiled, &mut self.map.rlmap.blocks);
        }

        // update animation stat
        self.player.anim.tick(cx.time_step());

        // update physics
        let dt = cx.dcx.dt_secs_f32();

        // scroll
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

        // walk
        let mut pos = self.player.pos;
        let mut dir = self.player.dir;

        if cx.kbd.is_key_pressed(Key::Left) {
            dir = Dir8::W;
            pos.x -= 1;
        }
        if cx.kbd.is_key_pressed(Key::Right) {
            dir = Dir8::E;
            pos.x += 1;
        }
        if cx.kbd.is_key_pressed(Key::Up) {
            dir = Dir8::N;
            pos.y -= 1;
        }
        if cx.kbd.is_key_pressed(Key::Down) {
            dir = Dir8::S;
            pos.y += 1;
        }

        if pos != self.player.pos && !self.map.rlmap.is_blocked(pos) {
            self.player.pos = pos;
            self.player
                .fov
                .update(5, self.player.pos, &mut self.map.rlmap);
        }
        self.player.dir = dir;
        self.player.anim.set_pattern(dir, false);

        Ok(())
    }

    fn render(&mut self, cx: &mut Context) -> AnfResult<()> {
        anf::gfx::clear_frame(&mut cx.dcx, fna3d::Color::rgb(210, 70, 70));

        // map
        let px_bounds = Rect2f::from((self.camera.pos, [1280.0, 720.0]));
        self.map.render(&mut cx.dcx, px_bounds.clone());

        // grids
        // let mut pass = cx.dcx.pass();
        // tiled_render::render_non_blocking_grids(
        //     &mut pass,
        //     &self.map.tiled,
        //     &self.map.rlmap.blocks,
        //     &px_bounds,
        // );
        // drop(pass);

        // player
        let mut pass = cx.dcx.pass();
        let pos = self.player.pos * 32;
        let pos = Vec2f::new(pos.x as f32, pos.y as f32) + Vec2f::new(16.0, 16.0);
        let pos = pos - self.camera.pos;
        let sprite = self.player.anim.current_frame();
        pass.sprite(sprite).dest_pos_px(pos);
        drop(pass);

        // player fov
        let mut pass = cx.dcx.pass();
        tiled_render::render_fov_shadows(&mut pass, &self.map.tiled, &self.player.fov, &px_bounds);
        drop(pass);

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
    fov: FovData,
}

fn clear_tiled(tiled: &mut tiled::Map) {
    for layer in &mut tiled.layers {
        let tiles = match &mut layer.tiles {
            tiled::LayerData::Finite(tiles) => tiles,
            tiled::LayerData::Infinite(_) => unimplemented!(),
        };

        for y in 0..tiled.height {
            for x in 0..tiled.width {
                tiles[y as usize][x as usize].gid = 0;
            }
        }
    }
}

fn gen_cave(tiled: &mut tiled::Map, blocks: &mut [bool]) {
    let size = [tiled.width as usize, tiled.height as usize];
    let cave = crate::rl::dun::gen_cave(size, 50, 20);

    let tile_layer = &mut tiled.layers[0];
    let tiles = {
        match &mut tile_layer.tiles {
            tiled::LayerData::Finite(tiles) => tiles,
            tiled::LayerData::Infinite(_) => unimplemented!(),
        }
    };

    for y in 0..size[1] {
        for x in 0..size[0] {
            let ix = x + y * size[0];
            // we need the offset
            let gid = 1 + if cave[ix] { Gid::Floor } else { Gid::Ceil } as u32;
            tiles[y][x] = tiled::LayerTile::new(gid);
            blocks[ix] = !cave[ix];
        }
    }
}

pub fn new_game(win: &WindowHandle, dcx: &mut DrawContext) -> RlGameData {
    let path = vfs::path("map/tmx/rl.tmx");
    let rlmap = {
        let mut map = rl::view::TiledRlMap::from_tiled_path(&path, dcx.device_mut());
        self::clear_tiled(&mut map.tiled);
        self::gen_cave(&mut map.tiled, &mut map.rlmap.blocks);
        map
    };

    let ika_atlas = TextureData2d::from_path(dcx.as_mut(), vfs::path("ika-chan.png")).unwrap();
    let ika_anim = {
        let origin = [0.5, 0.8].into();
        let patterns = rl::view::gen_anim4_with(&ika_atlas, 4.0, |s| {
            s.origin = origin;
            s.color = Color::rgb(255, 255, 100);
        });
        SpriteAnimState::new(patterns, Dir8::S)
    };

    let player = Player {
        anim: ika_anim,
        pos: Vec2i::default(),
        dir: Dir8::S,
        fov: FovData::new(10),
    };

    // TODO: update FoV here

    RlGameData {
        map: rlmap,
        camera: Camera::default(),
        player,
    }
}