//! Re-exported to the root of the module

use crate::gfx::{
    batcher::batch_data::batch_internals::ColoredVertexData, pipeline::shader::Shader,
    texture::Texture2D, vertices::VertexBuffer,
};

/// The required rendering pipeline by FNA3D
///
/// Corresponds to `GraphicsDevice` in FNA. ANF users don't have to use it directly. Refer to
/// `Batcher` instead!
///
/// Contains methods corresponding to:
///
/// * `FNA3D_ApplyEffect`
/// * `FNA3D_VerifySamplerState`, `FNA3D_VerifyVertexSamplerState`
/// * `FNA3D_ApplyVertexBufferBindings`
#[derive(Debug)]
pub struct Pipeline {
    v_binds: VBind,
    shader: Shader,
    // TODO: multiple samplers? when?
    state: SamplerTrack,
}

impl Pipeline {
    pub fn from_device(device: &mut fna3d::Device) -> Self {
        Self {
            v_binds: VBind::new(ColoredVertexData::decl()),
            state: SamplerTrack::from_device(device),
            // TODO: don't unwrap?
            shader: Shader::from_device(device).unwrap(),
        }
    }

    // ----------------------------------------
    // Shader

    /// `FNA3D_ApplyEffect`
    pub fn apply_effect(&mut self, device: &mut fna3d::Device, pass: u32) {
        self.shader.apply_effect(device, pass);
    }

    // ----------------------------------------
    // Sampler state & materials?

    /// * `FNA3D_VerifySamplerState`
    /// * `FNA3D_VerifyVertexSamplerState`
    pub fn set_texture(&mut self, device: &mut fna3d::Device, texture: &Texture2D) {
        self.state.set_texture(device, texture);
    }

    pub fn update_shader(&mut self) {
        self.shader.update();
    }

    // ----------------------------------------
    // Vertex binding

    /// Updates the vertex buffer binding
    pub fn rebind_vertex_buffer(&mut self, vbuf: &mut VertexBuffer, offset: i32) {
        self.v_binds.bind(vbuf, offset);
    }

    /// * `FNA3D_ApplyVertexBufferBindings`
    pub fn apply_vertex_buffer_bindings(&mut self, device: &mut fna3d::Device, base_vertex: i32) {
        self.v_binds.apply(device, base_vertex);
    }
}

// --------------------------------------------------------------------------------
// Internals (very WIP)

// TODO: what is this. what is instance drawing/frequency
/// Vertex buffer bindings
///
/// This is actually a slice of vertex buffer where `fna3d::Device` reads.
///
/// A part of the rendering pipeline. Component of `GraphicsDevice` in FNA3D.
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

    pub fn bind(&mut self, vbuf: &mut VertexBuffer, base_vertex: i32) {
        self.bind.vertexBuffer = vbuf.raw();
        self.bind.vertexDeclaration = vbuf.decl.clone();
        self.bind.vertexOffset = base_vertex;
    }

    // /// Updates bindings (updates the slice)
    // ///
    // /// Corresponds to `GraphicsDevice.SetVertexBufferData`.
    // pub fn update(&mut self, vbuf: &mut VertexBuffer, base_vertex: i32) {
    //     // self.is_updated = true; // so it's always true. is it ok?
    // }

    /// Cooredponds to `GraphicsDevice.PrepareVertexBindingArray`.
    ///
    /// Unlike FNA, we only use one `VertexBufferBinding`.
    pub fn apply(&mut self, device: &mut fna3d::Device, base_vertex: i32) {
        device.apply_vertex_buffer_bindings(&[self.bind], true, base_vertex);
        // self.is_updated = false;
    }

    // pub fn clear(&mut self) { }
}

/// Tracks `SamplerState` modifications
///
/// TODO: explain what is sampler
///
/// Component of `GraphicsDevice` in FNA
#[derive(Debug)]
struct SamplerTrack {
    pub samplers: Vec<fna3d::SamplerState>,
    pub v_samplers: Vec<fna3d::SamplerState>,
}

impl SamplerTrack {
    pub fn from_device(device: &fna3d::Device) -> Self {
        let (max_tx, max_v_tx) = device.get_max_texture_slots();
        log::info!("device max_textures: {}", max_tx);
        log::info!("device max_vertex_textures: {}", max_v_tx);
        assert!(
            max_tx != 0 && max_v_tx != 0,
            "Error on max texture slots. FNA3D may have been compiled in a wrong way: max_textures={}, max_vertex_textures={}",
            max_tx, max_v_tx
        );
        let mut s = fna3d::SamplerState::linear_clamp();
        s.set_address_u(fna3d::TextureAddressMode::Wrap);
        s.set_filter(fna3d::TextureFilter::Point);
        Self {
            samplers: vec![s.clone(); max_tx],
            v_samplers: vec![s; max_v_tx],
        }
    }

    pub fn set_texture(&mut self, device: &mut fna3d::Device, texture: &Texture2D) {
        let slot = 0;
        device.verify_sampler(slot as i32, texture.raw(), &self.samplers[slot]);
        // TODO: is this needed??
        // device.verify_vertex_sampler(slot as i32, texture.raw(), &self.v_samplers[slot]);
    }
}
