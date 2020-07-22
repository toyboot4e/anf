use crate::gfx::{
    batcher::batch_data::batch_internals::VertexData, pipeline::shader::Shader, texture::Texture2D,
    vertices::VertexBuffer,
};

/// Rendering pipeline data
///
/// Corresponds to `GraphicsDevice` in FNA.
///
/// Contains methods to call:
///
/// * `FNA3D_ApplyEffect`
/// * `FNA3D_ApplyVertexBufferBindings`
#[derive(Debug)]
pub struct Pipeline {
    v_binds: VBind,
    shader: Shader,
    state: SamplerTrack,
}

impl Pipeline {
    pub fn from_device(device: &mut fna3d::Device) -> Self {
        Self {
            v_binds: VBind::new(VertexData::decl()),
            state: SamplerTrack::from_device(device),
            // TODO: don't unwrap?
            shader: Shader::from_device(device).unwrap(),
        }
    }

    // ----------------------------------------
    // Shader

    pub fn apply_effect(&mut self, device: &mut fna3d::Device, pass: u32) {
        self.shader.apply_effect(device, pass);
    }

    // ----------------------------------------
    // Sampler state & materials?

    pub fn set_texture(&mut self, device: &mut fna3d::Device, texture: &Texture2D) {
        self.state.set_texture(device, texture);
    }

    // ----------------------------------------
    // Vertex binding

    pub fn bind_vertex_buffer(&mut self, vbuf: &mut VertexBuffer, offset: i32) {
        self.v_binds.bind(vbuf, offset);
    }

    /// Applices the binded vertex data to the `fna3d::Device`
    pub fn apply_vertex_buffer_bindings(&mut self, device: &mut fna3d::Device, base_vertex: i32) {
        self.v_binds.apply(device, base_vertex);
    }
}

// --------------------------------------------------------------------------------

// TODO: what is this. what is instance drawing/frequency
/// Vertex buffer bindings
///
/// This is actually a slice of vertex buffer where `fna3d::Device` reads.
///
/// A part of the rendering pipeline. Component of `GraphicsDevice` in FNA3D.
#[derive(Debug)]
pub struct VBind {
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
    /// Unlike FNA, we assume that we only use one `VertexBufferBinding`.
    pub fn apply(&mut self, device: &mut fna3d::Device, base_vertex: i32) {
        device.apply_vertex_buffer_bindings(&[self.bind], true, base_vertex);
        // self.is_updated = false;
    }

    // pub fn clear(&mut self) { }
}

// --------------------------------------------------------------------------------

/// Tracks `SamplerState` modifications
///
/// TODO: explain what is sampler
///
/// Component of `GraphicsDevice` in FNA
#[derive(Debug)]
pub struct SamplerTrack {
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
        Self {
            samplers: vec![fna3d::SamplerState::linear_wrap(); max_tx],
            v_samplers: vec![fna3d::SamplerState::linear_wrap(); max_v_tx],
        }
    }

    pub fn set_texture(&mut self, device: &mut fna3d::Device, texture: &Texture2D) {
        let slot = 0;
        device.verify_sampler(slot as i32, texture.raw(), &self.samplers[slot]);
        device.verify_vertex_sampler(slot as i32, texture.raw(), &self.v_samplers[slot]);
    }

    // // TODO: is should be called only on change
    // /// Cooresponds to half of `GrapihcsDevice.ApplyState`
    // pub fn apply_changes(&mut self, device: &mut fna3d::Device, texture: &Texture2D) {
    //     // mod_sampler = only first item

    //     {
    //         let i = 0;
    //         device.verify_sampler(i as i32, texture.raw(), &mut self.samplers[i]);
    //     }
    //     {
    //         let i = 0;
    //         device.verify_vertex_sampler(i as i32, texture.raw(), &mut self.samplers[i]);
    //     }
    // }
}
