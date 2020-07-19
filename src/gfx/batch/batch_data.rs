//! `BatchData` that accumulates vertex and texture data before buffering to GPU
//!
//! Actually the internal implementation is based on `Batcher` in Nez

use crate::gfx::{
    batch::batch_internals::{VertexData, MAX_SPRITES},
    texture::Texture2D,
};

/// The actual vertex data per rectangle sprite
type FourVertexInfo = [VertexData; 4];

impl crate::gfx::vertices::AnyVertexData for FourVertexInfo {}

/// Local data before buffering to GPU
///
/// Each info is indexed with sprite push (first, second, third, ..).
///
/// * `vertex_data`:
///   the actual vertex data to be set to `*fna3d::Buffer`
/// * `texture_info`:
///   each texture in it correspond to each batch
/// * `n_sprites`:
///   number of sprites accumulated
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

/// Slices `BatchData` based on `texture_info` field
///
/// Make sure to clear `BatchData::n_sprites` maually
pub struct BatchSpanIter {
    current: usize,
}

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

// // TODO: should I extract `flush` to `draw` module
// impl BatchData {
//     /// Actually draws all the pushed primitives applying vertex buffer bindings
//     pub fn flush(
//         &mut self,
//         device: &mut fna3d::Device,
//         ibuf: &IndexBuffer,
//         binds: &mut GpuBindings,
//         state: &mut GlState,
//     ) {
//         let mut current = 0;
//         for i in 1..self.n_sprites {
//             if &self.texture_info[i] != &self.texture_info[current] {
//                 // TODO: set texture
//                 draw::draw_indexed_primitives(
//                     device,
//                     &ibuf,
//                     binds,
//                     &self.texture_info[i],
//                     state,
//                     current as u32,
//                     (i - current) as u32,
//                 );
//                 current = i;
//             }
//         }

//         log::trace!(
//             "draw texture {}, {:?} at {:#?}",
//             self.n_sprites,
//             &self.texture_info[current],
//             &self.vertex_data[current..(current + self.n_sprites)]
//         );

//         // TODO: how to set texture (maybe VertexBufferBinding?)
//         draw::draw_indexed_primitives(
//             device,
//             &ibuf,
//             binds,
//             &self.texture_info[current],
//             state,
//             current as u32,
//             (self.n_sprites - current) as u32,
//         );

//         self.n_sprites = 0;
//     }
// }
