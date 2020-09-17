//! [`DrawContext`] and traits to push sprites
//!
//! Draw calls are automatically batched.
//!
//! # Example
//!
//! [`DrawContext`] → [`BatchPass`] → [`SpritePushCommand`]s:
//!
//! [`BatchPass`]: BatchPass
//! [`SpritePushCommand`]: SpritePushCommand
//!
//! ```no_run
//! use anf::{game::{AnfGameState, Context}, prelude::*};
//! use anf::gfx::TextureData2D;
//! use fna3d::Color;
//!
//! struct SampleState {
//!     tx: TextureData2D,
//! }
//!
//! impl AnfGameState for SampleState {
//!     fn update(&mut self, cx: &mut Context) {}
//!     fn render(&mut self, cx: &mut Context) {
//!         let mut pass = cx.dcx.pass(); // batch pass
//!         pass.texture(&self.tx).dest_pos_px([100.0, 100.0]); // push texture
//!         pass.texture(&self.tx).dest_pos_px([100.0, 400.0]);
//!     }
//! }
//! ```

use std::path::Path;

pub use anf_gfx::cmd::prelude::*;
use anf_gfx::{
    batcher::{bufspecs::ColoredVertexData, Batcher},
    cmd::{QuadPush, QuadPushBinding, SpritePushCommand},
    geom2d::*,
};
use fna3d::{self, Device};
use fna3d_hie::Pipeline;

use crate::game::app::TimeStep;

/// The ANF graphics API
///
/// Drops FNA3D device
#[derive(Debug)]
pub struct DrawContext {
    // states
    batcher: Batcher,
    pipe: Pipeline,
    push: QuadPush,
    /// dependency
    pub(crate) device: Device,
    /// dependency
    pub(crate) params: fna3d::PresentationParameters,
    /// interface
    pub(crate) time_step: TimeStep,
}

impl DrawContext {
    pub fn new(
        mut device: Device,
        default_shader: impl AsRef<Path>,
        params: fna3d::PresentationParameters,
    ) -> Self {
        let pipe = Pipeline::new(&mut device, ColoredVertexData::decl(), default_shader);
        let batcher = Batcher::from_device(&mut device);
        Self {
            device,
            batcher,
            pipe,
            push: QuadPush::default(),
            params,
            time_step: TimeStep::default(),
        }
    }
}

impl AsMut<fna3d::Device> for DrawContext {
    fn as_mut(&mut self) -> &mut fna3d::Device {
        &mut self.device
    }
}

impl DrawContext {
    /// Begins a batch pass, rendering with particular set of state
    pub fn pass(&mut self) -> BatchPass<'_> {
        BatchPass::new(self)
    }

    pub fn screen(&self) -> Rect2f {
        [
            0.0,
            0.0,
            self.params.backBufferWidth as f32,
            self.params.backBufferHeight as f32,
        ]
        .into()
    }

    pub fn dt_secs_f32(&self) -> f32 {
        self.time_step.dt_secs_f32()
    }
}

/// Handle to push sprites
///
/// Binds a set of state for rendering and flushes the [`SpriteBatch`] when it goes out of scope.
/// "Batch pass" is not a common word but I think it makes sence.
///
/// Currently it doesn't handle those state such as render taret.
///
/// [`SpriteBatch`]: anf_gfx::batcher::batch::SpriteBatch
pub struct BatchPass<'a> {
    dcx: &'a mut DrawContext,
}

/// Flush batch when it goes out of scope
impl<'a> Drop for BatchPass<'a> {
    fn drop(&mut self) {
        self.dcx
            .batcher
            .end(&mut self.dcx.device, &mut self.dcx.pipe);
    }
}

impl<'a> BatchPass<'a> {
    pub fn new(dcx: &'a mut DrawContext) -> Self {
        dcx.batcher.begin();
        Self { dcx }
    }

    pub fn texture<T: SubTexture2D>(&mut self, texture: T) -> SpritePushCommand<'_, T> {
        self.dcx.push.reset_to_defaults();
        let quad = QuadPushBinding {
            push: &mut self.dcx.push,
            batch: &mut self.dcx.batcher.batch,
        };
        SpritePushCommand::from_sub_texture(quad, texture)
    }

    pub fn sprite<T: Sprite>(&mut self, sprite: T) -> SpritePushCommand<'_, T> {
        self.dcx.push.reset_to_defaults();
        let quad = QuadPushBinding {
            push: &mut self.dcx.push,
            batch: &mut self.dcx.batcher.batch,
        };
        SpritePushCommand::from_sprite(quad, sprite)
    }
}
