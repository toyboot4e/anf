use crate::{
    batcher::batch::SpriteBatch,
    cmd::push::*,
    geom::*,
    texture::{SpriteData, SubTextureData2D, TextureData2D},
};

pub trait SubTexture: Texture2D {
    /// [x, y, w, h]: Normalized rectangle that represents a regon in texture
    fn uv_rect(&self) -> [f32; 4];
}

pub trait Sprite: SubTexture {
    fn rot(&self) -> f32;
    fn scale(&self) -> [f32; 2];
}

// --------------------------------------------------------------------------------
// impls

// TODO: share implementations?

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

impl SubTexture for TextureData2D {
    fn uv_rect(&self) -> [f32; 4] {
        [0.0, 0.0, 1.0, 1.0]
    }
}

// SubTextureData2D (delegated to `Texture2D`)
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

impl SubTexture for SubTextureData2D {
    fn uv_rect(&self) -> [f32; 4] {
        self.uv_rect
    }
}

// Sprite (delegated to `TextureData2D`)
impl Texture2D for SpriteData {
    fn raw_texture(&self) -> *mut fna3d::Texture {
        self.sub_tex.raw_texture()
    }

    fn w(&self) -> f32 {
        self.sub_tex.w()
    }

    fn h(&self) -> f32 {
        self.sub_tex.h()
    }
}

impl SubTexture for SpriteData {
    fn uv_rect(&self) -> [f32; 4] {
        self.sub_tex.uv_rect()
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

impl<T: SubTexture> SubTexture for &T {
    fn uv_rect(&self) -> [f32; 4] {
        (*self).uv_rect()
    }
}

// --------------------------------------------------------------------------------
// push

/// [`SpritePushCommand`] builder
pub trait PushGeometryBuilder {
    /// This is mainly for default implementations, but it can be used to modify [`QuadPush`] manually
    fn data(&mut self) -> &mut QuadPush;

    fn src_rect_normalized(&mut self, rect: impl Into<Rect2f>) -> &mut Self {
        self.data().src_rect = Scaled::Normalized(rect.into());
        self
    }

    fn src_rect_px(&mut self, rect: impl Into<Rect2f>) -> &mut Self {
        self.data().src_rect = Scaled::Px(rect.into());
        self
    }

    fn dest_pos_px(&mut self, xs: impl Into<[f32; 2]>) -> &mut Self {
        let xs = xs.into();

        let data = self.data();
        let mut rect = data.dest_rect.inner().clone();
        rect.x = xs[0];
        rect.y = xs[1];
        data.dest_rect = Scaled::Px(rect);

        self
    }

    fn dest_size_px(&mut self, ws: impl Into<[f32; 2]>) -> &mut Self {
        let ws = ws.into();

        let data = self.data();
        let mut rect = data.dest_rect.inner().clone();
        rect.w = ws[0];
        rect.h = ws[1];
        data.dest_rect = Scaled::Px(rect);

        self
    }

    fn dest_rect_px(&mut self, xs: impl Into<Rect2f>) -> &mut Self {
        let rect = xs.into();

        let data = self.data();
        data.dest_rect = Scaled::Px(rect.into());

        self
    }

    fn origin(&mut self, origin: Vec2f) -> &mut Self {
        self.data().origin = origin;
        self
    }

    fn color(&mut self, color: fna3d::Color) -> &mut Self {
        self.data().color = color;
        self
    }
}

/// Quads with color, rotation and skews
pub struct SpritePushCommand<'a> {
    pub push: &'a mut QuadPush,
    pub batch: &'a mut SpriteBatch,
    pub policy: DrawPolicy,
    pub flips: Flips,
}

impl<'a> PushGeometryBuilder for SpritePushCommand<'a> {
    fn data(&mut self) -> &mut QuadPush {
        &mut self.push
    }
}

impl<'a, 'b> SpritePushCommand<'b> {
    /// Sets texture
    pub fn texture<T: SubTexture>(&'a mut self, texture: T) -> SizedTexturePush<'a, 'b, T> {
        self.src_rect_normalized(texture.uv_rect());
        self.dest_size_px([texture.w(), texture.h()]);

        SizedTexturePush { cmd: self, texture }
    }

    /// Sets sprite
    pub fn sprite<T: Sprite>(&'a mut self, sprite: T) -> SizedTexturePush<'a, 'b, T> {
        self.src_rect_normalized(sprite.uv_rect());
        let scale = sprite.scale();
        self.dest_size_px([sprite.w() * scale[0], sprite.h() * scale[1]]);
        self.data().rot = sprite.rot();

        SizedTexturePush {
            cmd: self,
            texture: sprite,
        }
    }
}

/// Handle to push quads with a texture
pub struct SizedTexturePush<'a, 'b, T: Texture2D> {
    cmd: &'a mut SpritePushCommand<'b>,
    texture: T,
}

impl<'a, 'b, T: Texture2D> PushGeometryBuilder for SizedTexturePush<'a, 'b, T> {
    fn data(&mut self) -> &mut QuadPush {
        &mut self.cmd.push
    }
}

impl<'a, 'b, T: Texture2D> Drop for SizedTexturePush<'a, 'b, T> {
    fn drop(&mut self) {
        self.run();
    }
}

impl<'a, 'b, T: Texture2D> SizedTexturePush<'a, 'b, T> {
    fn run(&mut self) {
        // log::info!("{:?}", self.cmd.push);
        self.cmd.push.run_sized_texture(
            &mut self.cmd.batch,
            &self.texture,
            self.cmd.policy,
            self.cmd.flips,
        );
    }
}
