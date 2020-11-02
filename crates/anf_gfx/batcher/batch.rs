//! [`SpriteBatch`] and iterator of it

use crate::batcher::bufspecs::{QuadData, MAX_QUADS};

/// Quads with textures tracked
///
/// Sprites are technically textured quadliterals.
#[derive(Debug)]
pub struct SpriteBatch {
    quads: Vec<QuadData>,
    // TODO: use run-length encoding
    track: Vec<*mut fna3d::Texture>,
    n_quads: usize,
}

impl SpriteBatch {
    pub fn new() -> Self {
        let v = vec![QuadData::default(); MAX_QUADS];
        let t = vec![std::ptr::null_mut(); MAX_QUADS];

        SpriteBatch {
            quads: v,
            track: t,
            n_quads: 0,
        }
    }
}

impl SpriteBatch {
    /// Flush batcher if [`SpriteBatch`] is satured
    pub fn is_satured(&self) -> bool {
        self.quads.len() - 1 <= self.n_quads
    }

    /// Make sure the [`SpriteBatch`] is not satured before calling this method
    pub unsafe fn next_quad_mut(&mut self, texture: *mut fna3d::Texture) -> &mut QuadData {
        self.track[self.n_quads] = texture;
        let quad = &mut self.quads[self.n_quads];
        self.n_quads += 1;
        quad
    }

    pub fn any_quads_pushed(&self) -> bool {
        self.n_quads > 0
    }

    /// Iterator of draw calls
    pub fn iter(&self) -> DrawCallIter<'_> {
        DrawCallIter::from_batch(self)
    }

    /// Client vertices to upload to GPU
    pub fn pushed_quads(&self) -> &[QuadData] {
        &self.quads[0..self.n_quads]
    }

    /// Called after flushing
    pub fn clear(&mut self) {
        self.n_quads = 0;
    }
}

// --------------------------------------------------------------------------------
// Drawcall iterator

/// Slices [`SpriteBatch`] into [`SpriteDrawCall`]
#[derive(Debug)]
pub struct DrawCallIter<'a> {
    batch: &'a SpriteBatch,
    /// Next quad index
    ix: usize,
}

impl<'a> DrawCallIter<'a> {
    pub fn from_batch(batch: &'a SpriteBatch) -> Self {
        Self { batch, ix: 0 }
    }
}

impl<'a> Iterator for DrawCallIter<'a> {
    type Item = DrawCall;

    fn next(&mut self) -> Option<DrawCall> {
        if self.ix >= self.batch.n_quads {
            return None;
        }

        let lo = self.ix;
        for hi in (self.ix + 1)..self.batch.n_quads {
            if &self.batch.track[hi] == &self.batch.track[lo] {
                continue; // batch the quad
            }

            self.ix = hi;
            return Some(DrawCall {
                tex: self.batch.track[lo],
                lo,
                hi,
            });
        }

        let hi = self.batch.n_quads;
        self.ix = hi;
        return Some(DrawCall {
            tex: self.batch.track[lo],
            lo,
            hi,
        });
    }
}

/// Smart span of [`SpriteBatch`] that corresponds to a draw call
#[derive(Debug)]
pub struct DrawCall {
    pub tex: *mut fna3d::Texture,
    /// low (inclusive)
    pub lo: usize,
    /// high (exclusive)
    pub hi: usize,
}

impl DrawCall {
    pub fn n_quads(&self) -> usize {
        self.hi - self.lo
    }

    pub fn base_vtx(&self) -> usize {
        self.lo * 4 // each quad has four vertices
    }

    pub fn base_idx(&self) -> usize {
        self.lo * 6
    }

    pub fn n_triangles(&self) -> usize {
        self.n_quads() * 2
    }

    pub fn n_verts(&self) -> usize {
        self.n_quads() * 4
    }
}
