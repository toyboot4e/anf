//! 2D quad rendering
//!
//! # Example
//!
//! [`DrawContext`] provides with XNA-like interface:
//!
//! ```no_run
//! use anf::gfx::{DrawContext, Texture2D};
//! use std::path::Path;
//!
//! fn load_texture_and_draw_it(dcx: &mut DrawContext, tx: &Texture2D) {
//!     dcx.begin();
//!     dcx.cmd().dest_pos_px(100.0, 100.0).push_tx(&tx);
//!     dcx.end();
//! }
//! ```
//!
//! # Names
//!
//! ANF recommends using shorter names:
//!
//! * `dcx` referring to [`DrawContext`] (following the rustc naming [convension])
//! * `tx` referring to [`Texture2D`]
//! * `px` referring to "pixels"
//! * `cmd` referring to "command"
//!
//! # TODOs
//!
//! * TODO: asset management in example
//!
//! [`DrawContext`]: ./struct.DrawContext.html
//! [`Texture2D`]: ./struct.Texture2D.html
//! [convension]: https://rustc-dev-guide.rust-lang.org/conventions.html#naming-conventions

pub mod batcher;
pub mod buffers;
pub mod pipeline;

mod texture;
pub use texture::Texture2D;

use batcher::{primitives::*, Batcher, DrawPolicy, SpritePush};
use fna3d::Device;
use pipeline::Pipeline;

/// Render sprites!
///
/// * TODO: drop `Device`
/// * TODO: better push API
pub struct DrawContext {
    pub(crate) device: Device,
    batcher: Batcher,
    pipe: Pipeline,
    push: SpritePush,
}

impl DrawContext {
    pub fn new(device: Device, batcher: Batcher, pipe: Pipeline) -> Self {
        Self {
            device,
            batcher,
            pipe,
            push: SpritePush::default(),
        }
    }
}

impl DrawContext {
    /// Begins a apss
    pub fn begin(&mut self) {
        self.batcher.begin(&mut self.device);
    }

    /// Ends the pass and flushes batch
    ///
    /// Then the data is actually drawn to a render target.
    pub fn end(&mut self) {
        self.batcher.end(&mut self.device, &mut self.pipe);
    }

    /// `SpritePushCommand`
    pub fn cmd(&mut self) -> SpritePushCommand<'_> {
        self.batcher.begin(&mut self.device);
        self.push.reset_to_defaults();

        SpritePushCommand {
            dcx: self,
            policy: DrawPolicy { do_round: false },
            effects: 0,
        }
    }
}

/// Interface to push sprites
pub struct SpritePushCommand<'a> {
    dcx: &'a mut DrawContext,
    policy: DrawPolicy,
    effects: u8,
}

impl<'a> SpritePushCommand<'a> {
    pub fn push_tx(&mut self, texture: &Texture2D) {
        self.dcx.push.push(
            &mut self.dcx.batcher.batch,
            texture,
            self.policy,
            self.effects,
        );
    }
}

/// Builder
impl<'a> SpritePushCommand<'a> {
    #[inline]
    fn data(&mut self) -> &mut SpritePush {
        &mut self.dcx.push
    }

    /// In pixels (automatically normalized)
    pub fn src_rect(&mut self, x: f32, y: f32, w: f32, h: f32) -> &mut Self {
        self.data().src_rect = Rect2f { x, y, w, h };
        self
    }

    pub fn dest_pos_px(&mut self, x: f32, y: f32) -> &mut Self {
        let data = self.data();
        data.dest_rect.x = x;
        data.dest_rect.y = y;

        self
    }

    pub fn dest_size_px(&mut self, w: f32, h: f32) -> &mut Self {
        let data = self.data();
        data.is_dest_size_in_pixels = true;
        data.dest_rect.w = w;
        data.dest_rect.h = h;

        self
    }

    pub fn dest_rect_px(&mut self, xs: impl Into<[f32; 4]>) -> &mut Self {
        let xs = xs.into();

        let data = self.data();
        data.is_dest_size_in_pixels = true;
        data.dest_rect = Rect2f {
            x: xs[0],
            y: xs[1],
            w: xs[2],
            h: xs[3],
        };

        self
    }

    pub fn color(&mut self, color: fna3d::Color) -> &mut Self {
        self.data().color = color;
        self
    }
}
