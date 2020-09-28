//! Internals of orthogonal tilemap

/// Roguelike map data
pub struct RlMap {
    size: [usize; 2],
    /// True if collidges
    collision: Vec<bool>,
}

impl RlMap {
    pub fn size(&self) -> [usize; 2] {
        self.size
    }

    pub fn contains(&self, x: usize, y: usize) -> bool {
        !(x < 0 || y < 0 || self.size[0] <= x || self.size[1] <= y)
    }

    pub fn is_blocked(&self, x: usize, y: usize) -> bool {
        assert!(self.contains(x, y));

        let ix = x + self.size[1] * y;
        self.collision[ix]
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

        let mut layer = Vec::with_capacity((tiled.width * tiled.height) as usize);
        for (_y, row) in tiles.iter().enumerate() {
            for (_x, tile) in row.iter().enumerate() {
                layer.push(tile.gid != 0);
            }
        }

        Self {
            size: [tiled.width as usize, tiled.height as usize],
            collision: layer,
        }
    }
}
