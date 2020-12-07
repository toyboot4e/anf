//! Internals of quad rendering

pub mod batch;
pub mod bufspecs;

use {
    fna3d_hie::{Pipeline, Shader},
    fna3h::{buf::SetDataOptions, draw::PrimitiveType, tex::Texture, Device},
};

use crate::{
    batcher::{
        batch::{DrawCall, SpriteBatch},
        bufspecs::{GpuViBuffer, QuadData},
    },
    geom3d::Mat4x4,
};

/// Push quads and flush
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
    /// In row-major sence (XNA), they're transposed:
    ///
    /// * P_clip = P_local (M_model M_view) M_proj
    /// * M_transform = (M_view M_model)
    mat_model_view_proj: Mat4x4,
}

impl Batcher {
    pub fn from_device(device: &Device) -> Self {
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
        texture: *mut Texture,
        device: &Device,
        pipe: &mut Pipeline,
    ) -> &'a mut QuadData {
        if self.batch.is_satured() {
            self.flush(device, pipe);
        }

        unsafe { self.batch.next_quad_mut(texture) }
    }

    /// Draws all the pushed sprites
    pub fn flush(&mut self, device: &Device, pipe: &mut Pipeline) {
        if !self.batch.any_quads_pushed() {
            return;
        }

        // FIXME: get viewport
        self.set_proj_mat(&mut pipe.shader);
        pipe.shader.apply_effect(device, 0);

        self.bufs.vbuf.upload_vertices(
            device,
            0, // vertex offset
            self.batch.pushed_quads(),
            SetDataOptions::None,
        );

        pipe.shader.apply_effect(device, 0);

        pipe.set_vertex_attributes(&mut self.bufs.vbuf.inner, 0);

        for call in self.batch.iter() {
            self.draw(&call, device, pipe);
        }

        self.batch.clear();
    }

    fn draw(&self, call: &DrawCall, device: &Device, pipe: &mut Pipeline) {
        pipe.set_texture_raw(device, call.tex);
        pipe.upload_vertex_attributes(device, call.base_vtx() as u32);

        device.draw_indexed_primitives(
            PrimitiveType::TriangleList,
            call.base_vtx() as u32, // the number of vertices to skip
            0,
            call.n_verts() as u32,
            call.base_idx() as u32, // NOTE: our index buffer is cyclic and we don't need to actually calculate it
            call.n_triangles() as u32,
            self.bufs.ibuf.raw(),
            self.bufs.ibuf.elem_size(),
        );
    }
}

impl Batcher {
    fn set_proj_mat(&mut self, shader: &mut Shader) {
        self.mat_proj = Mat4x4::orthographic_off_center(0.0, 1280.0, 720.0, 0.0, 1.0, 0.0);
        self.mat_model_view_proj = Mat4x4::multiply(&self.mat_model_view, &self.mat_proj);
        unsafe {
            shader.set_param("MatrixTransform", &self.mat_model_view_proj.transpose());
        }
    }
}
