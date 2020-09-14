//! Graphics, the sprite rendering API

pub use anf_gfx::{
    geom,
    texture::{SpriteData, SubTextureData2D, TextureData2D},
};

use api::DrawContext;

/// Clears the frame buffer, that is, the screen
pub fn clear_frame(dcx: &mut DrawContext, clear_color: fna3d::Color) {
    dcx.as_mut()
        .clear(fna3d::ClearOptions::TARGET, clear_color, 0.0, 0);
}

pub mod api {
    //! [`DrawContext`] and traits to push sprites. Re-exported to [`crate::prelude`]
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
    //! use anf::prelude::*;
    //! use anf::gfx::TextureData2D;
    //! use fna3d::Color;
    //!
    //! struct SampleState {
    //!     tx: TextureData2D,
    //! }
    //!
    //! impl AnfGame for SampleState {
    //!     fn render(&mut self, ts: TimeStep, dcx: &mut DrawContext) {
    //!         anf::gfx::clear_frame(dcx, Color::cornflower_blue());
    //!         let mut pass = dcx.pass(); // batch pass
    //!         pass.cmd().dest_pos_px([100.0, 100.0]).texture(&self.tx); // push texture
    //!         pass.cmd().dest_pos_px([100.0, 400.0]).texture(&self.tx);
    //!     }
    //! }
    //! ```

    pub use anf_gfx::cmd::prelude::*;

    use anf_gfx::{
        batcher::{bufspecs::ColoredVertexData, Batcher},
        cmd::{QuadPush, SpritePushCommand},
        geom::*,
    };

    use fna3d::{self, Device};
    use fna3d_hie::Pipeline;

    use std::path::Path;

    /// The ANF graphics API
    ///
    /// * TODO: drop `Device`
    pub struct DrawContext {
        /// Use `as_mut` to get access
        device: Device,
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
}
