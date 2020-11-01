//! Internals of quad rendering

pub mod batch;
pub mod bufspecs;

use fna3d_hie::{buffers::GpuIndexBuffer, Pipeline, Shader};

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
    batch: SpriteBatch,
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
            mat_proj: Mat4x4::orthographic(0.0, 0.0, 1.0, 0.0),
            mat_model_view: Mat4x4::identity(),
            mat_model_view_proj: Mat4x4::default(),
        }
    }

    pub fn next_quad_mut<'a>(
        &'a mut self,
        texture: *mut fna3d::Texture,
        device: &fna3d::Device,
        pipe: &mut Pipeline,
    ) -> &'a mut QuadData {
        if self.batch.is_satured() {
            self.flush(device, pipe);
        }

        unsafe { self.batch.next_quad_mut(texture) }
    }

    /// Draws all the pushed sprites
    pub fn flush(&mut self, device: &fna3d::Device, pipe: &mut Pipeline) {
        if !self.batch.any_quads_pushed() {
            return;
        }

        // FIXME: get viewport
        self.set_proj_mat(&mut pipe.shader);
        pipe.shader.apply_effect(device, 0);

        self.upload_vertices(device);
        pipe.set_vertex_attributes(&mut self.bufs.vbuf.inner, 0);

        for call in self.batch.iter() {
            Self::make_draw_call(device, pipe, &mut self.bufs, call);
        }

        self.batch.clear();
    }
}

/// Sub procedures of [`Batcher::flush`]
/// ---
impl Batcher {
    fn set_proj_mat(&mut self, shader: &mut Shader) {
        self.mat_proj = Mat4x4::orthographic_off_center(0.0, 1280.0, 720.0, 0.0, 1.0, 0.0);
        self.mat_model_view_proj = Mat4x4::multiply(&self.mat_model_view, &self.mat_proj);
        unsafe {
            shader.set_param("MatrixTransform", &self.mat_model_view_proj.transpose());
        }
    }

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
        call: SpriteDrawCall,
    ) {
        pipe.set_texture_raw(device, call.texture());
        pipe.upload_vertex_attributes(device, call.base_vertex() as u32);
        Self::draw_triangles(device, call, &bufs.ibuf);
    }

    fn draw_triangles(device: &fna3d::Device, call: SpriteDrawCall, ibuf: &GpuIndexBuffer) {
        device.draw_indexed_primitives(
            fna3d::PrimitiveType::TriangleList,
            call.base_vertex() as u32, // the number of vertices to skip
            call.base_index() as u32, // NOTE: our index buffer is cyclic and we don't need to actually calculate it
            call.n_primitives() as u32,
            ibuf.raw(),
            ibuf.elem_size(),
        );
    }
}
