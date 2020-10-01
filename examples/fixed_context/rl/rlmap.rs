//! Internals of orthogonal tilemap

use crate::utils::grid2d::Vec2i;

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
            return false;
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
