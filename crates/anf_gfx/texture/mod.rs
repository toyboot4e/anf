//! 2D texture
//!
//! TODO: remove conversion methods

mod sprite;
mod texture;

pub use self::{
    sprite::{SpriteData, SubTextureData2d},
    texture::{Texture2dDrop, TextureData2d, TextureKind},
};

use crate::cmd::traits::*;
use fna3d::Color;

// --------------------------------------------------------------------------------
// impl texture/sprite traits

// TextureData2d
impl Texture2d for TextureData2d {
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

impl SubTexture2d for TextureData2d {
    fn uv_rect(&self) -> [f32; 4] {
        [0.0, 0.0, 1.0, 1.0]
    }
}

// SubTexuteData2d (delegated to `Texture2d`)
impl Texture2d for SubTextureData2d {
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

impl SubTexture2d for SubTextureData2d {
    fn uv_rect(&self) -> [f32; 4] {
        self.uv_rect
    }
}

// Sprite (delegated to `TextureData2d`)
impl Texture2d for SpriteData {
    fn raw_texture(&self) -> *mut fna3d::Texture {
        self.texture.raw_texture()
    }
    fn w(&self) -> f32 {
        self.texture.w() * self.uv_rect.w
    }
    fn h(&self) -> f32 {
        self.texture.h() * self.uv_rect.h
    }
}

impl SubTexture2d for SpriteData {
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
    fn color(&self) -> Color {
        self.color
    }
}

// trait implementations for reference types

impl<T: Texture2d> Texture2d for &T {
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

impl<T: SubTexture2d> SubTexture2d for &T {
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
    fn color(&self) -> Color {
        (*self).color()
    }
}

// --------------------------------------------------------------------------------
// impl OnSpritePush

impl OnSpritePush for TextureData2d {
    fn to_texture(&self) -> TextureData2d {
        self.clone()
    }

    fn on_sprite_push(&self, builder: &mut impl QuadParamsBuilder) {
        // TODO: needed?
        builder
            .src_rect_uv(self.uv_rect())
            .dest_size_px([self.w(), self.h()]);
    }
}

impl OnSpritePush for SubTextureData2d {
    fn to_texture(&self) -> TextureData2d {
        self.texture.clone()
    }

    fn on_sprite_push(&self, builder: &mut impl QuadParamsBuilder) {
        builder
            .src_rect_uv(self.uv_rect())
            .dest_size_px([self.w(), self.h()]);
    }
}

impl OnSpritePush for SpriteData {
    fn to_texture(&self) -> TextureData2d {
        self.texture.clone()
    }

    fn on_sprite_push(&self, builder: &mut impl QuadParamsBuilder) {
        let scale = self.scale();
        builder
            .src_rect_uv(self.uv_rect())
            .dest_size_px([self.w() * scale[0], self.h() * scale[1]])
            .origin(self.origin())
            .rot(self.rot())
            .color(self.color());
    }
}
