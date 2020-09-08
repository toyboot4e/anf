//! Graphics, the 2D sprite rendering API
//!
//! Draw calls are automatically batched.
//!
//! # Example
//!
//! We pull [`SpritePushCommand`] from [`BatchPass`], [`BatchPass`] from [`DrawContext`]:
//!
//! ```no_run
//! use anf::gfx::{prelude::*, Texture2D};
//!
//! fn example_rendering(dcx: &mut DrawContext, tx: &Texture2D) {
//!     let mut pass = dcx.pass(); // batch pass
//!     pass.cmd().dest_pos_px([100.0, 100.0]).texture(tx).run(); // push texture
//!     pass.cmd().dest_pos_px([100.0, 400.0]).texture(tx).run();
//! }
//! ```
//!
//! Other functionalities are performed via `gfx` module functions such as `clear_frame`.
//!
//! [convension]: https://rustc-dev-guide.rust-lang.org/conventions.html#naming-conventions

pub use anf_gfx::texture::{SubTexture2D, Texture2D};

use anf_gfx::{
    batcher::{bufspecs::ColoredVertexData, primitives::*, Batcher},
    cmd::{prelude::*, QuadPush, SpritePushCommand},
};

use fna3d::{self, Device};
use fna3d_hie::Pipeline;

use std::path::Path;

pub mod prelude {
    //! `DrawContext` and traits to push sprites

    pub use crate::gfx::DrawContext;
    pub use anf_gfx::cmd::prelude::*;
}

/// Clears the frame buffer, that is, the screen
pub fn clear_frame(dcx: &mut DrawContext, clear_color: fna3d::Color) {
    dcx.device
        .clear(fna3d::ClearOptions::TARGET, clear_color, 0.0, 0);
}

/// The ANF graphics API
///
/// * TODO: drop `Device`
pub struct DrawContext {
    pub(crate) device: Device, // open for `App`
    batcher: Batcher,
    pipe: Pipeline,
    /// Buffer that reduces allocation
    push: QuadPush,
}

impl DrawContext {
    pub fn new(mut device: Device, default_shader: impl AsRef<Path>) -> Self {
        let pipe = Pipeline::new(&mut device, ColoredVertexData::decl(), default_shader);
        let batcher = Batcher::from_device(&mut device);
        Self {
            device,
            batcher,
            pipe,
            push: QuadPush::default(),
        }
    }
}

impl DrawContext {
    /// Begins a batch pass, rendering with particular set of state
    pub fn pass(&mut self) -> BatchPass<'_> {
        BatchPass::new(self)
    }
}

/// Handle to push sprites
///
/// Binds a set of state for rendering and flushes the [`SpriteBatch`] when it goes out of scope.
/// "Batch pass" is not a common word but I think it makes sence.
///
/// Currently it doesn't handle those state such as render taret.
///
/// [`SpriteBatch`]: anf_gfx::batcher::batch::SpritePush
pub struct BatchPass<'a> {
    dcx: &'a mut DrawContext,
}

impl<'a> BatchPass<'a> {
    pub fn new(dcx: &'a mut DrawContext) -> Self {
        dcx.batcher.begin();
        Self { dcx }
    }

    /// [`QuadPush`] command
    pub fn cmd(&mut self) -> SpritePushCommand<'_> {
        self.dcx.push.reset_to_defaults();

        SpritePushCommand {
            push: &mut self.dcx.push,
            batch: &mut self.dcx.batcher.batch,
            policy: DrawPolicy { do_round: false },
            flips: Flips::NONE,
        }
    }
}

impl<'a> Drop for BatchPass<'a> {
    fn drop(&mut self) {
        self.dcx
            .batcher
            .end(&mut self.dcx.device, &mut self.dcx.pipe);
    }
}
