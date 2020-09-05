//! Push command to `SpriteBatch`
//!
//! The internal implementation is based on `Batcher` in Nez

use crate::{
    batcher::{batch::SpriteBatch, bufspecs::QuadData, primitives::*},
    texture::Texture2D,
};

// bitflags::bitflags! {
//     /// `SpriteEffects` in FNA
//     pub struct Flips: u8 {
//         /// Render the sprite as it is
//         const None = 0;
//         /// Render the sprite reversed along the X axis
//         const FlipH = 1;
//         /// Render the sprite reversed along the Y axis
//         const FlipV = 2;
//         const FlipHV = 3;
//     }
// }

/// Round or not
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DrawPolicy {
    pub do_round: bool,
    // is_batching_disabled: bool,
}

/// Normalized x offsets at top-left, top-right, bottom-left, bottom-right
const CORNER_OFFSET_X: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

/// Normalized y offsets at top-left, top-right, bottom-left, bottom-right
const CORNER_OFFSET_Y: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Rect2u {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

/// Top-left and bottom-right
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Skew2f {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

/// Top-left and bottom-right
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Rot2f {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

// TODO: what is depth. is it considered by Device?

/// Data to push a sprite into `SpriteBatch`
///
/// * `origin`: in pixels
/// * `src_rect`: in pixels
/// * `dest_rect`: in pixels or normalized
/// * `is_dest_size_in_pixels`:
///   If false, `src_rect` is assumed to have been normaliezd
#[derive(Debug)]
pub struct SpritePush {
    pub src_rect: Rect2f,
    pub dest_rect: Rect2f,
    pub color: fna3d::Color,
    pub origin: Vec2f,
    pub rot: f32,
    pub depth: f32,
    pub effects: u8, // TODO: isn't it `SpriteEffects` (flips)?
    pub is_dest_size_in_pixels: bool,
    pub skew: Skew2f,
}

impl Default for SpritePush {
    fn default() -> Self {
        Self {
            src_rect: Rect2f::normalized(), // set it in pixel
            dest_rect: Rect2f::default(),
            color: fna3d::Color::white(),
            origin: Vec2f::default(),
            rot: 0.0,
            depth: 0.0,
            effects: 0, // TODO: isn't it `SpriteEffects`?
            is_dest_size_in_pixels: true,
            skew: Skew2f::default(),
        }
    }
}

impl SpritePush {
    pub fn reset_to_defaults(&mut self) {
        self.src_rect = Rect2f::normalized();
        self.dest_rect = Rect2f::default();
        self.color = fna3d::Color::white();
        self.origin = Vec2f::default();
        self.rot = 0.0;
        self.depth = 0.0;
        self.effects = 0;
        self.is_dest_size_in_pixels = true;
        self.skew = Skew2f::default();
    }
}

impl SpritePush {
    // TODO: flush batch if nexessary
    pub fn push(
        &mut self,
        batch: &mut SpriteBatch,
        texture: &Texture2D,
        policy: DrawPolicy,
        effects: u8,
    ) {
        let inv_tex_w = 1.0 / texture.w as f32;
        let inv_tex_h = 1.0 / texture.h as f32;

        // let it be normalized
        let uvs = Rect2f {
            x: self.src_rect.x * inv_tex_w,
            y: self.src_rect.y * inv_tex_h,
            w: self.src_rect.w * inv_tex_w,
            h: self.src_rect.h * inv_tex_h,
        };

        self.origin.x = (self.origin.x / uvs.w) * inv_tex_w;
        self.origin.y = (self.origin.y / uvs.h) * inv_tex_h;

        // destination (NOT normalized)
        let dest = {
            let dest_pos = {
                let mut pos = self.dest_rect.left_up();
                if policy.do_round {
                    pos.round();
                }
                pos
            };

            let dest_size = {
                let mut size = self.dest_rect.size();
                if !self.is_dest_size_in_pixels {
                    size.x *= self.src_rect.w;
                    size.y *= self.src_rect.h;
                }
                size
            };

            Rect2f {
                x: dest_pos.x,
                y: dest_pos.y,
                w: dest_size.x,
                h: dest_size.y,
            }
        };

        self::push_quad(
            batch,
            &texture,
            self.origin,
            &uvs,
            &dest,
            &mut self.skew,
            self.color,
            self.rot,
            self.depth,
            effects,
        );
    }
}

// --------------------------------------------------------------------------------
// Core

#[inline]
fn push_quad(
    batch: &mut SpriteBatch,
    texture: &Texture2D,
    origin: Vec2f,    // ??
    uv_rect: &Rect2f, // normalized (uvs, texture coordinates)
    dest: &Rect2f,    // NOT normalized
    skew: &mut Skew2f,
    color: fna3d::Color,
    rot: f32,
    depth: f32,
    effects: u8, // TODO: use enum
) {
    let vertex = &mut batch.vertex_data[batch.n_quads];
    self::set_quad(
        vertex, skew, origin, uv_rect, dest, color, rot, depth, effects,
    );
    // TODO: use Rc?
    batch.texture_track[batch.n_quads] = texture.clone();
    batch.n_quads += 1;
}

/// Sets up four vertices that correspond to a quad (rect)
#[inline]
fn set_quad(
    vertex: &mut QuadData,
    skew: &mut Skew2f,
    origin: Vec2f,    // ??
    uv_rect: &Rect2f, // normalized (uvs, texture coordinates)
    dest: &Rect2f,    // NOT normalized
    color: fna3d::Color,
    rot: f32,
    depth: f32,
    effects: u8, // TODO: use enum
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
    if effects != 0 {
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
        let corner_x = (CORNER_OFFSET_X[i] - origin.x) * dest.w + skew_xs[i];
        let corner_y = (CORNER_OFFSET_Y[i] - origin.y) * dest.h - skew_ys[i];

        vertex[i].dest.x = (rot.x2 * corner_y) + (rot.x1 * corner_x) + dest.x;
        vertex[i].dest.y = (rot.y2 * corner_y) + (rot.y1 * corner_x) + dest.y;
        vertex[i].dest.z = depth;

        // Here, `^` is xor (exclusive or) operator. So if `effects` (actually flips?) equals to
        // zero, it does nothing and `i ^ effects` == `i`
        vertex[i].uvs.x = (CORNER_OFFSET_X[i ^ effects as usize] * uv_rect.w) + uv_rect.x;
        vertex[i].uvs.y = (CORNER_OFFSET_Y[i ^ effects as usize] * uv_rect.h) + uv_rect.y;

        vertex[i].color = color;
    }
}
