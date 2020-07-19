//! Internal utilities in `batch` module
//!
//! We use 15bits for vertex index

pub const MAX_SPRITES: usize = 2048;
pub const MAX_VERTICES: usize = MAX_SPRITES * 4;
pub const MAX_INDICES: usize = MAX_SPRITES * 6;

/// The actual vertex data
///
/// The data layout is dynamically specified to `fna3d::Device` as `fna3d::VertexDeclaration`
///
/// * `pos`: normalized position in TODO: where? render target?
/// * `uvs`: normalized position in texture (a.k.ak. texture coordinates)
#[derive(Debug, Clone)]
pub struct VertexData {
    pub pos: Vec3f,
    pub color: fna3d::Color,
    pub uvs: Vec2f,
}

impl crate::gfx::vertices::AnyVertexData for VertexData {}

impl Default for VertexData {
    fn default() -> Self {
        let color = fna3d::Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        };
        Self {
            pos: Vec3f::default(),
            color,
            uvs: Vec2f::default(),
            // ..Default::default() // TODO: why does it result in recursive call
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Rect2f {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Skew2f {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

impl VertexData {
    pub fn elems() -> &'static [fna3d::VertexElement] {
        &[
            fna3d::VertexElement {
                offset: 0,
                vertexElementFormat: fna3d::VertexElementFormat::Vector3 as u32,
                vertexElementUsage: fna3d::VertexElementUsage::Position as u32,
                usageIndex: 0,
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

    // pub fn decl() -> fna3d::VertexDeclaration {
    //     fna3d::VertexDeclaration::from_elems(Self::elems())
    // }
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
            x: 0 as f32,
            y: 0 as f32,
            w: 1 as f32,
            h: 1 as f32,
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
