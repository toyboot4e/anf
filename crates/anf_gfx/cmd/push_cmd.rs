use crate::{
    batcher::bufspecs::QuadData,
    cmd::push_params::{DrawPolicy, QuadParams, Scaled, Texture2d},
    geom2d::*,
};

// TODO: put it here
use crate::texture::TextureData2d;

/// Texture with size data and region. Used by [`QuadPushBuilder`]
pub trait SubTexture2d: Texture2d {
    /// [x, y, w, h]: Normalized rectangle that represents a regon in texture
    fn uv_rect(&self) -> [f32; 4];
}

/// Texture with size data, region and other geometry data. Used by [`QuadPushBuilder`]
pub trait Sprite: SubTexture2d {
    /// Rotation in radian
    fn rot(&self) -> f32;
    fn scale(&self) -> [f32; 2];
    /// Normalized origin
    fn origin(&self) -> [f32; 2];
    fn color(&self) -> fna3d::Color;
}

/// Comes with default implementation
pub trait QuadParamsBuilder {
    /// This is mainly for default implementations, but it can be used to modify [`QuadParams`] manually
    fn params(&mut self) -> &mut QuadParams;

    /// Set source rectangle in normalized coordinates
    ///
    /// Specify [x, y] and [w, h].
    fn src_rect_uv(&mut self, rect: impl Into<Rect2f>) -> &mut Self {
        self.params().src_rect = Scaled::Normalized(rect.into());
        self
    }

    /// Set the source rectangle in normalized pixels
    ///
    /// Specify [x, y] and [w, h].
    fn src_rect_px(&mut self, rect: impl Into<Rect2f>) -> &mut Self {
        self.params().src_rect = Scaled::Px(rect.into());
        self
    }

    /// Sets the origin position to the destination
    fn dest_pos_px(&mut self, xs: impl Into<[f32; 2]>) -> &mut Self {
        let xs = xs.into();

        let data = self.params();
        let mut rect = data.dest_rect.inner().clone();
        rect.x = xs[0];
        rect.y = xs[1];
        data.dest_rect = Scaled::Px(rect);

        self
    }

    /// Sets the size to the destination
    fn dest_size_px(&mut self, ws: impl Into<[f32; 2]>) -> &mut Self {
        let ws = ws.into();

        let data = self.params();
        let mut rect = data.dest_rect.inner().clone();
        rect.w = ws[0];
        rect.h = ws[1];
        data.dest_rect = Scaled::Px(rect);

        self
    }

    /// Sets origin position and size to the destination
    fn dest_rect_px(&mut self, xs: impl Into<Rect2f>) -> &mut Self {
        let rect = xs.into();

        let data = self.params();
        data.dest_rect = Scaled::Px(rect.into());

        self
    }

    /// Sets origin where we specify coordinates / where the quad rotates
    fn origin(&mut self, origin: impl Into<Vec2f>) -> &mut Self {
        self.params().origin = origin.into();
        self
    }

    /// Alpha value is considered here, too
    fn color(&mut self, color: fna3d::Color) -> &mut Self {
        self.params().color = color;
        self
    }

    fn rot(&mut self, rot: f32) -> &mut Self {
        self.params().rot = rot;
        self
    }

    fn flips(&mut self, flips: Flips) -> &mut Self {
        self.params().flips = flips;
        self
    }

    fn skew(&mut self, skew: Skew2f) -> &mut Self {
        self.params().skew = skew;
        self
    }
}

///
pub struct QuadPush<'a> {
    pub params: &'a mut QuadParams,
    pub target: &'a mut QuadData,
}

impl<'a> QuadParamsBuilder for QuadPush<'a> {
    fn params(&mut self) -> &mut QuadParams {
        &mut self.params
    }
}

impl<'a> QuadPush<'a> {
    fn on_set_sub_texture<T: SubTexture2d>(&'_ mut self, texture: &T) {
        self.src_rect_uv(texture.uv_rect())
            .dest_size_px([texture.w(), texture.h()]);
    }

    fn on_set_sprite<T: Sprite>(&'_ mut self, sprite: &T) {
        let scale = sprite.scale();
        self.src_rect_uv(sprite.uv_rect())
            .dest_size_px([sprite.w() * scale[0], sprite.h() * scale[1]])
            .origin(sprite.origin())
            .rot(sprite.rot())
            .color(sprite.color());
    }
}

/// Primary interface to push sprite
pub struct SpritePush<'a, T: Texture2d> {
    quad: QuadPush<'a>,
    texture: TextureData2d,
    policy: DrawPolicy,
    flips: Flips,
    /// From which `Texture2d` this is created
    _marker: std::marker::PhantomData<T>,
}

/// Push sprite to batch data when it goes out of scope
impl<'a, T: Texture2d> Drop for SpritePush<'a, T> {
    fn drop(&mut self) {
        self.run();
    }
}

impl<'a, T: Texture2d> SpritePush<'a, T> {
    /// Make sure the [`QuadPush`] is not satuerd
    pub fn new(quad: QuadPush<'a>, texture: &T) -> Self {
        let texture = TextureData2d::from_raw(
            texture.raw_texture(),
            // TODO: don't cast
            texture.w() as u32,
            texture.h() as u32,
            // FIXME:
            fna3d::SurfaceFormat::Color,
        );

        Self {
            quad,
            texture,
            policy: DrawPolicy { do_round: false },
            flips: Flips::NONE,
            _marker: std::marker::PhantomData,
        }
    }

    fn run(&mut self) {
        self.quad.params.run_texture2d(
            &mut self.quad.target,
            &self.texture,
            self.policy,
            self.flips,
        );
    }
}

/// impl default builder methods
impl<'a, 'b, T: Texture2d> QuadParamsBuilder for SpritePush<'a, T> {
    fn params(&mut self) -> &mut QuadParams {
        &mut self.quad.params
    }
}

impl<'a, 'b, T: SubTexture2d> SpritePush<'a, T> {
    /// Make sure the [`QuadPush`] is not satuerd
    pub fn from_sub_texture(mut quad: QuadPush<'a>, sub_texture: &'b T) -> Self {
        quad.on_set_sub_texture(sub_texture);
        Self::new(quad, sub_texture)
    }
}

impl<'a, 'b, T: Sprite> SpritePush<'a, T> {
    /// Make sure the [`QuadPush`] is not satuerd
    pub fn from_sprite(mut quad: QuadPush<'a>, sub_texture: &'b T) -> Self {
        quad.on_set_sprite(sub_texture);
        Self::new(quad, sub_texture)
    }
}
