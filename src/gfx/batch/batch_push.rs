//! Push command to `BatchData`
//!
//! Actually the internal implementation is based on `Batcher` in Nez

use crate::gfx::{
    batch::{batch_data::BatchData, batch_internals::*},
    texture::Texture2D,
};

bitflags::bitflags! {
    /// `SpriteEffects` in FNA
    pub struct Flips: u8 {
        /// Render the sprite as it is
        const None = 0;
        /// Render the sprite reversed along the X axis
        const FlipH = 1;
        /// Render the sprite reversed along the Y axis
        const FlipV = 2;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DrawPolicy {
    pub do_round: bool,
    // is_batching_disabled: bool,
}

/// Normalized x offsets at top-left, top-right, bottom-left, bottom-right
const CORNER_OFFSET_X: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

/// Normalized y offsets at top-left, top-right, bottom-left, bottom-right
const CORNER_OFFSET_Y: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

// TODO: what is depth. is it considered by Device

/// How to push a sprite into `BatchData`
///
/// A sprite is a rectangle that is represented as two triangles i.e. four vertices and six
/// indices
///
/// * `src_rect`: in pixels
/// * `dest_rect`: in pixels or normalized
///
/// * `is_dest_size_in_pixels`:
///   If false, `src_rect` is assumed to be normaliezd
#[derive(Debug)]
pub struct SpritePushCommand {
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

impl Default for SpritePushCommand {
    fn default() -> Self {
        let color = fna3d::Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        };
        Self {
            src_rect: Rect2f::normalized(), // set it in pixel
            dest_rect: Rect2f::default(),
            color,
            origin: Vec2f::default(),
            rot: 0f32,
            depth: 0f32,
            effects: 0, // TODO: isn't it `SpriteEffects`?
            is_dest_size_in_pixels: true,
            skew: Skew2f::default(),
        }
    }
}

// TODO: extract builder

/// Builder methods
/// ---
impl SpritePushCommand {
    /// In pixels
    pub fn src_rect(&mut self, x: f32, y: f32, w: f32, h: f32) {
        self.src_rect = Rect2f { x, y, w, h };
    }

    pub fn dest_pos(&mut self, x: f32, y: f32) {
        self.dest_rect.x = x;
        self.dest_rect.y = y;
    }

    pub fn dest_size(&mut self, w: f32, h: f32) {
        self.dest_rect.w = w;
        self.dest_rect.h = h;
    }

    pub fn dest_rect(&mut self, xs: impl Into<[f32; 4]>) {
        let xs = xs.into();
        self.dest_rect = Rect2f {
            x: xs[0],
            y: xs[1],
            w: xs[2],
            h: xs[3],
        };
    }
}

impl SpritePushCommand {
    pub fn run(self, batch: &mut BatchData, texture: &Texture2D, policy: DrawPolicy, effects: u8) {
        let inv_tex_w = 1.0 / texture.w as f32;
        let inv_tex_h = 1.0 / texture.h as f32;
        self.push_vertices(batch, inv_tex_w, inv_tex_h, policy, effects);
        batch.texture_info[batch.n_sprites] = texture.clone(); // TODO: use Rc
        batch.n_sprites += 1;
    }

    #[inline]
    fn push_vertices(
        mut self,
        batch: &mut BatchData,
        inv_tex_w: f32,
        inv_tex_h: f32,
        policy: DrawPolicy,
        effects: u8,
    ) {
        // TODO: overwriting fields vs use local variables

        // it's in pixels
        let src_rect = Rect2f {
            x: self.src_rect.x * inv_tex_w,
            y: self.src_rect.y * inv_tex_h,
            w: self.src_rect.w * inv_tex_w,
            h: self.src_rect.h * inv_tex_h,
        };

        self.origin.x = (self.origin.x / src_rect.w) * inv_tex_w;
        self.origin.y = (self.origin.y / src_rect.h) * inv_tex_h;

        let dest_pos = {
            let mut pos = self.src_rect.left_up();
            if policy.do_round {
                pos.round();
            }
            pos
        };

        let dest_size = {
            // TODO: should I round size
            let mut size = self.dest_rect.size();
            if !self.is_dest_size_in_pixels {
                size.x *= src_rect.w;
                size.y *= src_rect.h;
            }
            size
        };

        // rotation matrix
        let (rot_1x, rot_1y, rot_2x, rot_2y) = if self.rot <= f32::EPSILON {
            let sin = self.rot.sin();
            let cos = self.rot.cos();
            (cos, sin, -sin, cos)
        } else {
            (1.0, 0.0, 0.0, 1.0)
        };

        // flip our skew values if we have a flipped sprite
        if self.effects != 0 {
            self.skew.y1 *= -1 as f32;
            self.skew.y2 *= -1 as f32;
            self.skew.x1 *= -1 as f32;
            self.skew.x2 *= -1 as f32;
        }

        let vertex = &mut batch.vertex_data[batch.n_sprites];

        // top, top, bottom, bottom
        let skew_xs = [self.skew.x1, self.skew.x1, self.skew.x2, self.skew.x2];
        // left, right, right, left
        let skew_ys = [self.skew.y1, self.skew.y2, self.skew.y1, self.skew.y2];

        // push four vertices: top-left, top-right, bottom-left, and bottom-right, respectively
        for i in 0..4 {
            let corner_x = (CORNER_OFFSET_X[i] - self.origin.x) * dest_size.x + skew_xs[i];
            let corner_y = (CORNER_OFFSET_Y[i] - self.origin.y) * dest_size.y - skew_ys[i];

            vertex[i].dest.x = (rot_2x * corner_y) + (rot_1x * corner_x) + dest_pos.x;
            vertex[i].dest.y = (rot_2y * corner_y) + (rot_1y * corner_x) + dest_pos.y;
            vertex[i].dest.z = self.depth;

            // Here, `^` is xor (exclusive or) operator.
            // So if `effects` (actually flips?) equals to zero, it does nothing and
            // `i ^ effects` == `i`
            vertex[i].uvs.x = (CORNER_OFFSET_X[i ^ effects as usize] * src_rect.w) + src_rect.x;
            vertex[i].uvs.y = (CORNER_OFFSET_Y[i ^ effects as usize] * src_rect.h) + src_rect.y;
            vertex[i].color = self.color;
        }
    }
}
