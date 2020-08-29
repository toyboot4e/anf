//! Internal data types and constants in the `batch` module

/// We use 16 bits for vertex index
pub const INDEX_ELEM_SIZE: fna3d::IndexElementSize = fna3d::IndexElementSize::Bits16;

/// 2048
pub const MAX_SPRITES: usize = 2048;
/// 2048 * 4
pub const MAX_VERTICES: usize = MAX_SPRITES * 4;
/// 2048 * 4 * 6 = 49152 < 65536 = 2^16
pub const MAX_INDICES: usize = MAX_SPRITES * 6;

// --------------------------------------------------------------------------------
// VertexData

/// The actual vertex data
///
/// The data layout is dynamically specified as `fna3d::VertexDeclaration`
///
/// * `dest`: position in pixels in target
/// * `color`: color
/// * `uvs`: normalized position in texture (a.k.ak. texture coordinates)
#[derive(Debug, Clone)]
#[repr(C)]
pub struct ColoredVertexData {
    pub dest: Vec3f, // TODO: use 2D dest vec
    pub color: fna3d::Color,
    pub uvs: Vec2f,
}

impl crate::gfx::vertices::VertexData for ColoredVertexData {}

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

// --------------------------------------------------------------------------------
// Primitives

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Top-left and size
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Rect2f {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Vec2f {
    pub fn round(&mut self) {
        self.x = self.x.round();
        self.y = self.y.round();
    }
}

impl Rect2f {
    pub fn normalized() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 1.0,
            h: 1.0,
        }
    }

    pub fn left_up(&self) -> Vec2f {
        Vec2f {
            x: self.x,
            y: self.y,
        }
    }

    pub fn size(&self) -> Vec2f {
        Vec2f {
            x: self.w,
            y: self.h,
        }
    }
}
