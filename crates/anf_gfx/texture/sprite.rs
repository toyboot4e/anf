pub use crate::texture::texture::{TextureData2d, TextureKind};

use fna3h::Color;

use crate::geom2d::*;

/// Conversion
impl TextureData2d {
    pub fn trim_px(&self, rect: impl Into<[u32; 4]>) -> SubTextureData2d {
        let rect = rect.into();

        let uv_rect = [
            rect[0] as f32 / self.w as f32,
            rect[1] as f32 / self.h as f32,
            rect[2] as f32 / self.w as f32,
            rect[3] as f32 / self.h as f32,
        ];

        SubTextureData2d {
            texture: self.clone(),
            uv_rect,
        }
    }

    pub fn trim_uv(&self, uv_rect: impl Into<Rect2f>) -> SubTextureData2d {
        SubTextureData2d {
            texture: self.clone(),
            // TODO: change this: T -> Rect2f -> [f32; 4]
            uv_rect: uv_rect.into().into(),
        }
    }
}

/// 2D texture handle with region (uv values)
#[derive(Debug, PartialEq, Clone)]
pub struct SubTextureData2d {
    pub(crate) texture: TextureData2d,
    pub(crate) uv_rect: [f32; 4],
}

impl SubTextureData2d {
    pub fn new(texture: TextureData2d, uv_rect: impl Into<[f32; 4]>) -> Self {
        Self {
            texture,
            uv_rect: uv_rect.into(),
        }
    }

    pub fn size(&self) -> [f32; 2] {
        self.texture.size()
    }

    pub fn size_uv(&self) -> [f32; 2] {
        let size = self.texture.size();
        let parent = [self.uv_rect[2], self.uv_rect[3]];
        [size[0] / parent[0], size[1] / parent[1]]
    }
}

impl AsRef<TextureData2d> for SubTextureData2d {
    fn as_ref(&self) -> &TextureData2d {
        &self.texture
    }
}

/// 2D texture handle with region (uv values), origin, color, scale, rotation and flips
#[derive(Debug, Clone)]
pub struct SpriteData {
    pub texture: TextureData2d,
    pub uv_rect: Rect2f,
    /// [0.0, 0.0] is left up (default value), [1.0, 1.0] is right down
    pub origin: Vec2f,
    pub color: Color,
    pub scale: Vec2f,
    pub rot: f32,
    pub flips: Flips,
}

impl AsRef<TextureData2d> for SpriteData {
    fn as_ref(&self) -> &TextureData2d {
        &self.texture
    }
}

impl SpriteData {
    /// Alternative to [`Default`]
    pub fn from_texture(texture: TextureData2d) -> Self {
        Self {
            texture,
            uv_rect: Rect2f::unit(),
            origin: Vec2f::zero(),
            color: Color::white(),
            scale: Vec2f::one(),
            rot: 0.0,
            flips: Flips::NONE,
        }
    }

    pub fn texture_w(&self) -> u32 {
        self.texture.w
    }

    pub fn texture_y(&self) -> u32 {
        self.texture.w
    }

    pub fn texture_size_px(&self) -> [u32; 2] {
        [self.texture.w, self.texture.h]
    }

    pub fn size_px(&self) -> Vec2f {
        self.uv_rect.size() * Vec2f::new(self.texture.w as f32, self.texture.h as f32)
    }

    pub fn size_uv(&self) -> Vec2f {
        self.uv_rect.size()
    }
}
