//! Bundlers

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use anf::{engine::draw::*, gfx::prelude::*};

use crate::{
    render::tiled_render,
    rl::map::RlMap,
    utils::{
        anim::{LoopMode, SpriteAnimPattern, SpriteAnimState},
        grid2d::{Dir4, Dir8, Rect2i, Vec2i},
    },
};

/// Bundles tiled map and roguelike map data
pub struct TiledRlMap {
    pub tiled: tiled::Map,
    pub rlmap: RlMap,
    // TODO: support multiple textures
    texture: TextureData2d,
}

impl TiledRlMap {
    pub fn from_tiled_path(path: &Path, device: &mut fna3d::Device) -> Self {
        let file = fs::File::open(path).unwrap();

        let tiled = tiled::parse_with_path(file, &path).unwrap_or_else(|_err| {
            panic!(
                "error opening tiled map file: `{}` does it exist",
                path.display()
            )
        });
        let rlmap = RlMap::from_tiled(&tiled);

        let texture = {
            // tile image is relative to the tmx file directory
            let tmx_directory = path.parent().unwrap();
            let img_path = tmx_directory.join(&tiled.tilesets[0].images[0].source);
            TextureData2d::from_path(device, img_path).unwrap()
        };

        Self {
            tiled,
            rlmap,
            texture,
        }
    }

    pub fn render(&mut self, dcx: &mut DrawContext, px_bounds: impl Into<Rect2f>) {
        tiled_render::render_tiled(dcx, &self.tiled, &self.texture, px_bounds);
    }
}

/// Generates character walking animation from 4x3 character image
pub fn gen_anim4(texture: &TextureData2d, fps: f32) -> HashMap<Dir8, SpriteAnimPattern> {
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
