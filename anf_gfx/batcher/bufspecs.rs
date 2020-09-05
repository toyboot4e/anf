//! `BatchData` specification on vertex/index buffer

use crate::{
    batcher::primitives::*,
    buffers::{DynamicVertexBuffer, IndexBuffer},
};
use anf_deps::fna3d;

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
impl crate::buffers::VertexData for QuadData {}
impl crate::buffers::VertexData for ColoredVertexData {}

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
            vertexStride: 24, // FIXME: is this right
            elementCount: 3,
            elements: Self::elems().as_ptr() as *mut _,
        }
    }
}

/// Vertex/index buffer based on the `bufspecs` types and constants
///
/// # Immutability of `IndexBuffer`
///
/// Our `IndexBuffer` is only for drawing quadliterals and it won't be modified after this
/// initialization:
///
/// ```
/// use anf_gfx::batcher::bufspecs::{MAX_INDICES, MAX_QUADS};
///
/// let mut indices = [0; MAX_INDICES];
/// // for each quadliteral, we need two triangles (i.e. four verices and six indices)
/// for n in 0..MAX_QUADS as i16 {
///     let (i, v) = (n * 6, n * 4);
///     indices[i as usize] = v as i16;
///     indices[(i + 1) as usize] = v + 1 as i16;
///     indices[(i + 2) as usize] = v + 2 as i16;
///     indices[(i + 3) as usize] = v + 3 as i16;
///     indices[(i + 4) as usize] = v + 2 as i16;
///     indices[(i + 5) as usize] = v + 1 as i16;
/// }
/// ```
#[derive(Debug)]
pub struct ViBuffer {
    pub vbuf: DynamicVertexBuffer,
    pub ibuf: IndexBuffer,
}

impl ViBuffer {
    pub fn from_device(device: &mut fna3d::Device) -> Self {
        // let mut device = fna3d::Device::from_params(&mut params, true);
        // device.reset_backbuffer(&mut params);

        let vbuf = DynamicVertexBuffer::new(
            device,
            ColoredVertexData::decl(),
            self::MAX_VERTICES as u32,
            fna3d::BufferUsage::WriteOnly,
        );

        let mut ibuf = IndexBuffer::new(
            device,
            self::INDEX_ELEM_SIZE,
            self::MAX_INDICES as u32,
            fna3d::BufferUsage::WriteOnly, // what is this
            false,
        );

        ibuf.set_data(device, 0, &Self::gen_index_array());

        ViBuffer { vbuf, ibuf }
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
