//! Re-exported to the super module

use crate::gfx::{
    batcher::batch_data::batch_internals::{ColoredVertexData, MAX_SPRITES},
    texture::Texture2D,
};

/// The actual vertex data per rectangle (quad)
pub type FourVertexInfo = [ColoredVertexData; 4];

impl crate::gfx::vertices::VertexData for FourVertexInfo {}

/// Accumulates vertex data with related `Texture2D` (s)
///
/// Each info is indexed with sprite push (first, second, third, ..). Each batch can be iterated
/// via [`BatchSpanIter`].
///
/// * `vertex_data`:
///   the actual vertex data to be set to `VertexBuffer`
/// * `texture_slots`:
///   each texture corresponds to each quad (NOT each batch)
/// * `n_quads`:
///   the number of sprites accumulated in this data
///
/// * TODO: refactor to use buffer-length encoding
///
/// [`BatchSpanIter`]: ./struct.BatchSpanIter.html
#[derive(Debug)]
pub struct BatchData {
    pub vertex_data: Vec<self::FourVertexInfo>,
    // TODO: use Rc
    pub texture_slots: Vec<Texture2D>,
    pub n_quads: usize,
}

impl BatchData {
    pub fn new() -> Self {
        let v = vec![self::FourVertexInfo::default(); MAX_SPRITES];
        // FIXME: use max texture slot?
        let t = vec![Texture2D::empty(); MAX_SPRITES];

        Self {
            vertex_data: v,
            texture_slots: t,
            n_quads: 0,
        }
    }
}

/// Slices `BatchData` into `BatchSpan`s each of which corresponds to a draw call
///
/// Make sure to clear `BatchData::n_quads` maually after making draw calls.
///
/// ```no_run
/// // batcher: &mut Batcher in scope
/// let iter = BatchSpanIter::new();
/// while let Some((slot, span)) = iter.next(&batcher.batch_data) {
///     // make a draw call
/// }
/// batcher.batch_data.n_quads = 0;
/// ```
#[derive(Debug)]
pub struct BatchSpanIter {
    current: usize,
    nth: usize,
}

/// Span of `BatchData` for a draw call generated with `BatchSpanIter`
///
/// `lo` multipled by 2 is the base vertex index
#[derive(Debug)]
pub struct BatchSpan {
    pub lo: usize,
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
            if &batch.texture_slots[hi] != &batch.texture_slots[lo] {
                self.current = hi;
                Some((lo, BatchSpan { lo, hi }));
            }
        }
        let hi = batch.n_quads;
        self.current = hi;
        Some((lo, BatchSpan { lo, hi }))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::mem::size_of;
    #[test]
    fn test_size() {
        assert_eq!(size_of::<ColoredVertexData>(), 24);
        assert_eq!(size_of::<FourVertexInfo>(), 96);
    }
}
