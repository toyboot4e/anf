//! Roguelike map with Tiled

use std::{collections::HashMap, fs, path::Path};

use anf::{engine::draw::*, gfx::prelude::*};

use crate::{
    render::tiled_render,
    rl::{
        fov::OpacityMap,
        grid2d::{Dir8, Vec2i},
    },
    utils::anim::{LoopMode, SpriteAnimPattern},
};

/// Roguelike map data
pub struct RlMap {
    pub size: [usize; 2],
    /// True if collidges
    pub blocks: Vec<bool>,
}

impl RlMap {
    pub fn contains(&self, pos: impl Into<Vec2i>) -> bool {
        let pos = pos.into();
        let (x, y) = (pos.x, pos.y);
        !(x < 0 || y < 0 || self.size[0] as i32 <= x || self.size[1] as i32 <= y)
    }

    /// Returns if the position is blocked or outsize of the map
    pub fn is_blocked(&self, pos: impl Into<Vec2i>) -> bool {
        let pos = pos.into();
        if !self.contains(pos) {
            return true;
        }

        let ix = pos.x + self.size[0] as i32 * pos.y;
        self.blocks[ix as usize]
    }
}

impl RlMap {
    pub fn from_tiled(tiled: &tiled::Map) -> Self {
        let collision = tiled
            .layers
            .iter()
            .find(|l| l.name == "collision")
            .expect("layer with name `collision` is required");

        let tiles = match &collision.tiles {
            tiled::LayerData::Finite(f) => f,
            tiled::LayerData::Infinite(_) => unimplemented!("tiled map infinite layer"),
        };

        // extract collision data from tiled map data
        let mut collision = Vec::with_capacity((tiled.width * tiled.height) as usize);
        for (_y, row) in tiles.iter().enumerate() {
            for (_x, tile) in row.iter().enumerate() {
                collision.push(tile.gid != 0);
            }
        }

        Self {
            size: [tiled.width as usize, tiled.height as usize],
            blocks: collision,
        }
    }
}

impl OpacityMap for RlMap {
    fn is_opaque(&self, pos: Vec2i) -> bool {
        !self.contains(pos) || self.is_blocked(pos)
    }

    fn contains(&self, pos: Vec2i) -> bool {
        <Self>::contains(self, pos)
    }
}

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
