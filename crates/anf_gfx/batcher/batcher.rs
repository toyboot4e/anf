//! Re-exported to super module

use fna3d_hie::{buffers::GpuIndexBuffer, Pipeline};

use crate::{
    batcher::{
        batch::{SpriteBatch, SpriteDrawCall},
        bufspecs::GpuViBuffer,
    },
    geom3d::{Mat3f, Vec3f},
};

/// [`SpriteBatch`] with GPU vertex/index buffer handle
#[derive(Debug)]
pub struct Batcher {
    pub batch: SpriteBatch,
    bufs: GpuViBuffer,
    is_begin_called: bool,
    /// The projection matrix (orthographic matrix)
    proj_mat: Mat3f,
    /// The transformation matrix
    transform_mat: Mat3f,
    /// The view projection matrix used by vertex shader
    ///
    /// M_v = M_t M_p
    view_proj_mat: Mat3f,
}

impl Batcher {
    pub fn from_device(device: &mut fna3d::Device) -> Self {
        Self {
            batch: SpriteBatch::new(),
            bufs: GpuViBuffer::from_device(device),
            is_begin_called: false,
            proj_mat: Mat3f::orthographic(1.0, 1.0, 1.0, 0.0),
            transform_mat: Mat3f::identity(),
            view_proj_mat: Mat3f::default(),
        }
    }
}

/// Batch cycle
/// ---
impl Batcher {
    /// Accessor to `Batcher` would like this marking method
    pub fn begin(&mut self) {
        // TODO: this is nonsense..
        self.is_begin_called = true;
    }

    /// Flushes batch data to actually draw to a render target
    pub fn end(&mut self, device: &mut fna3d::Device, p: &mut Pipeline) {
        self.flush(device, p);
    }

    pub fn is_satured(&self) -> bool {
        self.batch.is_satured()
    }

    /// Draws all the pushed sprites
    pub fn flush(&mut self, device: &mut fna3d::Device, pipe: &mut Pipeline) {
        // guard
        if !self.is_begin_called {
            log::warn!("`Batcher::flush` was called before begin");
            return;
        }
        self.is_begin_called = false;

        if !self.batch.any_quads_pushed() {
            return;
        }

        self.flush_impl(device, pipe);
    }

    fn flush_impl(&mut self, device: &mut fna3d::Device, pipe: &mut Pipeline) {
        // Material (blend, sampler, depth/stencil, rasterizer)
        // viewport, scissors rect

        // update shader matrix
        // FIXME: get viewport
        self.proj_mat = Mat3f::orthographic_off_center(0.0, 1280.0, 720.0, 0.0, 1.0, 0.0);

        unsafe {
            let name = std::ffi::CString::new("MatrixTransform").unwrap();
            self.view_proj_mat = Mat3f::multiply(&self.transform_mat, &self.proj_mat);
            // internally, MojoShader uses column-major matrices so we transpose it
            pipe.shader
                .set_param(&name, &self.view_proj_mat.transpose());
            // TODO: use inlined transposed orthographic matrix for efficiency
        }

        // `FNA3D_ApplyEffect`
        pipe.apply_effect(device, 0);

        // `FNA3D_SetVertexBufferData`
        self.upload_vertices(device);

        // `FNA3D_DrawIndexedPrimitives`
        for call in self.batch.iter() {
            Self::make_draw_call(device, pipe, &mut self.bufs, call);
        }

        self.batch.clear();
    }
}

/// Sub procedures of [`Batcher::flush`]
/// ---
impl Batcher {
    /// Copies vertex data from CPU to GPU ([`SpriteBatch::vertex_data`] to [`VertexBuffer`])
    fn upload_vertices(&mut self, device: &mut fna3d::Device) {
        let offset = 0;
        let data = &mut self.batch.quads_to_upload_to_gpu();
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
