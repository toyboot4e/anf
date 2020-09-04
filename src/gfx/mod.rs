//! 2D quad rendering
//!
//! * `dcx` refers to `DrawContext` (following the rustc naming [convension])
//! * `tx` refers to texture
//!
//! # Example
//!
//! ```
//! use anf::{gfx::{DrawContext, Texture2D}, vfs};
//!
//! fn load_texture_and_draw_it(dcx: &mut DrawContext) {
//!     let tx = Texture::from_path(vfs::path("my_texture.png")).unwrap();
//!     dcx.cmd().dest_pos_px(100, 100).push_tx(&tx);
//! }
//! ```
//!
//! Note that this is memory leak.
//!
//! * TODO: asset management
//! * TODO: consider using a matrix crate e.g. [mint](https://docs.rs/mint/) or glam
//! * TODO: event handling
//!
//! [convension]: https://rustc-dev-guide.rust-lang.org/conventions.html#naming-conventions

pub mod batcher;
pub mod buffers;
pub mod pipeline;

mod texture;
pub use texture::Texture2D;

use batcher::{primitives::*, Batcher, DrawPolicy, SpritePush};
use fna3d::Device;
use pipeline::Pipeline;

/// Render sprites! Often referred to as `dcx`
///
/// The name `dcx` follows the rustc [naming convension] (though I often see `ctx` even in rustc).
///
/// [naming convension]: https://rustc-dev-guide.rust-lang.org/conventions.html#naming-conventions
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

    pub fn batcher(&mut self) -> &mut Batcher {
        &mut self.batcher
    }
}

impl DrawContext {
    pub fn begin(&mut self) {
        self.batcher.begin(&mut self.device);
    }

    /// Ends the pass and flushes batch data to actually draw to a render target
    pub fn end(&mut self) {
        self.batcher.end(&mut self.device, &mut self.pipe);
    }

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

    pub fn dest_pos(&mut self, x: f32, y: f32) -> &mut Self {
        let data = self.data();
        data.dest_rect.x = x;
        data.dest_rect.y = y;

        self
    }

    // TODO: dest_size_normalized
    pub fn dest_size_px(&mut self, w: f32, h: f32) -> &mut Self {
        let data = self.data();
        data.is_dest_size_in_pixels = true;
        data.dest_rect.w = w;
        data.dest_rect.h = h;

        self
    }

    pub fn dest_rect(&mut self, xs: impl Into<[f32; 4]>) -> &mut Self {
        let xs = xs.into();
        self.data().dest_rect = Rect2f {
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
