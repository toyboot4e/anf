//! Re-exported to super module

use ::fna3d_hie::{buffers::GpuIndexBuffer, Pipeline};

use crate::{
    batcher::{
        batch::{SpriteBatch, SpriteDrawCall},
        bufspecs::{GpuViBuffer, QuadData},
    },
    geom3d::Mat4x4,
};

/// [`SpriteBatch`] with GPU vertex/index buffer handle
#[derive(Debug)]
pub struct Batcher {
    pub batch: SpriteBatch,
    bufs: GpuViBuffer,
    /// The projection matrix (fixed to orthographic matrix)
    mat_proj: Mat4x4,
    /// The transformation matrix
    mat_model_view: Mat4x4,
    /// The view projection matrix used by vertex shader
    ///
    /// # Coordinate systems
    ///
    /// See https://learnopengl.com/Getting-started/Coordinate-Systems
    ///
    /// In column-major sence,
    ///
    /// * P_clip = M_proj (M_view M_model) P_local
    /// * M_transform = (M_view M_model)
    ///
    /// In row-major sence,
    ///
    /// * P_clip = P_local (M_model M_view) M_proj
    /// * M_transform = (M_view M_model)
    mat_model_view_proj: Mat4x4,
}

impl Batcher {
    pub fn from_device(device: &fna3d::Device) -> Self {
        Self {
            batch: SpriteBatch::new(),
            bufs: GpuViBuffer::from_device(device),
            mat_proj: Mat4x4::orthographic(1.0, 1.0, 1.0, 0.0),
            mat_model_view: Mat4x4::identity(),
            mat_model_view_proj: Mat4x4::default(),
        }
    }

    pub fn is_satured(&self) -> bool {
        self.batch.is_satured()
    }

    pub fn next_quad_mut_safe(
        &mut self,
        texture: *mut fna3d::Texture,
        device: &fna3d::Device,
        pipe: &mut Pipeline,
    ) -> &mut QuadData {
        if self.batch.is_satured() {
            self.flush(device, pipe);
        }

        unsafe { self.batch.next_quad_mut(texture) }
    }
}

/// Batch cycle
/// ---
impl Batcher {
    /// Draws all the pushed sprites
    pub fn flush(&mut self, device: &fna3d::Device, pipe: &mut Pipeline) {
        if !self.batch.any_quads_pushed() {
            return;
        }

        self.flush_impl(device, pipe);
    }

    fn flush_impl(&mut self, device: &fna3d::Device, pipe: &mut Pipeline) {
        // Material (blend, sampler, depth/stencil, rasterizer)
        // viewport, scissors rect

        // update shader matrix
        // FIXME: get viewport
        self.mat_proj = Mat4x4::orthographic_off_center(0.0, 1280.0, 720.0, 0.0, 1.0, 0.0);

        unsafe {
            let name = std::ffi::CString::new("MatrixTransform").unwrap();
            self.mat_model_view_proj = Mat4x4::multiply(&self.mat_model_view, &self.mat_proj);
            // internally, MojoShader uses column-major matrices so we transpose it
            pipe.shader
                .set_param(&name, &self.mat_model_view_proj.transpose());
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
    fn upload_vertices(&mut self, device: &fna3d::Device) {
        let offset = 0;
        let data = &mut self.batch.quads_mut();
        self.bufs
            .vbuf
            .upload_vertices(device, offset, data, fna3d::SetDataOptions::None);
    }

    /// Runs [`SpriteDrawCall`] got from [`SpriteBatch`]
    fn make_draw_call(
        device: &fna3d::Device,
        pipe: &mut Pipeline,
        bufs: &mut GpuViBuffer,
        call: SpriteDrawCall<'_>,
    ) {
        pipe.set_texture_raw(device, call.texture());
        pipe.set_vertex_attributes(&mut bufs.vbuf.inner, 0);
        pipe.upload_vertex_attributes(device, call.base_vertex() as u32);
        self::draw_triangles(device, call, &bufs.ibuf);
    }
}

fn draw_triangles(device: &fna3d::Device, call: SpriteDrawCall<'_>, ibuf: &GpuIndexBuffer) {
    device.draw_indexed_primitives(
        fna3d::PrimitiveType::TriangleList,
        call.base_vertex() as u32, // the number of vertices to skip
        call.base_index() as u32, // REMARK: our index buffer is cyclic and we don't need to actually calculate it
        call.n_primitives() as u32,
        ibuf.raw(),
        ibuf.elem_size(),
    );
}
