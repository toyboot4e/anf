//! `Batcher`. Re-exported to the super module

use crate::gfx::{
    batcher::{
        batch_data::{self, batch_internals},
        buffers::ViBuffers,
    },
    Pipeline,
};

/// Batches draw calls to graphics card as much as possible
#[derive(Debug)]
pub struct Batcher {
    pub batch: batch_data::BatchData,
    is_begin_called: bool,
    bufs: ViBuffers,
}

// TODO: draw sprites.. why does it not work? I'm in hell

impl Batcher {
    pub fn from_device(device: &mut fna3d::Device) -> Self {
        let decl = batch_internals::ColoredVertexData::decl();
        Self {
            batch: batch_data::BatchData::new(),
            is_begin_called: false,
            bufs: ViBuffers::from_device(device),
        }
    }
}

/// Batch cycle
/// ---
impl Batcher {
    /// Begins a pass
    pub fn begin(&mut self, device: &mut fna3d::Device) {
        self.is_begin_called = true;
    }

    // TODO: begin_with_target

    /// Ends the pass and flushes batch data to actually draw to a render target
    pub fn end(&mut self, device: &mut fna3d::Device, p: &mut Pipeline) {
        if !self.is_begin_called {
            log::warn!("`Batcher::end` is called before `begin`");
            return;
        }
        self.flush(device, p);
    }

    /// Draws all the pushed sprites.
    ///
    /// This is public only for documentation purpose; this function should only be called from
    /// `end`, `anf::gfx::end_frame` or when pushing vertices and they saturate (out of capaicty of
    /// `BatchData`).
    ///
    /// Contains the following rendering pipeline calls:
    ///
    /// * `FNA3D_ApplyEffect` (`Pipeline::apply_effect`):
    ///   Yes a shader is required even if we do nothing with it.
    /// * `FNA3D_SetVertexData` (`IndexBuffer::set_data`):
    ///   Sets our `VertexData` to `VertexBuffer`
    /// * `FNA3D_VerifySamplerState`, `FNA3D_VerifyVertexSamplerState` (`Pipeline::set_texture`):
    ///   Sets `SamplerState` (`Texture` etc.) to `Device`
    /// * `FNA3D_ApplyVertexBufferBindings` (`Pipeline::apply_vertex_buffer_bindings`):
    ///   Sets our `VertexBuffer` to `fna3d::Device` and prepares shader program ("the last thing to do" before drawing).
    /// * `FNA3D_DrawIndexedPrimitives`:
    ///   Finally draw rectangle sprites (quads) as primitives (triangles)
    pub fn flush(&mut self, device: &mut fna3d::Device, pipe: &mut Pipeline) {
        // FIXME: `flush` can be called if it's not begun (in end_frame)
        if !self.is_begin_called {
            log::warn!("`Batcher::flush` has to be called after `begin`");
            return;
        }
        self.is_begin_called = false;

        if self.batch.n_quads == 0 {
            return;
        }

        self.flush_prep_render_state(device, pipe);

        // FNA3D_ApplyEffect (this is required)
        pipe.apply_effect(device, 0);
        // FNA3D_SetVertexData (copies vertex data from `BatchData` to `VertexBuffer`)
        self.flush_set_vertex(device);
        // FNA3D_VerifySamplerState, FNA3D_VerifyVertexSamplerState,
        // FNA3D_ApplyVertexBufferBindings (slices `VertexBuffer` to `VertexBufferBinding`)
        // FNA3D_DrawIndexedPrimitives
        self.flush_draw(device, pipe);

        self.batch.n_quads = 0;
    }
}

/// Sub procedures of `flush`
impl Batcher {
    #[inline]
    fn flush_prep_render_state(&mut self, device: &mut fna3d::Device, pipe: &mut Pipeline) {
        // GraphicsDevice.BlendState = _blendState;
        // GraphicsDevice.SamplerStates[0] = _samplerState;
        // GraphicsDevice.DepthStencilState = _depthStencilState;
        // GraphicsDevice.RasterizerState = _rasterizerState;

        // GraphicsDevice.SetVertexBuffer(_vertexBuffer);
        // GraphicsDevice.Indices = _indexBuffer;

        // var viewport = GraphicsDevice.Viewport;

        pipe.update_shader();
        // inlined CreateOrthographicOffCenter
        // TODO: set transform
    }

    /// Copies vertex data from CPU to GPU (`BatchData` to `VertexBuffer`)
    #[inline]
    fn flush_set_vertex(&mut self, device: &mut fna3d::Device) {
        let data = &mut self.batch.vertex_data[0..self.batch.n_quads];
        let offset = 0 as i32;
        // FNA3D_SetVertexBufferData
        self.bufs
            .vbuf
            .set_data(device, offset as u32, data, fna3d::SetDataOptions::None);
    }

    /// Makes draw calls
    ///
    /// Vertex data is already set before is functions
    #[inline]
    fn flush_draw(&mut self, device: &mut fna3d::Device, pipe: &mut Pipeline) {
        let mut iter = batch_data::BatchSpanIter::new();
        while let Some((slot, span)) = iter.next(&mut self.batch) {
            self.make_draw_call(device, pipe, slot, span);
        }
    }

    /// Makes a draw call over a sprite batch
    ///
    /// Corresponds to `GraphicsDevice.DrawIndexedPrimitives`
    ///
    /// Calls these methods:
    ///
    /// * `FNA3D_VerifySamplerState`
    /// * `FNA3D_VerifyVertexSamplerState`
    /// * `FNA3D_ApplyVertexBufferBindings`
    /// * `FNA3D_DrawIndexedPrimitives`
    #[inline]
    fn make_draw_call(
        &mut self,
        device: &mut fna3d::Device,
        pipe: &mut Pipeline,
        slot: usize,
        span: batch_data::BatchSpan,
    ) {
        log::trace!(
            "draw call with {:?} with {:?} with vertices\n{:#?}",
            span,
            &self.batch.texture_slots[slot],
            &self.batch.vertex_data[span.lo..span.hi]
        );

        // ----------------------------------------
        // GraphicsDevice.ApplyState

        // update sampler state
        // TODO: call it only when it's necessary (like when making a texture)
        // TODO: Material (BlendState, depth stencil state and rasterizer state)
        pipe.set_texture(device, &self.batch.texture_slots[slot]);

        // ----------------------------------------
        // GraphicsDevice.PrepareVertexBindingArray

        // update vertex bindings
        pipe.rebind_vertex_buffer(&mut self.bufs.vbuf.inner, 0);

        // "the very last thing to call before making a draw call"
        // (`GraphicsDevice.PrepareVertexBindingArray` > `FNA3D_SetVertexBufferBindings`)
        let base_vertex = span.lo * 4;
        pipe.apply_vertex_buffer_bindings(device, base_vertex as i32);

        // ----------------------------------------
        // Make a draw call
        //
        // TODO: using draw_primitives?

        let n_primitives = span.len() as u32 * 2;
        device.draw_indexed_primitives(
            fna3d::PrimitiveType::TriangleList,
            // FIXME: is this OK?
            base_vertex as u32, // the number of vertices to skip
            span.lo as u32 * 6, // our index buffer is cyclic and we don't need to actually calculate it
            n_primitives,
            self.bufs.ibuf.raw(),
            self.bufs.ibuf.elem_size(),
        );
    }
}
