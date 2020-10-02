//! Tiled map rendering

use std::cmp;

use anf::{engine::prelude::*, gfx::prelude::*};

use tiled::LayerData;

use crate::{
    rl::fov::*,
    utils::grid2d::{Rect2i, Vec2i, Vec2u},
};

// --------------------------------------------------------------------------------
// Coordinates

/// World coordinates to tile coordinates rounding up remaning pixels (this is visually correct)
pub fn w2t_round_up(w: impl Into<Vec2f>, tiled: &tiled::Map) -> Vec2i {
    let w = w.into();
    let x = (w.x as u32 + tiled.tile_width - 1) / tiled.tile_width;
    let y = (w.y as u32 + tiled.tile_width - 1) / tiled.tile_height;
    Vec2i::new(x as i32, y as i32)
}

/// World coordinates to tile coordinates flooring remaning pixels
pub fn w2t_floor(w: impl Into<Vec2f>, tiled: &tiled::Map) -> Vec2i {
    let w = w.into();
    let x = w.x as u32 / tiled.tile_width;
    let y = w.y as u32 / tiled.tile_height;
    Vec2i::new(x as i32, y as i32)
}

fn grid_bounds_from_pixel_bounds(map: &tiled::Map, bounds: impl Into<Rect2f>) -> Rect2i {
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

    let size = [
        (right_down.x - left_up.x) as u32,
        (right_down.y - left_up.y) as u32,
    ];

    Rect2i::new(left_up, size)
}

// --------------------------------------------------------------------------------
// Rendering

/// Renders a tiled map in a bounds in world coordinates
pub fn render_tiled(
    dcx: &mut DrawContext,
    tiled: &tiled::Map,
    texture: &TextureData2d,
    px_bounds: impl Into<Rect2f>,
) {
    let px_bounds: Rect2f = px_bounds.into();
    let grid_bounds = self::grid_bounds_from_pixel_bounds(tiled, px_bounds.clone());

    let mut pass = dcx.pass();
    for layer in tiled.layers.iter().filter(|l| l.visible) {
        render_layer(&mut pass, tiled, layer, texture, &px_bounds, &grid_bounds);
    }
}

pub fn render_layer(
    pass: &mut BatchPass<'_>,
    tiled: &tiled::Map,
    layer: &tiled::Layer,
    texture: &TextureData2d,
    px_bounds: &Rect2f,
    grid_bounds: &Rect2i,
) {
    let tile_size = Vec2u::new(tiled.tile_width, tiled.tile_height);
    let tiles = match layer.tiles {
        LayerData::Finite(ref f) => f,
        LayerData::Infinite(_) => unimplemented!("tiled map infinite layer"),
    };

    self::walk_visible_cells_with(grid_bounds, |x, y| {
        let tile = tiles[y][x];
        if tile.gid == 0 {
            return;
        }

        // TODO: handle multiple tilesets
        let tileset = tiled.get_tileset_by_gid(tile.gid).unwrap();
        let id = tile.gid - tileset.first_gid;

        // get uv rect (another approach is to calculate them when loading tiled maps)
        let n_cols = tileset.images[0].width as u32 / tileset.tile_width;
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
                    (x as i32 * tile_size.x as i32 - px_bounds.left_up().x as i32) as f32,
                    (y as i32 * tile_size.y as i32 - px_bounds.left_up().y as i32) as f32,
                ),
                (tile_size.x as f32, tile_size.y as f32),
            ]);
    });
}

pub fn render_fov_shadows(
    pass: &mut BatchPass<'_>,
    tiled: &tiled::Map,
    fov: &FovData,
    px_bounds: &Rect2f,
) {
    let tile_size = Vec2u::new(tiled.tile_width, tiled.tile_height);
    let grid_bounds = self::grid_bounds_from_pixel_bounds(tiled, px_bounds.clone());

    self::walk_visible_cells_with(&grid_bounds, |x, y| {
        // FIXME: why is this semi-transparent
        let color = if fov.is_in_view([x as i32, y as i32].into()) {
            let len = (Vec2i::new(x as i32, y as i32) - fov.origin()).len_f32();
            let x = (len as f32 / fov.radius() as f32).sin();
            Color::rgba(0, 0, 0, 255).multiply(ease(x) * 0.5)
        } else {
            Color::rgba(0, 0, 0, 255).multiply(0.7)
        };

        pass.white_dot()
            .color(color)
            .src_rect_uv([0.0, 0.0, 1.0, 1.0])
            .dest_rect_px([
                (
                    (x as i32 * tile_size.x as i32 - px_bounds.left_up().x as i32) as f32,
                    (y as i32 * tile_size.y as i32 - px_bounds.left_up().y as i32) as f32,
                ),
                (tile_size.x as f32, tile_size.y as f32),
            ]);
    });

    /// x: [0.0, 1.0]
    fn ease(x: f32) -> f32 {
        if x < 0.5 {
            4.0 * x * x * x
        } else {
            1.0 - (-2.0 * x as f32 + 2.0).powf(3.0) / 2.0
        }
    }
}

// --------------------------------------------------------------------------------
// Debug rendering

pub fn render_non_blocking_grids(
    pass: &mut BatchPass<'_>,
    tiled: &tiled::Map,
    blocks: &[bool],
    px_bounds: &Rect2f,
) {
    let grid_size = Vec2u::new(tiled.width, tiled.height);
    let tile_size = Vec2u::new(tiled.tile_width, tiled.tile_height);
    let grid_bounds = self::grid_bounds_from_pixel_bounds(tiled, px_bounds.clone());

    self::walk_visible_cells_with(&grid_bounds, |x, y| {
        let ix = x + y * grid_size.x as usize;
        if blocks[ix] {
            return;
        }

        let pos = Vec2f::new(
            (x as i32 * tile_size.x as i32 - px_bounds.left_up().x as i32) as f32,
            (y as i32 * tile_size.y as i32 - px_bounds.left_up().y as i32) as f32,
        );

        pass.rect(
            [pos + Vec2f::new(2.0, 2.0), (28.0, 28.0).into()],
            Color::white().multiply(0.5),
        );
    });
}

// --------------------------------------------------------------------------------
// Internal utilities

/// Iterates through tiled cells in view
///
/// # Warning
///
/// Closures may be inefficient
fn walk_visible_cells_with(grid_bounds: &Rect2i, mut f: impl FnMut(usize, usize)) {
    let left_up = grid_bounds.left_up();
    let right_down = grid_bounds.right_down();

    for y in (left_up.y as usize)..(right_down.y as usize) {
        for x in (left_up.x as usize)..(right_down.x as usize) {
            f(x, y);
        }
    }
}
