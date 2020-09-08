//! Push command to `SpriteBatch`
//!
//! The internal implementation is based on `Batcher` in Nez
//!
//! * FiXME: array of struct vs struct of array
//! * TODO: try rotations
//! * TODO: try using transoform matrix

use crate::batcher::{batch::SpriteBatch, bufspecs::QuadData, primitives::*};

/// Round or not
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DrawPolicy {
    pub do_round: bool,
    // is_batching_disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct Rect2u {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Debug)]
pub enum Scaled<T> {
    Px(T),
    Normalized(T),
}

impl<T> Scaled<T> {
    pub fn inner(&self) -> &T {
        match self {
            Scaled::Px(x) => x,
            Scaled::Normalized(x) => x,
        }
    }
}

// --------------------------------------------------------------------------------
// traits

pub trait RawTexture {
    fn raw_texture(&self) -> *mut fna3d::Texture;
}

pub trait SizedTexture: RawTexture {
    /// Pixel
    fn w(&self) -> f32;
    /// Pixel
    fn h(&self) -> f32;
}

// --------------------------------------------------------------------------------
// SpritePush

/// Full-featured parameter to push a sprite into `SpriteBatch`
///
/// Geometry values may be normalized or un-normalized and they're normalized when we push
/// quadliterals.
#[derive(Debug)]
pub struct QuadPush {
    // TODO: consider using two vectors per src/dest
    pub src_rect: Scaled<Rect2f>,
    pub dest_rect: Scaled<Rect2f>,
    /// Normalized origin
    pub origin: Vec2f,
    pub color: fna3d::Color,
    pub rot: f32,
    pub depth: f32,
    pub flips: Flips,
    pub skew: Skew2f,
}

impl Default for QuadPush {
    fn default() -> Self {
        Self {
            src_rect: Scaled::Normalized(Rect2f::normalized()),
            dest_rect: Scaled::Normalized(Rect2f::default()),
            origin: Vec2f::default(),
            color: fna3d::Color::white(),
            rot: 0.0,
            depth: 0.0,
            flips: Flips::NONE,
            skew: Skew2f::default(),
        }
    }
}

impl QuadPush {
    pub fn reset_to_defaults(&mut self) {
        self.src_rect = Scaled::Normalized(Rect2f::normalized());
        self.dest_rect = Scaled::Normalized(Rect2f::default());
        self.origin = Vec2f::default();
        self.color = fna3d::Color::white();
        self.rot = 0.0;
        self.depth = 0.0;
        self.flips = Flips::NONE;
        self.skew = Skew2f::default();
    }
}

/// Run
/// ---
///
/// Flush `SpriteBatch` before running if it's saturated.
impl QuadPush {
    pub fn run_sized_texture(
        &self,
        batch: &mut SpriteBatch,
        texture: &impl SizedTexture,
        policy: DrawPolicy,
        flips: Flips,
    ) {
        let (src_rect, dest_rect) = self.geometry_normalized(policy, texture);
        // TODO: round
        // if policy.do_round {
        //     rect.x = rect.x.round();
        //     rect.y = rect.y.round();
        // }
        self::push_sized_texture(
            batch,
            texture,
            self.origin,
            src_rect,
            dest_rect,
            self.skew,
            self.color,
            self.rot,
            self.depth,
            flips,
        );
    }

    /// -> (src_rect, origin, dest_rect)
    #[inline]
    fn geometry_normalized(
        &self,
        policy: DrawPolicy,
        texture: &impl SizedTexture,
    ) -> (Rect2f, Rect2f) {
        let inv_tex_w = 1.0 / texture.w();
        let inv_tex_h = 1.0 / texture.h();

        let src_rect = match &self.src_rect {
            Scaled::Normalized(uvs) => uvs.clone(),
            Scaled::Px(rect) => Rect2f {
                x: rect.x * inv_tex_w,
                y: rect.y * inv_tex_h,
                w: rect.w * inv_tex_w,
                h: rect.h * inv_tex_h,
            },
        };

        let dest_rect = match &self.dest_rect {
            Scaled::Normalized(rect) => Rect2f {
                x: rect.x * texture.w(),
                y: rect.y * texture.h(),
                w: rect.w * texture.w(),
                h: rect.h * texture.h(),
            },
            Scaled::Px(rect) => Rect2f {
                x: rect.x,
                y: rect.y,
                // TODO: does this work well with sub textures
                w: rect.w * src_rect.w,
                h: rect.h * src_rect.h,
            },
        };

        (src_rect, dest_rect)
    }
}

// --------------------------------------------------------------------------------
// Core

/// Pass normalized geometry values
#[inline]
fn push_sized_texture(
    batch: &mut SpriteBatch,
    texture: &impl RawTexture,
    origin: Vec2f,
    src_rect: Rect2f,
    dest_rect: Rect2f,
    skew: Skew2f,
    color: fna3d::Color,
    rot: f32,
    depth: f32,
    flips: Flips,
) {
    let quad = &mut batch.quads[batch.n_quads];
    self::set_quad(
        quad, skew, origin, src_rect, dest_rect, color, rot, depth, flips,
    );
    batch.raw_texture_track[batch.n_quads] = texture.raw_texture();
    batch.n_quads += 1;
}

/// Normalized x offsets at top-left, top-right, bottom-left, bottom-right
const CORNER_OFFSET_X: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

/// Normalized y offsets at top-left, top-right, bottom-left, bottom-right
const CORNER_OFFSET_Y: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

/// Pass normalized geometry values
#[inline]
fn set_quad(
    quad: &mut QuadData,
    mut skew: Skew2f,
    origin: Vec2f,
    src_rect: Rect2f,
    dest_rect: Rect2f,
    color: fna3d::Color,
    rot: f32,
    depth: f32,
    flips: Flips,
) {
    let rot = if rot >= f32::EPSILON {
        let sin = rot.sin();
        let cos = rot.cos();
        Rot2f {
            x1: cos,
            y1: sin,
            x2: -sin,
            y2: cos,
        }
    } else {
        Rot2f {
            x1: 1.0,
            y1: 0.0,
            x2: 0.0,
            y2: 1.0,
        }
    };

    // flip our skew values if we have a flipped sprite
    // FIXME is this OK??
    if flips != Flips::NONE {
        skew.y1 *= -1.0;
        skew.y2 *= -1.0;
        skew.x1 *= -1.0;
        skew.x2 *= -1.0;
    }

    // top, top, bottom, bottom
    let skew_xs = [skew.x1, skew.x1, skew.x2, skew.x2];
    // left, right, right, left
    let skew_ys = [skew.y1, skew.y2, skew.y1, skew.y2];

    // push four vertices: top-left, top-right, bottom-left, and bottom-right, respectively
    for i in 0..4 {
        let corner_x = (CORNER_OFFSET_X[i] - origin.x) * dest_rect.w + skew_xs[i];
        let corner_y = (CORNER_OFFSET_Y[i] - origin.y) * dest_rect.h - skew_ys[i];

        quad[i].dest.x = (rot.x2 * corner_y) + (rot.x1 * corner_x) + dest_rect.x;
        quad[i].dest.y = (rot.y2 * corner_y) + (rot.y1 * corner_x) + dest_rect.y;
        quad[i].dest.z = depth;

        // Here, `^` is xor (exclusive or) operator. So if `effects` (actually flips?) equals to
        // zero, it does nothing and `i ^ effects` == `i`
        quad[i].uvs.x = (CORNER_OFFSET_X[i ^ flips.bits() as usize] * src_rect.w) + src_rect.x;
        quad[i].uvs.y = (CORNER_OFFSET_Y[i ^ flips.bits() as usize] * src_rect.h) + src_rect.y;

        quad[i].color = color;
    }
}
