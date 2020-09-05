//! Re-exported to the root of the module

use crate::{
    batcher::bufspecs::ColoredVertexData, buffers::VertexBufferData, pipeline::shader::Shader,
    texture::Texture2D,
};
use anf_deps::fna3d;
use std::path::Path;

/// Rendering pipeline states
///
/// * TODO: maybe extract resource bindings slots? (see sg_bindings)
///
/// # Missing features
///
/// * multiple vertex/index buffer slots
/// * rasterizer state, depth stencil state
/// * sampler count (MSAA), sampling masks
#[derive(Debug)]
pub struct Pipeline {
    v_binds: VBind,
    shader: Shader,
    state: SamplerSlots,
}

impl Pipeline {
    pub fn from_device(device: &mut fna3d::Device, shader_path: impl AsRef<Path>) -> Self {
        Self {
            v_binds: VBind::new(ColoredVertexData::decl()),
            state: SamplerSlots::from_device(device),
            shader: Shader::from_device(device, shader_path).expect("faild to create a shader"),
        }
    }
}

/// Rendering pipeline methods
/// ---
impl Pipeline {
    /// Resets shader uniforms
    pub fn update_shader(&mut self) {
        self.shader.apply_uniforms();
    }

    /// `FNA3D_ApplyEffect`
    pub fn apply_effect(&mut self, device: &mut fna3d::Device, pass: u32) {
        self.shader.apply_effect(device, pass);
    }

    /// Binds data set with `FNA3D_SetVertexData`. FIXME: this is loose about ownership and borrow model
    pub fn rebind_vertex_buffer(&mut self, vbuf: &mut VertexBufferData, offset: u32) {
        self.v_binds.bind(vbuf, offset);
    }

    /// * `FNA3D_VerifySamplerState`
    pub fn set_texture(&mut self, device: &mut fna3d::Device, texture: &Texture2D) {
        self.state.set_texture(device, texture);
    }

    /// * `FNA3D_ApplyVertexBufferBindings`
    ///
    /// "The very last thing to call when making a draw call".
    pub fn apply_vertex_buffer_bindings(&mut self, device: &mut fna3d::Device, base_vertex: i32) {
        self.v_binds.apply(device, base_vertex);
    }
}

// --------------------------------------------------------------------------------
// Internals

/// Vertex buffer binding that can send/copy/upload its vertex data to GPU
///
/// * TODO: what is instance drawing/frequency
/// * TODO: multiples slots
#[derive(Debug)]
struct VBind {
    bind: fna3d::VertexBufferBinding,
    // is_updated: bool,
}

impl VBind {
    fn new(decl: fna3d::VertexDeclaration) -> Self {
        VBind {
            bind: fna3d::VertexBufferBinding {
                vertexBuffer: std::ptr::null_mut(),
                vertexDeclaration: decl,
                vertexOffset: 0,
                instanceFrequency: 0,
            },
            // is_updated: false,
        }
    }

    pub fn bind(&mut self, vbuf: &mut VertexBufferData, base_vertex: u32) {
        self.bind.vertexBuffer = vbuf.raw();
        self.bind.vertexDeclaration = vbuf.decl.clone();
        self.bind.vertexOffset = base_vertex as i32;
    }

    pub fn apply(&mut self, device: &mut fna3d::Device, base_vertex: i32) {
        device.apply_vertex_buffer_bindings(&[self.bind], true, base_vertex);
        // self.is_updated = false;
    }

    // pub fn clear(&mut self) { }
}

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
            samplers: vec![sampler.clone(); max_tx],
            v_samplers: vec![sampler; max_v_tx],
        }
    }

    pub fn set_texture(&mut self, device: &mut fna3d::Device, texture: &Texture2D) {
        let slot = 0;
        device.verify_sampler(slot as i32, texture.raw(), &self.samplers[slot]);
        // device.verify_vertex_sampler(slot as i32, texture.raw(), &self.v_samplers[slot]);
    }
}
