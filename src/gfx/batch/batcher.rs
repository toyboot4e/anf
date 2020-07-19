use crate::gfx::batch::{batch_data, batch_internals, draw, shader};
use std::ffi::c_void;

/// The main interface to render sprites
///
/// `Batcher` automatically batches pushed sprites when it flushes
///
/// # Immediate mode vs batch mode
pub struct Batcher {
    pub batch: batch_data::BatchData,
    is_begin_called: bool,
    // --- rendering pipeline ---
    gbuf: draw::GpuBuffer,
    binds: draw::GpuBindings,
    state: draw::GlState,
    shader: shader::Shader,
    pub(crate) win: *mut c_void,
}

impl Batcher {
    pub fn new(device: &mut fna3d::Device, win: *mut c_void) -> Self {
        let decl = batch_internals::VertexData::decl();
        let shader = shader::Shader::from_device(device).expect("failed to make default shader");
        Self {
            batch: batch_data::BatchData::new(),
            is_begin_called: false,
            gbuf: draw::GpuBuffer::from_device(device),
            binds: draw::GpuBindings::new(decl),
            state: draw::GlState::from_device(device),
            shader,
            win,
        }
    }

    /// Begin pass
    pub fn begin(&mut self, device: &mut fna3d::Device) {
        self.prep_render_state(device); // FIXME: should it be called..?
        self.is_begin_called = true;
    }

    /// End pass
    pub fn end(&mut self, device: &mut fna3d::Device) {
        if !self.is_begin_called {
            log::warn!("`Batcher::end` is called before `begin`");
            return;
        }
        self.flush(device);
        // `Game.EndDraw` = `GraphicsDevice.Present`
    }

    /// Draws all the pushed sprites
    pub(crate) fn flush(&mut self, device: &mut fna3d::Device) {
        if !self.is_begin_called {
            log::warn!("`Batcher::begin` has to be be called before flushing");
            return;
        }

        if self.batch.n_sprites == 0 {
            return;
        }

        self.prep_render_state(device);
        self.flush_set_vertex(device);
        self.flush_draw(device);

        self.batch.n_sprites = 0;
    }
}

impl Batcher {
    #[inline]
    fn flush_set_vertex(&mut self, device: &mut fna3d::Device) {
        let data = &mut self.batch.vertex_data[0..self.batch.n_sprites];
        let offset = 0 as i32;
        self.gbuf
            .vbuf
            .set_data(device, offset as u32, data, fna3d::SetDataOptions::None);
        // apply sampler state change
        self.binds.on_set_vbuf(&mut self.gbuf.vbuf.inner, offset);
    }

    #[inline]
    fn flush_draw(&mut self, device: &mut fna3d::Device) {
        let mut iter = batch_data::BatchSpanIter::new();
        while let Some(span) = iter.next(&mut self.batch) {
            log::trace!(
                "draw texture {}, {:?} at {:#?}",
                self.batch.n_sprites,
                &self.batch.texture_info[span.offset],
                &self.batch.vertex_data[span.offset..(span.offset + span.len)]
            );
            draw::draw_indexed_primitives(
                device,
                &self.gbuf.ibuf,
                &mut self.binds,
                &mut self.state,
                &self.batch.texture_info[span.offset],
                span.offset as u32,
                span.len as u32,
            );
        }
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
        self.shader.apply(device, 0);
    }
}
