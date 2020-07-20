//! Re-exported to `batcher` module

use crate::gfx::{
    batch::{batch_data, batch_internals},
    batcher::{
        buffers::{GlState, VBinds, ViBuffers},
        shader,
    },
};
use std::ffi::c_void;

/// The main interface for users to render 2D sprites
///
/// `Batcher` automatically batches pushed sprites when it flushes. Rendering cycle would be
/// `anf::gfx::begin_frame`, `batcher::begin`, `anf::gfx::push`, `batcher::end` and
/// `anf::gfx::end_framme`
///
/// # Immediate mode vs batch mode
pub struct Batcher {
    pub batch: batch_data::BatchData,
    is_begin_called: bool,
    // --- rendering pipeline ---
    bufs: ViBuffers,
    pub(crate) v_binds: VBinds,
    state: GlState,
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
            bufs: ViBuffers::from_device(device),
            v_binds: VBinds::new(decl),
            state: GlState::from_device(device),
            shader,
            win,
        }
    }
}

/// Batch cycle
/// ---
impl Batcher {
    /// Begin pass
    pub fn begin(&mut self, device: &mut fna3d::Device) {
        self.flush_prep_render_state(device); // FIXME: should it be called..?
        self.is_begin_called = true;
    }

    /// End pass
    pub fn end(&mut self, device: &mut fna3d::Device) {
        if !self.is_begin_called {
            log::warn!("`Batcher::end` is called before `begin`");
            return;
        }
        self.flush(device);
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

        self.flush_prep_render_state(device);
        // FNA3D_ApplyEffect (this is a required rendering pipeline)
        self.shader.apply_effect(device, 0);
        // FNA3D_SetVertexData
        self.flush_set_vertex(device);
        // FNA3D_VerifySamplerState, FNA3D_VerifyVertexSamplerState, FNA3D_ApplyVertexBufferBindings, FNA3D_DrawIndexedPrimitives
        self.flush_draw(device);

        self.batch.n_sprites = 0;
    }
}

/// Sub procedures of `flush`
impl Batcher {
    /// Does some things before flushing `BatchData`
    ///
    /// 1. push data set with `begin`
    /// 2. set transform matrix to effect
    fn flush_prep_render_state(&mut self, device: &mut fna3d::Device) {
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
    }

    /// Sets vertex data to `fna3d::Device` and maybe updates `VertexBufferBindings`
    #[inline]
    fn flush_set_vertex(&mut self, device: &mut fna3d::Device) {
        let data = &mut self.batch.vertex_data[0..self.batch.n_sprites];
        let offset = 0 as i32;
        // FNA3D_SetVertexBufferData
        self.bufs
            .vbuf
            .set_data(device, offset as u32, data, fna3d::SetDataOptions::None);
        // update vertex bindings
        self.v_binds.update(&mut self.bufs.vbuf.inner, offset);
    }

    /// Makes draw calls
    ///
    /// Vertex data is already set before is functions
    #[inline]
    fn flush_draw(&mut self, device: &mut fna3d::Device) {
        let mut iter = batch_data::BatchSpanIter::new();
        while let Some(span) = iter.next(&mut self.batch) {
            self.make_draw_call(device, span);
        }
    }

    /// Makes a draw call
    #[inline]
    fn make_draw_call(&mut self, device: &mut fna3d::Device, span: batch_data::BatchSpan) {
        log::trace!(
            "draw texture {}, {:?} at {:#?}",
            self.batch.n_sprites,
            &self.batch.texture_info[span.offset],
            &self.batch.vertex_data[span.offset..(span.offset + span.len)]
        );

        // update sampler state
        // TODO: only when it's necessary (like when making a texture)
        self.state
            .set_texture(device, &self.batch.texture_info[span.offset]);

        // "the very last thing to call before making a draw call"
        // (`GraphicsDevice.PrepareVertexBindingArray` > `FNA3D_SetVertexBufferBindings`)
        let vertex_offset = span.offset * 4;
        self.v_binds
            .apply_vertex_buffer_bindings(device, vertex_offset as i32);

        // make a draw call
        device.draw_indexed_primitives(
            fna3d::PrimitiveType::TriangleList,
            span.offset as u32, // the number of vertices to skip
            0,                  // the number of indices to skip.
            // base_offset * 6, // our index buffer is cyclic and we don't need to actually calculate it
            span.len as u32 * 2, // the number of triangles to draw
            self.bufs.ibuf.raw(),
            self.bufs.ibuf.elem_size(),
        );
    }
}
