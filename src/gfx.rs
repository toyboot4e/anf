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
    //! impl AnfLifecycle for SampleState {
    //!     fn render(&mut self, ts: TimeStep, dcx: &mut DrawContext) {
    //!         anf::gfx::clear_frame(dcx, Color::cornflower_blue());
    //!         let mut pass = dcx.pass(); // batch pass
    //!         pass.cmd().dest_pos_px([100.0, 100.0]).texture(&self.tx); // push texture
    //!         pass.cmd().dest_pos_px([100.0, 400.0]).texture(&self.tx);
    //!     }
    //! }
    //! ```

    use std::path::Path;

    pub use anf_gfx::cmd::prelude::*;
    use anf_gfx::{
        batcher::{bufspecs::ColoredVertexData, Batcher},
        cmd::{QuadPush, QuadPushCommand, SpritePushCommand},
        geom::*,
    };
    use fna3d::{self, Device};
    use fna3d_hie::Pipeline;

    use crate::framework::TimeStep;

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

        pub fn screen_size(&self) -> [u32; 2] {
            [
                self.params.backBufferWidth as u32,
                self.params.backBufferHeight as u32,
            ]
        }

        pub fn screen_size_f32(&self) -> [f32; 2] {
            [
                self.params.backBufferWidth as f32,
                self.params.backBufferHeight as f32,
            ]
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

    impl<'a> BatchPass<'a> {
        pub fn new(dcx: &'a mut DrawContext) -> Self {
            dcx.batcher.begin();
            Self { dcx }
        }

        /// [`QuadPush`] command
        pub fn cmd(&mut self) -> QuadPushCommand<'_> {
            self.dcx.push.reset_to_defaults();

            QuadPushCommand {
                push: &mut self.dcx.push,
                batch: &mut self.dcx.batcher.batch,
                policy: DrawPolicy { do_round: false },
                flips: Flips::NONE,
            }
        }

        pub fn texture<T: SubTexture>(&mut self, texture: T) -> SpritePushCommand<'_, T> {
            self.dcx.push.reset_to_defaults();

            let uv_rect = texture.uv_rect();
            let size = [texture.w(), texture.h()];

            let mut x = SpritePushCommand {
                texture,
                quad: QuadPushCommand {
                    push: &mut self.dcx.push,
                    batch: &mut self.dcx.batcher.batch,
                    policy: DrawPolicy { do_round: false },
                    flips: Flips::NONE,
                },
            };

            x.src_rect_normalized(uv_rect);
            x.dest_size_px(size);
            x
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
