//! Re-exported to super module

use crate::{
    batcher::{
        batch::{BatchSpan, BatchSpanIter, SpriteBatch},
        bufspecs::ViBuffer,
    },
    pipeline::Pipeline,
    texture::Texture2D,
};

// for documentation (types in scope are automatically linked with [`TypeName`])
#[allow(unused_imports)]
use fna3d::{sys::*, Device};

/// Wrapper of [`SpriteBatch`]
///
/// [`Batcher`] accumulates vertex data and batches draw calls when flushing. Draw call is about
/// calling a drawing function of a low-level graphics API. In our case it is
/// [`FNA3D_DrawIndexedPrimitives`].
#[derive(Debug)]
pub struct Batcher {
    pub batch: SpriteBatch,
    is_begin_called: bool,
    bufs: ViBuffer,
}

impl Batcher {
    pub fn from_device(device: &mut fna3d::Device) -> Self {
        Self {
            batch: SpriteBatch::new(),
            is_begin_called: false,
            bufs: ViBuffer::from_device(device),
        }
    }
}

/// Batch cycle
/// ---
impl Batcher {
    pub fn begin(&mut self) {
        self.is_begin_called = true;
    }

    /// Flushes batch data to actually draw to a render target
    pub fn end(&mut self, device: &mut fna3d::Device, p: &mut Pipeline) {
        if !self.is_begin_called {
            log::warn!("`Batcher::end` is called before `begin`");
            return;
        }
        self.flush(device, p);
    }

    /// Draws all the pushed vertices.
    fn flush(&mut self, device: &mut fna3d::Device, pipe: &mut Pipeline) {
        // FIXME: `flush` can be called if it's not begun (in end_frame)
        if !self.is_begin_called {
            log::warn!("`Batcher::flush` has to be called after `begin`");
            return;
        }
        self.is_begin_called = false;

        if self.batch.n_quads == 0 {
            return;
        }

        // reset shader uniform
        self.flush_prep_render_state(device, pipe);

        // `FNA3D_ApplyEffect`
        pipe.apply_effect(device, 0);

        // `FNA3D_SetVertexData` (copies vertex data from `SpriteBatch` to `VertexBuffer`)
        self.flush_set_vertex(device);

        // `FNA3D_VerifySamplerState`, `FNA3D_VerifyVertexSamplerState`
        // `FNA3D_ApplyVertexBufferBindings` (slices `VertexBuffer` to `VertexBufferBinding`)
        // ad finally `FNA3D_DrawIndexedPrimitives`
        self.flush_draw(device, pipe);

        self.batch.n_quads = 0;
    }
}

/// Sub procedures of [`Batcher::flush`]
impl Batcher {
    #[inline]
    fn flush_prep_render_state(&mut self, _device: &mut fna3d::Device, pipe: &mut Pipeline) {
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

    /// Copies vertex data from CPU to GPU ([`SpriteBatch`] to [`VertexBuffer`])
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
    /// Vertex data is already set before this function call.
    #[inline]
    fn flush_draw(&mut self, device: &mut fna3d::Device, pipe: &mut Pipeline) {
        let mut iter = BatchSpanIter::new();
        while let Some(span) = iter.next(&self.batch) {
            let texture = &self.batch.texture_track[span.lo];
            Self::make_draw_call(device, pipe, &mut self.bufs, &texture, span);
        }
    }

    /// Makes a draw call over a sprite batch
    ///
    /// Calls these methods:
    ///
    /// * [`FNA3D_VerifySamplerState`]
    /// * [`FNA3D_VerifyVertexSamplerState`]
    /// * [`FNA3D_ApplyVertexBufferBindings`]
    /// * [`FNA3D_DrawIndexedPrimitives`]
    #[inline]
    fn make_draw_call(
        device: &mut fna3d::Device,
        pipe: &mut Pipeline,
        bufs: &mut ViBuffer,
        texture: &Texture2D,
        span: BatchSpan,
    ) {
        // update sampler state
        pipe.set_texture(device, texture);

        // update vertex bindings
        pipe.rebind_vertex_buffer(&mut bufs.vbuf.inner, 0);

        // "the very last thing to call before making a draw call"
        let base_vertex = span.lo * 4;
        pipe.apply_vertex_buffer_bindings(device, base_vertex as i32);

        let n_primitives = span.len() as u32 * 2;
        device.draw_indexed_primitives(
            fna3d::PrimitiveType::TriangleList,
            // FIXME: is this OK?
            base_vertex as u32, // the number of vertices to skip
            span.lo as u32 * 6, // our index buffer is cyclic and we don't need to actually calculate it
            n_primitives,
            bufs.ibuf.raw(),
            bufs.ibuf.elem_size(),
        );
    }
}
