//! Example tiled game

use std::{cmp, fs};

use anf::{gfx::prelude::*, prelude::*, vfs};
use sdl2::event::Event;

use tiled::LayerData;
pub use tiled::{Image, Layer, Map, Tile, Tileset};

use crate::{
    base::{context::Context, framework::SampleUserDataLifecycle},
    grid2d::{Rect2i, Vec2i, Vec2u},
};

pub struct TiledGameData {
    map: Map,
    texture: TextureData2d,
}

impl SampleUserDataLifecycle<Context> for TiledGameData {
    #[allow(unused_variables)]
    fn event(&mut self, cx: &mut Context, ev: &Event) -> AnfResult<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn update(&mut self, cx: &mut Context) -> AnfResult<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn render(&mut self, cx: &mut Context) -> AnfResult<()> {
        self::render_tiled(
            &mut cx.dcx,
            &self.map,
            Rect2i::new([0, 0], [1280, 720]),
            &self.texture,
        );
        Ok(())
    }

    #[allow(unused_variables)]
    fn debug_render(&mut self, cx: &mut Context) -> AnfResult<()> {
        Ok(())
    }
}

pub fn new_game(win: &WindowHandle, dcx: &mut DrawContext) -> TiledGameData {
    let path = vfs::path("map/tmx/1.tmx");
    let file = fs::File::open(&path).unwrap();

    let tiles = TextureData2d::from_path(dcx.as_mut(), vfs::path("map/images/nekura_1/m_mura.png"))
        .unwrap();

    TiledGameData {
        map: tiled::parse_with_path(file, &path).unwrap(),
        texture: tiles,
    }
}

// --------------------------------------------------------------------------------
// tiled map rendering

/// World coordinates to tile coordinates
pub fn w2t(w: impl Into<Vec2i>, map: &Map) -> Vec2i {
    let w = w.into();
    let x = (w.x as u32 + map.tile_width - 1) / map.tile_width;
    let y = (w.y as u32 + map.tile_width - 1) / map.tile_height;
    Vec2i::new(x as i32, y as i32)
}

/// Renders a tiled map in a bounds in world coordinates
pub fn render_tiled(dcx: &mut DrawContext, map: &Map, bounds: Rect2i, texture: &TextureData2d) {
    let tile_size = Vec2u::new(map.tile_width, map.tile_height);

    let left_up = {
        let mut pos = w2t(bounds.left_up(), map);
        pos.x = cmp::max(pos.x, 0);
        pos.y = cmp::max(pos.y, 0);
        pos
    };
    let right_down = {
        let mut pos = w2t(bounds.right_down(), map);
        pos.x = cmp::min(pos.x, (map.width - 1) as i32);
        pos.y = cmp::min(pos.y, (map.height - 1) as i32);
        pos
    };

    let mut pass = dcx.pass();
    for layer in map.layers.iter().filter(|l| l.visible) {
        // tiled tiles start with right-bottom so we reverse them
        // for (y, row) in layer.tiles.iter().rev().enumerate().clone() {
        //     for (x, tile) in row.iter().enumerate() {

        let tiles = match layer.tiles {
            LayerData::Finite(ref f) => f,
            LayerData::Infinite(_) => unimplemented!("tiled map infinite layer"),
        };

        for y in (left_up.y as usize)..(right_down.y as usize) {
            for x in (left_up.x as usize)..(right_down.x as usize) {
                let tile = tiles[y][x];
                if tile.gid == 0 {
                    continue;
                }

                let set = map.get_tileset_by_gid(tile.gid).unwrap();
                // the offset begins with one (not zero. zero is for "empty" tile)
                let id = tile.gid - set.first_gid;

                // TODO: detect from which image (or tile?) we're drawing
                let n_cols = set.images[0].width as u32 / set.tile_width;
                let n_rows = set.images[0].width as u32 / set.tile_height;
                // let u = (id % n_cols) as f32 / n_cols as f32;
                // let v = (id % n_rows) as f32 / n_rows as f32;

                // let mut push = anf::gfx::batcher::push();
                let src_x = id % n_cols;
                let src_y = id / n_cols;

                pass.texture(texture)
                    .src_rect_px([
                        tile_size.x as f32 * src_x as f32,
                        tile_size.x as f32 * src_y as f32,
                        tile_size.x as f32,
                        tile_size.y as f32,
                    ])
                    .dest_rect_px([
                        (
                            (x as u32 * tile_size.x - bounds.left_up().x as u32) as f32,
                            (y as u32 * tile_size.y - bounds.left_up().y as u32) as f32,
                        ),
                        (tile_size.x as f32, tile_size.y as f32),
                    ]);
            }
        }
    }
}
