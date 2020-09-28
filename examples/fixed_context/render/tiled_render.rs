use std::cmp;

use anf::{engine::prelude::*, gfx::prelude::*};

use tiled::LayerData;
pub use tiled::{Image, Layer, Map, Tile, Tileset};

use crate::utils::grid2d::{Rect2i, Vec2i, Vec2u};

/// World coordinates to tile coordinates rounding up remaning pixels (this is visually correct)
pub fn w2t_round_up(w: impl Into<Vec2f>, map: &Map) -> Vec2i {
    let w = w.into();
    let x = (w.x as u32 + map.tile_width - 1) / map.tile_width;
    let y = (w.y as u32 + map.tile_width - 1) / map.tile_height;
    Vec2i::new(x as i32, y as i32)
}

/// World coordinates to tile coordinates flooring remaning pixels
pub fn w2t_floor(w: impl Into<Vec2f>, map: &Map) -> Vec2i {
    let w = w.into();
    let x = w.x as u32 / map.tile_width;
    let y = w.y as u32 / map.tile_height;
    Vec2i::new(x as i32, y as i32)
}

/// Renders a tiled map in a bounds in world coordinates
pub fn render_tiled(
    dcx: &mut DrawContext,
    map: &Map,
    texture: &TextureData2d,
    bounds: impl Into<Rect2f>,
) {
    let bounds = bounds.into();

    let left_up = {
        let mut pos = w2t_floor(bounds.left_up(), map);
        pos.x = cmp::max(pos.x, 0);
        pos.y = cmp::max(pos.y, 0);
        pos
    };

    let right_down = {
        let mut pos = w2t_round_up(bounds.right_down(), map);
        pos.x = cmp::min(pos.x, (map.width - 1) as i32);
        pos.y = cmp::min(pos.y, (map.height - 1) as i32);
        pos
    };

    let tile_size = Vec2u::new(map.tile_width, map.tile_height);

    let mut pass = dcx.pass();
    for layer in map.layers.iter().filter(|l| l.visible) {
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
                // let n_rows = set.images[0].width as u32 / set.tile_height;
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
                            (x as i32 * tile_size.x as i32 - bounds.left_up().x as i32) as f32,
                            (y as i32 * tile_size.y as i32 - bounds.left_up().y as i32) as f32,
                        ),
                        (tile_size.x as f32, tile_size.y as f32),
                    ]);
            }
        }
    }
}
