//! [`SpriteBatch`] and iterator of it

use crate::batcher::bufspecs::{QuadData, MAX_QUADS};

/// Quads with textures tracked
///
/// Sprites are technically textured quadliterals.
#[derive(Debug)]
pub struct SpriteBatch {
    quads: Vec<QuadData>,
    // TODO: use run-length encoding
    raw_texture_track: Vec<*mut fna3d::Texture>,
    n_quads: usize,
}

impl SpriteBatch {
    pub fn new() -> Self {
        let v = vec![QuadData::default(); MAX_QUADS];
        let t = vec![std::ptr::null_mut(); MAX_QUADS];

        SpriteBatch {
            quads: v,
            raw_texture_track: t,
            n_quads: 0,
        }
    }
}

/// For quad push
impl SpriteBatch {
    /// Flush batcher if [`SpriteBatch`] is satured
    pub fn is_satured(&self) -> bool {
        self.quads.len() - 1 <= self.n_quads
    }

    /// Make sure the [`SpriteBatch`] is not satured before calling this method
    pub unsafe fn next_quad_mut(&mut self, texture: *mut fna3d::Texture) -> &mut QuadData {
        self.raw_texture_track[self.n_quads] = texture;
        let quad = &mut self.quads[self.n_quads];
        self.n_quads += 1;
        quad
    }

    pub fn any_quads_pushed(&self) -> bool {
        self.n_quads > 0
    }

    /// Iterator of draw calls
    pub fn iter(&self) -> SpriteDrawCallIter<'_> {
        SpriteDrawCallIter {
            batch: self,
            current: 0,
            quad_count: 0,
        }
    }

    /// Client vertices to upload to GPU
    pub fn quads_mut(&mut self) -> &mut [QuadData] {
        &mut self.quads[0..self.n_quads]
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
pub struct SpriteDrawCallIter<'a> {
    batch: &'a SpriteBatch,
    current: usize,
    quad_count: usize,
}

impl<'a> Iterator for SpriteDrawCallIter<'a> {
    type Item = SpriteDrawCall;

    fn next(&mut self) -> Option<SpriteDrawCall> {
        if self.current >= self.batch.n_quads {
            return None;
        }

        self.quad_count += 1; // current quad count is `self.quad_count - 1`

        let lo = self.current;
        for hi in (self.current + 1)..self.batch.n_quads {
            if &self.batch.raw_texture_track[hi] == &self.batch.raw_texture_track[lo] {
                continue; // batch the quad
            }

            // we found different texture. make a draw call
            self.current = hi;
            return Some(SpriteDrawCall {
                span: BatchSpan { lo, hi },
                texture: self.batch.raw_texture_track[lo],
            });
        }

        // finally make a draw call
        let hi = self.batch.n_quads;
        self.current = hi;
        return Some(SpriteDrawCall {
            span: BatchSpan { lo, hi },
            texture: self.batch.raw_texture_track[lo],
        });
    }
}

/// Smart span of [`SpriteBatch`] to make a draw call
#[derive(Debug)]
pub struct SpriteDrawCall {
    span: BatchSpan,
    texture: *mut fna3d::Texture,
}

impl SpriteDrawCall {
    pub fn texture(&self) -> *mut fna3d::Texture {
        self.texture
    }

    pub fn base_vertex(&self) -> usize {
        self.span.lo * 4 // each quad has four vertices
    }

    pub fn base_index(&self) -> usize {
        self.span.lo * 6
    }

    pub fn n_primitives(&self) -> usize {
        self.span.len() * 2
    }
}

/// [`lo`, `hi`) span of quadliterals in [`SpriteBatch`] for making a draw call
///
/// Note that `lo` multipled by 2 is the base vertex index because we're counting quadliterals.
#[derive(Debug)]
struct BatchSpan {
    /// low (inclusive)
    pub lo: usize,
    /// high (exclusive)
    pub hi: usize,
}

impl BatchSpan {
    /// Corresponds to the number of sprites to draw
    ///
    /// `len` multipled by 2 is the number of triangles
    pub fn len(&self) -> usize {
        self.hi - self.lo
    }
}
