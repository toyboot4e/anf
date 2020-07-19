//! `BatchData` that accumulates vertex and texture data before buffering to GPU
//!
//! Actually the internal implementation is based on `Batcher` in Nez

use crate::{
    batch::{
        batch_internals::{VertexData, MAX_SPRITES},
        draw::{self, GlState, GpuBindings},
    },
    gfx::{texture::Texture2D, vertices::IndexBuffer},
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

// TODO: should I extract `flush` to `draw` module
impl BatchData {
    /// Actually draws all the pushed primitives
    pub fn flush(
        &mut self,
        device: &mut fna3d::Device,
        ibuf: &IndexBuffer,
        binds: &mut GpuBindings,
        state: &mut GlState,
    ) {
        let mut current = 0;
        for i in 1..self.n_sprites {
            if &self.texture_info[i] != &self.texture_info[current] {
                // TODO: set texture
                draw::draw_indexed_primitives(
                    device,
                    &ibuf,
                    binds,
                    &self.texture_info[i],
                    state,
                    current as u32,
                    (i - current) as u32,
                );
                current = i;
            }
        }

        log::trace!(
            "draw texture {}, {:?} at {:#?}",
            self.n_sprites,
            &self.texture_info[current],
            &self.vertex_data[current..(current + self.n_sprites)]
        );

        // TODO: how to set texture (maybe VertexBufferBinding?)
        draw::draw_indexed_primitives(
            device,
            &ibuf,
            binds,
            &self.texture_info[current],
            state,
            current as u32,
            (self.n_sprites - current) as u32,
        );

        self.n_sprites = 0;
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
