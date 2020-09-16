use crate::{cmd::prelude::*, geom::*};

pub use crate::texture::texture::{TextureData2D, TextureKind};

/// Conversion
impl TextureData2D {
    pub fn trim_px(&self, rect: impl Into<[u32; 4]>) -> SubTextureData2D {
        let rect = rect.into();
        let uv_rect = [
            rect[0] as f32 / self.w as f32,
            rect[1] as f32 / self.h as f32,
            rect[2] as f32 / self.w as f32,
            rect[3] as f32 / self.h as f32,
        ];
        SubTextureData2D {
            texture: self.clone(),
            uv_rect,
        }
    }

    pub fn trim_uv(&self, uv_rect: impl Into<Rect2f>) -> SubTextureData2D {
        SubTextureData2D {
            texture: self.clone(),
            // T -> Rect2f -> [f32; 4]
            //
            // this is NOT performance-wise, but good for ease
            // e.g. `my_texture.trim_uv[offset, size]`
            uv_rect: uv_rect.into().into(),
        }
    }
}

/// 2D texture handle with region (uv values)
///
/// # Safety
///
/// It's NOT guaranteed that the internal texture is still alive because it's using a pointer.
#[derive(Debug, PartialEq, Clone)]
pub struct SubTextureData2D {
    pub(crate) texture: TextureData2D,
    pub(crate) uv_rect: [f32; 4],
}

impl SubTextureData2D {
    pub fn new(texture: TextureData2D, uv_rect: impl Into<[f32; 4]>) -> Self {
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

impl AsRef<TextureData2D> for SubTextureData2D {
    fn as_ref(&self) -> &TextureData2D {
        &self.texture
    }
}

/// Full-featured 2D texture handle
///
/// # Safety
///
/// It's NOT guaranteed that the internal texture is still alive because it's using a pointer.
#[derive(Debug, Clone)]
pub struct SpriteData {
    pub texture: TextureData2D,
    pub uv_rect: Rect2f,
    /// [0.0, 0.0] is left-up (default0, [1.0, 1.0] is right-down
    pub origin: Vec2f,
    pub color: fna3d::Color,
    pub scale: Vec2f,
    pub rot: f32,
    pub flips: Flips,
}

impl AsRef<TextureData2D> for SpriteData {
    fn as_ref(&self) -> &TextureData2D {
        &self.texture
    }
}

impl Default for SpriteData {
    fn default() -> Self {
        Self {
            texture: TextureData2D::empty(),
            uv_rect: Rect2f::unit(),
            origin: Vec2f::zero(),
            color: fna3d::Color::white(),
            scale: Vec2f::one(),
            rot: 0.0,
            flips: Flips::NONE,
        }
    }
}

impl SpriteData {
    pub fn size(&self) -> Vec2f {
        self.uv_rect.size() * Vec2f::new(self.texture.w as f32, self.texture.h as f32)
    }
    pub fn size_uv(&self) -> Vec2f {
        self.uv_rect.size()
    }
}

// --------------------------------------------------------------------------------
// Sprite/texture trait impls

// TextureData2D
impl Texture2D for TextureData2D {
    fn raw_texture(&self) -> *mut fna3d::Texture {
        self.raw()
    }

    fn w(&self) -> f32 {
        self.w as f32
    }

    fn h(&self) -> f32 {
        self.h as f32
    }
}

impl SubTexture2D for TextureData2D {
    fn uv_rect(&self) -> [f32; 4] {
        [0.0, 0.0, 1.0, 1.0]
    }
}

// SubTexuteData2D (delegated to `Texture2D`)
impl Texture2D for SubTextureData2D {
    fn raw_texture(&self) -> *mut fna3d::Texture {
        self.texture.raw()
    }

    fn w(&self) -> f32 {
        self.texture.w()
    }

    fn h(&self) -> f32 {
        self.texture.h()
    }
}

impl SubTexture2D for SubTextureData2D {
    fn uv_rect(&self) -> [f32; 4] {
        self.uv_rect
    }
}

// Sprite (delegated to `TextureData2D`)
impl Texture2D for SpriteData {
    fn raw_texture(&self) -> *mut fna3d::Texture {
        self.texture.raw_texture()
    }
    fn w(&self) -> f32 {
        self.texture.w()
    }
    fn h(&self) -> f32 {
        self.texture.h()
    }
}

impl SubTexture2D for SpriteData {
    fn uv_rect(&self) -> [f32; 4] {
        self.uv_rect.clone().into()
    }
}

impl Sprite for SpriteData {
    fn rot(&self) -> f32 {
        self.rot
    }
    fn scale(&self) -> [f32; 2] {
        self.scale.into()
    }
    fn origin(&self) -> [f32; 2] {
        self.origin.into()
    }
}

// implementations for reference types
impl<T: Texture2D> Texture2D for &T {
    fn raw_texture(&self) -> *mut fna3d::Texture {
        (*self).raw_texture()
    }
    fn w(&self) -> f32 {
        (*self).w()
    }
    fn h(&self) -> f32 {
        (*self).h()
    }
}

impl<T: SubTexture2D> SubTexture2D for &T {
    fn uv_rect(&self) -> [f32; 4] {
        (*self).uv_rect()
    }
}

impl<T: Sprite> Sprite for &T {
    fn rot(&self) -> f32 {
        (*self).rot()
    }
    fn scale(&self) -> [f32; 2] {
        (*self).scale()
    }
    fn origin(&self) -> [f32; 2] {
        (*self).origin()
    }
}
