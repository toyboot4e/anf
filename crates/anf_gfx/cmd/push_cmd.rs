use crate::{
    batcher::batch::SpriteBatch,
    cmd::push_params::{DrawPolicy, QuadPush, Scaled, Texture2D},
    geom2d::*,
};

/// Texture with size data and region. Used by [`QuadPushBuilder`]
pub trait SubTexture2D: Texture2D {
    /// [x, y, w, h]: Normalized rectangle that represents a regon in texture
    fn uv_rect(&self) -> [f32; 4];
}

/// Texture with size data, region and other geometry data. Used by [`QuadPushBuilder`]
pub trait Sprite: SubTexture2D {
    /// Rotation in radian
    fn rot(&self) -> f32;
    fn scale(&self) -> [f32; 2];
    /// Normalized origin
    fn origin(&self) -> [f32; 2];
}

/// Comes with default implementation
pub trait QuadPushBuilder {
    /// This is mainly for default implementations, but it can be used to modify [`QuadPush`] manually
    fn data(&mut self) -> &mut QuadPush;

    /// Set source rectangle in normalized coordinates
    fn src_rect_uv(&mut self, rect: impl Into<Rect2f>) -> &mut Self {
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

    fn origin(&mut self, origin: impl Into<Vec2f>) -> &mut Self {
        self.data().origin = origin.into();
        self
    }

    fn color(&mut self, color: fna3d::Color) -> &mut Self {
        self.data().color = color;
        self
    }

    fn rot(&mut self, rot: f32) -> &mut Self {
        self.data().rot = rot;
        self
    }

    fn flips(&mut self, flips: Flips) -> &mut Self {
        self.data().flips = flips;
        self
    }

    fn skew(&mut self, skew: Skew2f) -> &mut Self {
        self.data().skew = skew;
        self
    }
}

pub struct QuadPushBinding<'a> {
    pub push: &'a mut QuadPush,
    pub batch: &'a mut SpriteBatch,
}

impl<'a> QuadPushBuilder for QuadPushBinding<'a> {
    fn data(&mut self) -> &mut QuadPush {
        &mut self.push
    }
}

impl<'a> QuadPushBinding<'a> {
    fn on_set_sub_texture<T: SubTexture2D>(&'_ mut self, texture: &T) {
        self.src_rect_uv(texture.uv_rect())
            .dest_size_px([texture.w(), texture.h()]);
    }

    pub fn on_set_sprite<T: Sprite>(&'_ mut self, sprite: &T) {
        let scale = sprite.scale();
        self.src_rect_uv(sprite.uv_rect())
            .dest_size_px([sprite.w() * scale[0], sprite.h() * scale[1]])
            .origin(sprite.origin())
            .rot(sprite.rot());
    }
}

/// Primary interface to push sprite
pub struct SpritePushCommand<'a, T: Texture2D> {
    quad: QuadPushBinding<'a>,
    texture: T,
    policy: DrawPolicy,
    flips: Flips,
}

/// Push sprite to batch data when it goes out of scope
impl<'a, T: Texture2D> Drop for SpritePushCommand<'a, T> {
    fn drop(&mut self) {
        self.run();
    }
}

impl<'a, T: Texture2D> SpritePushCommand<'a, T> {
    pub fn new(quad: QuadPushBinding<'a>, texture: T) -> Self {
        Self {
            quad,
            texture,
            policy: DrawPolicy { do_round: false },
            flips: Flips::NONE,
        }
    }

    fn run(&mut self) {
        self.quad
            .push
            .run_texture2d(&mut self.quad.batch, &self.texture, self.policy, self.flips);
    }
}

/// impl default builder methods
impl<'a, T: Texture2D> QuadPushBuilder for SpritePushCommand<'a, T> {
    fn data(&mut self) -> &mut QuadPush {
        &mut self.quad.push
    }
}

impl<'a, T: SubTexture2D> SpritePushCommand<'a, T> {
    pub fn from_sub_texture(mut quad: QuadPushBinding<'a>, sub_texture: T) -> Self {
        quad.on_set_sub_texture(&sub_texture);
        Self::new(quad, sub_texture)
    }
}

impl<'a, T: Sprite> SpritePushCommand<'a, T> {
    pub fn from_sprite(mut quad: QuadPushBinding<'a>, sub_texture: T) -> Self {
        quad.on_set_sprite(&sub_texture);
        Self::new(quad, sub_texture)
    }
}
