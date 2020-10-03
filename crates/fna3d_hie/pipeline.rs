//! Re-exported to the root of the module

use crate::{
    buffers::{GpuVertexAttributes, GpuVertexBuffer},
    shader::Shader,
};

/// Pipeline state objects
///
/// * TODO: multiple slots (buffers and attributes)
/// * TODO: rasterizer state, depth stencil state
/// * TODO: sampler count (MSAA), sampling masks
#[derive(Debug)]
pub struct Pipeline {
    vtx_attrs: GpuVertexAttributes,
    pub shader: Shader,
    sampler: SamplerSlots,
}

impl Pipeline {
    pub fn new(
        device: &mut fna3d::Device,
        initial_vtx_decl: fna3d::VertexDeclaration,
        shader_bytes: &[u8],
    ) -> Self {
        let mut shader =
            Shader::from_bytes(device, shader_bytes).expect("failed to load shader from bytes");
        shader.set_projection_matrix_1d(&fna3d::mojo::ORTHOGRAPHIC_MATRIX);

        Self {
            vtx_attrs: GpuVertexAttributes::new(initial_vtx_decl),
            shader,
            sampler: SamplerSlots::from_device(device),
        }
    }
}

/// Rendering pipeline methods
/// ---
impl Pipeline {
    // pub fn set_proj_mat(&mut self, mat: &Mat4) {}

    /// * `FNA3D_ApplyEffect`
    pub fn apply_effect(&mut self, device: &mut fna3d::Device, pass: u32) {
        self.shader.apply_effect(device, pass);
    }

    /// * `FNA3D_VerifySamplerState`
    pub fn set_texture_raw(&mut self, device: &mut fna3d::Device, texture: *mut fna3d::Texture) {
        self.sampler.set_texture_raw(device, texture);
    }

    /// Copies vertex buffer attributes
    pub fn set_vertex_attributes(&mut self, vbuf: &mut GpuVertexBuffer, offset: u32) {
        self.vtx_attrs.reset_vertex_attributes(vbuf, offset);
    }

    /// * `FNA3D_ApplyVertexBufferBindings`
    ///
    /// "The very last thing to call when making a draw call".
    pub fn upload_vertex_attributes(&mut self, device: &mut fna3d::Device, base_vertex: u32) {
        self.vtx_attrs.upload_vertex_attributes(device, base_vertex);
    }
}

// --------------------------------------------------------------------------------
// Internals

/// Slots of texture sampling methods
#[derive(Debug)]
struct SamplerSlots {
    pub samplers: Vec<fna3d::SamplerState>,
    pub v_samplers: Vec<fna3d::SamplerState>,
}

impl SamplerSlots {
    pub fn from_device(device: &fna3d::Device) -> Self {
        let (max_tx, max_v_tx) = device.get_max_texture_slots();

        log::info!("device max_textures: {}", max_tx);
        log::info!("device max_vertex_textures: {}", max_v_tx);

        assert!(
            max_tx != 0 && max_v_tx != 0,
            "Error on max texture slots. FNA3D may have been compiled in a wrong way: max_textures={}, max_vertex_textures={}",
            max_tx, max_v_tx
        );

        let sampler = {
            let mut s = fna3d::SamplerState::linear_clamp();
            s.set_address_u(fna3d::TextureAddressMode::Wrap);
            s.set_filter(fna3d::TextureFilter::Point);
            s
        };

        SamplerSlots {
            samplers: vec![sampler.clone(); max_tx as usize],
            v_samplers: vec![sampler; max_v_tx as usize],
        }
    }

    pub fn set_texture_raw(&mut self, device: &mut fna3d::Device, texture: *mut fna3d::Texture) {
        let slot = 0;
        device.verify_sampler(slot as u32, texture, &self.samplers[slot]);
    }
}
