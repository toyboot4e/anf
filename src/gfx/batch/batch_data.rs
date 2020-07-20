//! `BatchData` accumulates vertex and texture data before buffering to GPU
//!
//! Actually the internal implementation is based on `Batcher` in Nez

use crate::gfx::{
    batch::batch_internals::{VertexData, MAX_SPRITES},
    texture::Texture2D,
};

/// The actual vertex data per rectangle sprite
type FourVertexInfo = [VertexData; 4];

impl crate::gfx::vertices::SomeVertexData for FourVertexInfo {}

/// Local data before buffering to GPU
///
/// Each info is indexed with sprite push (first, second, third, ..). Each batch can be iterated
/// via [`BatchSpanIter`].
///
/// * `vertex_data`:
///   the actual vertex data to be set to `VertexBuffer`
/// * `texture_info`:
///   each texture in it corresponds to each batch
/// * `n_sprites`:
///   the number of sprites accumulated in this data
///
/// [`BatchSpanIter`]: ./struct.BatchSpanIter.html
#[derive(Debug)]
pub struct BatchData {
    pub vertex_data: Vec<self::FourVertexInfo>,
    // TODO: use Rc
    pub texture_info: Vec<Texture2D>,
    pub n_sprites: usize,
}

impl BatchData {
    pub fn new() -> Self {
        let v = vec![self::FourVertexInfo::default(); MAX_SPRITES];
        // FIXME: what size should I use?
        let t = vec![Texture2D::empty(); 16];

        Self {
            vertex_data: v,
            texture_info: t,
            n_sprites: 0,
        }
    }
}

/// Slices `BatchData` into `BatchSpan`s each of which corresponds to a draw call
///
/// Make sure to clear `BatchData::n_sprites` maually after making draw calls.
///
/// ```no_run
/// // batcher: &mut Batcher in scope
/// let iter = BatchSpanIter::new();
/// while let Some(span) = iter.next(&batcher.batch_data) {
///     // make a draw call
/// }
/// batcher.batch_data.n_sprites = 0;
/// ```
pub struct BatchSpanIter {
    current: usize,
}

/// Span of `BatchData` for a draw call
pub struct BatchSpan {
    pub offset: usize,
    pub len: usize,
}

impl BatchSpanIter {
    pub fn new() -> Self {
        Self { current: 0 }
    }

    pub fn next(&mut self, batch: &BatchData) -> Option<BatchSpan> {
        if self.current >= batch.n_sprites {
            return None;
        }
        let lo = self.current;
        for hi in 1..batch.n_sprites {
            if &batch.texture_info[hi] != &batch.texture_info[lo] {
                self.current = hi;
                return Some(BatchSpan {
                    offset: lo,
                    len: hi - lo,
                });
            }
        }
        let hi = batch.n_sprites;
        self.current = hi;
        return Some(BatchSpan {
            offset: lo,
            len: hi - lo,
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::mem::size_of;
    #[test]
    fn test_size() {
        assert_eq!(size_of::<VertexData>(), 24);
        assert_eq!(size_of::<FourVertexInfo>(), 96);
    }
}
