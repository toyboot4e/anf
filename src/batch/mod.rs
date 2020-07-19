//! Sprite batch
//!
//! Corresponds to both `GraphicsDevice` and `SpriteBatch`

pub mod batch_data;
mod batch_internals;
pub mod draw;

pub mod batch_push;
pub use batch_push::DrawPolicy;

use std::ffi::c_void;

// TODO: make a more fluent API
// TODO: add begin guard
pub fn push() -> batch_push::SpritePushCommand {
    batch_push::SpritePushCommand::default()
}

/// The main interface to render sprites
///
/// `Batcher` automatically batches pushed sprites when it's possible
///
/// # Immediate mode vs batch mode
pub struct Batcher {
    pub batch: batch_data::BatchData,
    gbuf: draw::GpuBuffer,
    binds: draw::GpuBindings,
    state: draw::GlState,
    is_begin_called: bool,
    win: *mut c_void,
}

impl Batcher {
    pub fn new(device: &mut fna3d::Device, win: *mut c_void) -> Self {
        let decl = batch_internals::VertexData::decl();
        Self {
            batch: batch_data::BatchData::new(),
            gbuf: draw::GpuBuffer::from_device(device),
            binds: draw::GpuBindings::new(decl),
            state: draw::GlState::from_device(device),
            is_begin_called: false,
            win,
        }
    }

    pub fn clear(device: &mut fna3d::Device, clear_color: fna3d::Color) {
        device.clear(fna3d::ClearOptions::Target, clear_color, 0.0, 0);
    }

    pub fn begin(&mut self, device: &mut fna3d::Device) {
        self.is_begin_called = true;
        device.begin_frame();
    }

    pub fn end(&mut self, device: &mut fna3d::Device) {
        if !self.is_begin_called {
            log::warn!("`Batcher::end` is called without begin");
            return;
        }
        self.flush(device);
        device.swap_buffers(None, None, self.win); // `Game.EndDraw` = `GraphicsDevice.Present`
    }

    /// Draws all the pushed sprites
    fn flush(&mut self, device: &mut fna3d::Device) {
        if !self.is_begin_called {
            log::warn!("`Batcher::begin` has to be be called before flushing");
            return;
        }

        if self.batch.n_sprites == 0 {
            return;
        }

        self.prep_render_state(device);

        // set vertex data to the device
        {
            let data = &mut self.batch.vertex_data[0..self.batch.n_sprites];
            let offset = 0 as i32;
            self.gbuf
                .vbuf
                .set_data(device, offset as u32, data, fna3d::SetDataOptions::None);
            self.binds.on_set_vbuf(&mut self.gbuf.vbuf.inner, offset);
        }

        // then actually draw the copied primitives
        self.batch
            .flush(device, &self.gbuf.ibuf, &mut self.binds, &mut self.state);
    }

    /// Does some things before flushing `BatchData`
    ///
    /// 1. apply data that are set with `begin`
    /// 2. set transform matrix
    /// 3. apply effect
    fn prep_render_state(&mut self, device: &mut fna3d::Device) {

        // GraphicsDevice.BlendState = _blendState;
        // GraphicsDevice.SamplerStates[0] = _samplerState;
        // GraphicsDevice.DepthStencilState = _depthStencilState;
        // GraphicsDevice.RasterizerState = _rasterizerState;

        // GraphicsDevice.SetVertexBuffer(_vertexBuffer);
        // GraphicsDevice.Indices = _indexBuffer;

        // var viewport = GraphicsDevice.Viewport;

        // // inlined CreateOrthographicOffCenter

        // _projectionMatrix.M11 = (float)( 2.0 / (double) ( viewport.Width / 2 * 2 - 1 ) );
        // _projectionMatrix.M22 = (float)( -2.0 / (double) ( viewport.Height / 2 * 2 - 1 ) );

        // _projectionMatrix.M41 = -1 - 0.5f * _projectionMatrix.M11;
        // _projectionMatrix.M42 = 1 - 0.5f * _projectionMatrix.M22;

        // Matrix.Multiply(ref _transformMatrix, ref _projectionMatrix, out _matrixTransformMatrix);
        // _spriteEffect.SetMatrixTransform(ref _matrixTransformMatrix);

        // // we have to Apply here because custom effects often wont have a vertex shader and we need the default SpriteEffect's
        // _spriteEffectPass.Apply();
    }
}
