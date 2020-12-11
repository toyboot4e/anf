//! Specification of vertex/index buffer used by [`SpriteBatch`]
//!
//! [`SpriteBatch`]: crate::batcher::batch::SpriteBatch

use crate::{geom2d::*, geom3d::Vec3f};

use fna3d_hie::buf::{GpuDynamicVertexBuffer, GpuIndexBuffer};

use fna3h::{
    buf::{
        BufferUsage, IndexElementSize, VertexDeclaration, VertexElement, VertexElementFormat,
        VertexElementUsage,
    },
    tex::Texture,
    Color, Device,
};

// --------------------------------------------------------------------------------
// Constants

/// We use 16 bits for vertex index
pub const INDEX_ELEM_SIZE: IndexElementSize = IndexElementSize::Bits16;

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
    /// Vertex position
    pub color: Color,
    /// Normalized source position in texture (also known as texels or texture coordinates)
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
impl fna3d_hie::buf::VertexData for ColoredVertexData {}
impl fna3d_hie::buf::VertexData for QuadData {}

impl Default for ColoredVertexData {
    fn default() -> Self {
        let color = Color::rgba(0, 0, 0, 0);
        Self {
            dest: Vec3f::default(),
            color,
            uvs: Vec2f::default(),
        }
    }
}

impl ColoredVertexData {
    const ELEMS: &'static [VertexElement; 3] = &[
        VertexElement {
            offset: 0,
            vertexElementFormat: VertexElementFormat::Vector3 as u32,
            vertexElementUsage: VertexElementUsage::Position as u32,
            usageIndex: 0, // TODO: what's this
        },
        VertexElement {
            offset: 12,
            vertexElementFormat: VertexElementFormat::Color as u32,
            vertexElementUsage: VertexElementUsage::Color as u32,
            usageIndex: 0,
        },
        VertexElement {
            offset: 16,
            vertexElementFormat: VertexElementFormat::Vector2 as u32,
            vertexElementUsage: VertexElementUsage::TextureCoordinate as u32,
            usageIndex: 0,
        },
    ];

    pub fn decl() -> VertexDeclaration {
        VertexDeclaration {
            vertexStride: 24,
            elementCount: 3,
            elements: Self::ELEMS as *const _ as *mut _,
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
    pub fn from_device(device: &Device) -> Self {
        let vbuf = GpuDynamicVertexBuffer::new(
            device,
            ColoredVertexData::decl(),
            self::MAX_VERTICES as u32,
            BufferUsage::WriteOnly,
        );

        let mut ibuf = GpuIndexBuffer::new(
            device,
            self::INDEX_ELEM_SIZE,
            self::MAX_INDICES as u32,
            BufferUsage::WriteOnly, // what is this
            false,
        );

        let indices = fna3d_hie::gen_quad_indices!(self::MAX_QUADS);
        ibuf.upload_indices(device, 0, &indices);

        GpuViBuffer { vbuf, ibuf }
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
