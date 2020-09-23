//! Specification of vertex/index buffer used by [`SpriteBatch`]
//!
//! [`SpriteBatch`]: crate::batcher::batch::SpriteBatch

use crate::geom2d::*;
use fna3d_hie::buffers::{GpuDynamicVertexBuffer, GpuIndexBuffer};

// --------------------------------------------------------------------------------
// Constants

/// We use 16 bits for vertex index
pub const INDEX_ELEM_SIZE: fna3d::IndexElementSize = fna3d::IndexElementSize::Bits16;

/// 2048
pub const MAX_QUADS: usize = 2048;

/// 2048 * 4
pub const MAX_VERTICES: usize = MAX_QUADS * 4;

/// 2048 * 4 * 6 = 49152 < 65536 = 2^16
pub const MAX_INDICES: usize = MAX_QUADS * 6;

// --------------------------------------------------------------------------------
// Vertex types

/// The actual vertex data type in `anf_gfx::batcher`
#[derive(Debug, Clone)]
#[repr(C)]
pub struct ColoredVertexData {
    /// Destination position in pixels
    pub dest: Vec3f,
    pub color: fna3d::Color,
    /// Normalized source position in texture (also known as texture coordinates or texels)
    pub uvs: Vec2f,
}

/// The actual quadliteral data type in `anf_gfx::batcher`
///
/// This is actually an array of [`ColoredVertexData`], however, we need to wrap it with a newtype
/// struct so that we can implement `QuadData` (because we can't implemenet traits for arrays).
#[derive(Clone, Debug, Default)]
pub struct QuadData([ColoredVertexData; 4]);

impl std::ops::Deref for QuadData {
    type Target = [ColoredVertexData; 4];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for QuadData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// mark them as data that can be set to vertex buffer in GPU memory
impl fna3d_hie::buffers::VertexData for ColoredVertexData {}
impl fna3d_hie::buffers::VertexData for QuadData {}

impl Default for ColoredVertexData {
    fn default() -> Self {
        let color = fna3d::Color::rgba(0, 0, 0, 0);
        Self {
            dest: Vec3f::default(),
            color,
            uvs: Vec2f::default(),
        }
    }
}

impl ColoredVertexData {
    pub fn elems() -> &'static [fna3d::VertexElement] {
        &[
            fna3d::VertexElement {
                offset: 0,
                vertexElementFormat: fna3d::VertexElementFormat::Vector3 as u32,
                vertexElementUsage: fna3d::VertexElementUsage::Position as u32,
                usageIndex: 0, // TODO: what's this
            },
            fna3d::VertexElement {
                offset: 12,
                vertexElementFormat: fna3d::VertexElementFormat::Color as u32,
                vertexElementUsage: fna3d::VertexElementUsage::Color as u32,
                usageIndex: 0,
            },
            fna3d::VertexElement {
                offset: 16,
                vertexElementFormat: fna3d::VertexElementFormat::Vector2 as u32,
                vertexElementUsage: fna3d::VertexElementUsage::TextureCoordinate as u32,
                usageIndex: 0,
            },
        ]
    }

    pub fn decl() -> fna3d::VertexDeclaration {
        fna3d::VertexDeclaration {
            vertexStride: 24,
            elementCount: 3,
            elements: Self::elems().as_ptr() as *mut _,
        }
    }
}

/// GPU vertex/index buffer handle specific for `anf_gfx::batcher`
#[derive(Debug)]
pub struct GpuViBuffer {
    pub vbuf: GpuDynamicVertexBuffer,
    pub ibuf: GpuIndexBuffer,
}

impl GpuViBuffer {
    pub fn from_device(device: &mut fna3d::Device) -> Self {
        let vbuf = GpuDynamicVertexBuffer::new(
            device,
            ColoredVertexData::decl(),
            self::MAX_VERTICES as u32,
            fna3d::BufferUsage::WriteOnly,
        );

        let mut ibuf = GpuIndexBuffer::new(
            device,
            self::INDEX_ELEM_SIZE,
            self::MAX_INDICES as u32,
            fna3d::BufferUsage::WriteOnly, // what is this
            false,
        );

        ibuf.upload_indices(device, 0, &Self::gen_index_array());

        GpuViBuffer { vbuf, ibuf }
    }

    fn gen_index_array() -> [i16; self::MAX_INDICES] {
        let mut indices = [0; self::MAX_INDICES];
        // for each quadliteral, we need two triangles (i.e. four verices and six indices)
        for n in 0..self::MAX_QUADS as i16 {
            let (i, v) = (n * 6, n * 4);
            indices[i as usize] = v as i16;
            indices[(i + 1) as usize] = v + 1 as i16;
            indices[(i + 2) as usize] = v + 2 as i16;
            indices[(i + 3) as usize] = v + 3 as i16;
            indices[(i + 4) as usize] = v + 2 as i16;
            indices[(i + 5) as usize] = v + 1 as i16;
        }
        indices
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::mem::size_of;
    #[test]
    fn test_size() {
        assert_eq!(size_of::<ColoredVertexData>(), 24);
        assert_eq!(size_of::<QuadData>(), 96);
    }
}
