//! Re-exported to super module

use crate::{
    batcher::{
        batch::{SpriteBatch, SpriteDrawCall},
        bufspecs::GpuViBuffer,
    },
    fna3d_hie::{buffers::GpuIndexBuffer, Pipeline},
};

/// [`SpriteBatch`] with GPU vertex/index buffer handle
#[derive(Debug)]
pub struct Batcher {
    pub batch: SpriteBatch,
    bufs: GpuViBuffer,
    is_begin_called: bool,
}

impl Batcher {
    pub fn from_device(device: &mut fna3d::Device) -> Self {
        Self {
            batch: SpriteBatch::new(),
            bufs: GpuViBuffer::from_device(device),
            is_begin_called: false,
        }
    }
}

/// Batch cycle
/// ---
impl Batcher {
    /// Accessor to `Batcher` would like this marking method
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

    /// Draws all the pushed sprites
    fn flush(&mut self, device: &mut fna3d::Device, pipe: &mut Pipeline) {
        // guard
        if !self.is_begin_called {
            log::warn!("`Batcher::flush` was called before begin");
            return;
        }
        self.is_begin_called = false;
        if self.batch.n_quads == 0 {
            return;
        }

        self.flush_impl(device, pipe);
    }

    fn flush_impl(&mut self, device: &mut fna3d::Device, pipe: &mut Pipeline) {
        // blend, sampler, depth/stencil, rasterizer
        // viewport, scissors rect

        // pipe.shader.apply_uniforms();
        pipe.apply_effect(device, 0); // `FNA3D_ApplyEffect`
        self.upload_vertices(device); // `FNA3D_SetVertexBufferData`
        for call in self.batch.iter() {
            Self::make_draw_call(device, pipe, &mut self.bufs, call);
        }

        self.batch.n_quads = 0;
    }
}

/// Sub procedures of [`Batcher::flush`]
/// ---
impl Batcher {
    /// Copies vertex data from CPU to GPU ([`SpriteBatch::vertex_data`] to [`VertexBuffer`])
    fn upload_vertices(&mut self, device: &mut fna3d::Device) {
        let offset = 0;
        let data = &mut self.batch.quads[0..self.batch.n_quads];
        self.bufs
            .vbuf
            .upload_vertices(device, offset, data, fna3d::SetDataOptions::None);
    }

    /// Runs [`SpriteDrawCall`] got from [`SpriteBatch`]
    fn make_draw_call(
        device: &mut fna3d::Device,
        pipe: &mut Pipeline,
        bufs: &mut GpuViBuffer,
        call: SpriteDrawCall<'_>,
    ) {
        pipe.set_texture_raw(device, call.texture());
        pipe.reset_vertex_attributes(&mut bufs.vbuf.inner, 0);
        pipe.upload_vertex_attributes(device, call.base_vertex() as u32);
        self::draw_triangles(device, call, &bufs.ibuf);
    }
}

fn draw_triangles(device: &mut fna3d::Device, call: SpriteDrawCall<'_>, ibuf: &GpuIndexBuffer) {
    device.draw_indexed_primitives(
        fna3d::PrimitiveType::TriangleList,
        call.base_vertex() as u32, // the number of vertices to skip
        call.base_index() as u32, // REMARK: our index buffer is cyclic and we don't need to actually calculate it
        call.n_primitives() as u32,
        ibuf.raw(),
        ibuf.elem_size(),
    );
}
