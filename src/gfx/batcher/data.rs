//! `BatchData` and iterator of it
//!
//! Presudo example:
//!
//! ```
//! use anf::gfx::batcher::data::{BatchData, BatchSpan, BatchSpanIter};
//!
//! pub struct YourContext { /* your data */ }
//! fn make_draw_call(cx: &mut YourContext, batch: &BatchData, slot: usize, span: BatchSpan) {
//!     /* .. */
//! }
//!
//! fn flush_batch(cx: &mut YourContext, batch: &mut BatchData) {
//!      let mut iter = BatchSpanIter::new();
//!      while let Some((slot, span)) = iter.next(&batch) {
//!          make_draw_call(cx, batch, slot, span);
//!      }
//! }
//! ```
//!
//! Not so elegant but enough for internals
//!
//! [`BatchSpanIter`]: ./struct.BatchSpanIter.html

use crate::gfx::{
    batcher::bufspecs::{QuadData, MAX_QUADS},
    texture::Texture2D,
};

/// Accumulates vertex data tracking what `Texture2D` are used for each
#[derive(Debug)]
pub struct BatchData {
    /// The actual vertex data to be set to `VertexBuffer`
    pub vertex_data: Vec<QuadData>,
    /// Each texture corresponds to each quad (NOT each batch)
    ///
    /// TODO: use Rc?
    pub texture_track: Vec<Texture2D>,
    pub n_quads: usize,
}

impl BatchData {
    pub fn new() -> Self {
        let v = vec![QuadData::default(); MAX_QUADS];
        // FIXME: use max texture slot?
        let t = vec![Texture2D::empty(); MAX_QUADS];

        Self {
            vertex_data: v,
            texture_track: t,
            n_quads: 0,
        }
    }
}

/// Slices `BatchData` to `BatchSpan`s, each of which corresponds to a draw call
///
/// Make sure to clear `BatchData::n_quads` maually after making draw calls.
///
/// ```
/// use anf::gfx::batcher::{Batcher, data::BatchSpanIter};
///
/// fn flush_batcher(batcher: &mut Batcher) {
///      let mut iter = BatchSpanIter::new();
///      while let Some((slot, span)) = iter.next(&batcher.batch) {
///          // make a draw call
///      }
///      batcher.batch.n_quads = 0;
/// }
/// ```
#[derive(Debug)]
pub struct BatchSpanIter {
    current: usize,
    nth: usize,
}

/// [`lo`, `hi`) span of quadliterals in `BatchData` for making a draw call
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
        Self { current: 0, nth: 0 }
    }

    /// Returns the texture slot and a range of vertices
    pub fn next(&mut self, batch: &BatchData) -> Option<(usize, BatchSpan)> {
        if self.current >= batch.n_quads {
            return None;
        }
        let nth = self.nth; // this it NOT actually texture slots. TODO: run-length encoding
        self.nth += 1;
        let lo = self.current;
        for hi in 1..batch.n_quads {
            if &batch.texture_track[hi] != &batch.texture_track[lo] {
                self.current = hi;
                Some((lo, BatchSpan { lo, hi }));
            }
        }
        let hi = batch.n_quads;
        self.current = hi;
        Some((lo, BatchSpan { lo, hi }))
    }
}
