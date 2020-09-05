//! [`SpriteBatch`] and iterator of it
//!
//! Presudo example:
//!
//! ```
//! use anf_gfx::batcher::batch::{SpriteBatch, BatchSpan, BatchSpanIter};
//!
//! pub struct YourContext { /* your data */ }
//!
//! fn flush_batch(cx: &mut YourContext, batch: &mut SpriteBatch) {
//!      let mut iter = BatchSpanIter::new();
//!      while let Some(span) = iter.next(&batch) {
//!          let texture = &batch.texture_track[span.lo];
//!          /* make a draw call with your context */
//!      }
//! }
//! ```
//!
//! Not so elegant but enough for internals

use crate::{
    batcher::bufspecs::{QuadData, MAX_QUADS},
    texture::Texture2D,
};

/// Sprite batch data
///
/// Sprites are technically textured quadliterals. `SpriteBatch` accumulates vertex data tracking
/// what [`Texture2D`] are used for each. So this is the data for sprite batching.
#[derive(Debug)]
pub struct SpriteBatch {
    /// The actual vertex data to be uploaded via [`VertexBufferData`]
    pub vertex_data: Vec<QuadData>,
    /// Each texture corresponds to each quad (NOT each batch)
    ///
    /// TODO: use Rc?
    pub texture_track: Vec<Texture2D>,
    pub n_quads: usize,
}

impl SpriteBatch {
    pub fn new() -> Self {
        let v = vec![QuadData::default(); MAX_QUADS];
        // FIXME: use max texture slot?
        let t = vec![Texture2D::empty(); MAX_QUADS];

        SpriteBatch {
            vertex_data: v,
            texture_track: t,
            n_quads: 0,
        }
    }
}

/// Slices [`SpriteBatch`] to [`BatchSpan`]s, each of which corresponds to a draw call
///
/// Make sure to clear [`SpriteBatch::n_quads`] maually after making draw calls.
///
/// ```
/// use anf_gfx::batcher::{Batcher, batch::BatchSpanIter};
///
/// fn flush_batcher(batcher: &mut Batcher) {
///      let mut iter = BatchSpanIter::new();
///      while let Some(span) = iter.next(&batcher.batch) {
///          // make a draw call
///      }
///      batcher.batch.n_quads = 0;
/// }
/// ```
#[derive(Debug)]
pub struct BatchSpanIter {
    current: usize,
    quad_count: usize,
}

/// [`lo`, `hi`) span of quadliterals in [`SpriteBatch`] for making a draw call
///
/// Note that `lo` multipled by 2 is the base vertex index because we're counting quadliterals.
#[derive(Debug)]
pub struct BatchSpan {
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

impl BatchSpanIter {
    pub fn new() -> Self {
        Self {
            current: 0,
            quad_count: 0,
        }
    }

    /// Returns the texture slot and a range of vertices
    pub fn next(&mut self, batch: &SpriteBatch) -> Option<BatchSpan> {
        if self.current >= batch.n_quads {
            return None;
        }

        self.quad_count += 1; // current quad count is `self.quad_count - 1`

        let lo = self.current;
        for hi in 1..batch.n_quads {
            if &batch.texture_track[hi] != &batch.texture_track[lo] {
                self.current = hi;
                return Some(BatchSpan { lo, hi });
            }
        }

        let hi = batch.n_quads;
        self.current = hi;
        Some(BatchSpan { lo, hi })
    }
}
