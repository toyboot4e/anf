//! `BatchData` specification

use crate::gfx::batcher::primitives::*;

// --------------------------------------------------------------------------------
// Constants

/// We use 16 bits for vertex index
pub const INDEX_ELEM_SIZE: fna3d::IndexElementSize = fna3d::IndexElementSize::Bits16;

/// 2048
pub const MAX_SPRITES: usize = 2048;
/// 2048 * 4
pub const MAX_VERTICES: usize = MAX_SPRITES * 4;
/// 2048 * 4 * 6 = 49152 < 65536 = 2^16
pub const MAX_INDICES: usize = MAX_SPRITES * 6;

// --------------------------------------------------------------------------------
// Vertex types

/// The actual vertex data type
#[derive(Debug, Clone)]
#[repr(C)]
pub struct ColoredVertexData {
    /// Destination position in pixels
    ///
    /// * TODO: isn't it normalized?
    pub dest: Vec3f, // TODO: use 2D dest vec
    pub color: fna3d::Color,
    /// Normalized source position in texture (a.k.a. texture coordinates or texels)
    pub uvs: Vec2f,
}

/// The actual quadliteral data type
pub type QuadData = [ColoredVertexData; 4];

// mark them as data that can be set to vertex buffer in GPU memory
impl crate::gfx::buffers::VertexData for QuadData {}
impl crate::gfx::buffers::VertexData for ColoredVertexData {}

impl Default for ColoredVertexData {
    fn default() -> Self {
        let color = fna3d::Color::rgba(0, 0, 0, 0);
        Self {
            dest: Vec3f::default(),
            color,
            uvs: Vec2f::default(),
            // ..Default::default() // TODO: why does it result in recursive call
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
            vertexStride: 24, // FIXME: is this right
            elementCount: 3,
            elements: Self::elems().as_ptr() as *mut _,
        }
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
