use crate::{
    batcher::{batch::SpriteBatch, primitives::*},
    cmd::push::*,
    texture::{SubTexture2D, Texture2D},
};

impl RawTexture for *mut fna3d::Texture {
    fn raw_texture(&self) -> *mut fna3d::Texture {
        self.clone()
    }
}

impl RawTexture for &Texture2D {
    fn raw_texture(&self) -> *mut fna3d::Texture {
        self.raw()
    }
}

impl SizedTexture for &Texture2D {
    fn w(&self) -> f32 {
        self.w as f32
    }

    fn h(&self) -> f32 {
        self.h as f32
    }
}

pub trait SubTexture: SizedTexture {
    /// [x, y, w, h]: Normalized rectangle that represents a regon in texture
    fn uv_rect(&self) -> [f32; 4];
}

impl SubTexture for &crate::texture::Texture2D {
    fn uv_rect(&self) -> [f32; 4] {
        [0.0, 0.0, 1.0, 1.0]
    }
}

/// Default implementation of `SpritePush` builder
pub trait PushGeometryBuilder {
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
}

// QuadPush with SizedTexture

pub struct SizedTexturePush<'a, 'b, T: SizedTexture> {
    cmd: &'a mut SpritePushCommand<'b>,
    texture: T,
}

impl<'a, 'b, T: SizedTexture> PushGeometryBuilder for SizedTexturePush<'a, 'b, T> {
    fn data(&mut self) -> &mut QuadPush {
        &mut self.cmd.push
    }
}

impl<'a, 'b, T: SizedTexture> Drop for SizedTexturePush<'a, 'b, T> {
    fn drop(&mut self) {
        self.run();
    }
}

impl<'a, 'b, T: SizedTexture> SizedTexturePush<'a, 'b, T> {
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
