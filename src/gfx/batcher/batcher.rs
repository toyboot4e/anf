//! Re-exported to the super module

use crate::gfx::batcher::{
    batch::{batch_data, batch_internals},
    buffers::{GlState, VBinds, ViBuffers},
    shader,
};
use std::ffi::c_void;

/// The main interface for users to render 2D sprites
///
/// `Batcher` automatically batches draw calls of pushed sprites when it flushes.
///
/// # Immediate mode vs batch mode
///
/// TODO: discuss. Probablly we always can prefer batch mode
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

    /// Draws all the pushed sprites.
    ///
    /// This is public only for documentation purpose; this functions should only be called from
    /// `anf::gfx::end_frame` or when pushing vertices and they saturate (out of capaicty of
    /// `BatchData`).
    ///
    /// Contains the most of the rendering cycle of FNA3D:
    ///
    /// * `FNA3D_ApplyEffect`:
    ///   Yes shaders are required even if we do nothing with it.
    /// * `FNA3D_SetVertexData`:
    ///   Sets our `VertexData` to `VertexBuffer`
    /// * `FNA3D_VerifySamplerState`, `FNA3D_VerifyVertexSamplerState`:
    ///   Run only when necessary (e.g. when the texture changes)
    /// * `FNA3D_ApplyVertexBufferBindings`:
    ///   Prepares shader program ("the last thing to do" before drawing).
    /// * `FNA3D_DrawIndexedPrimitives`:
    ///   Finally draws primitives (triangles)
    pub fn flush(&mut self, device: &mut fna3d::Device) {
        // FIXME: `flush` can be called if it's not begun (in end_frame)
        if !self.is_begin_called {
            log::warn!("`Batcher::flush` has to be called after `begin`");
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
        // FNA3D_VerifySamplerState, FNA3D_VerifyVertexSamplerState,
        // FNA3D_ApplyVertexBufferBindings, FNA3D_DrawIndexedPrimitives
        self.flush_draw(device);

        self.batch.n_sprites = 0;
    }
}

/// Sub procedures of `flush`
impl Batcher {
    /// Does two things:
    ///
    /// 1. push data set on `begin`
    /// 2. set transform matrix to effect
    #[inline]
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
            "draw call with {} sprites with texture {:?} with vertices\n{:#?}",
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
