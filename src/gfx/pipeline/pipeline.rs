use crate::gfx::{
    batcher::batch::batch_internals::VertexData, pipeline::shader::Shader, texture::Texture2D,
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
    // rendering pipeline
    pub v_binds: VBind,
    pub shader: Shader,
    // state
    pub state: SamplerTrack,
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
}

// --------------------------------------------------------------------------------

// TODO: what is this. what is instance drawing/frequency
/// Vertex buffer bindings
///
/// Binded to a range of `VertexBuffer` before actually drawing primitives.
///
/// A part of the rendering pipeline. Component of `GraphicsDevice` in FNA3D.
#[derive(Debug)]
pub struct VBind {
    bind: fna3d::VertexBufferBinding,
    is_updated: bool,
}

impl VBind {
    pub fn new(decl: fna3d::VertexDeclaration) -> Self {
        VBind {
            bind: fna3d::VertexBufferBinding {
                vertexBuffer: std::ptr::null_mut(),
                vertexDeclaration: decl,
                vertexOffset: 0,
                instanceFrequency: 0,
            },
            is_updated: false,
        }
    }

    /// Updates bindings
    ///
    /// Corresponds to `GraphicsDevice.SetVertexBufferData`. Different from `GraphicsDevice`, we
    /// dont' use non-native `VertexBufferBinding` and this method directly updates a native
    /// (FNA3D) `VertexBuffer`.
    pub fn bind(&mut self, vbuf: &mut VertexBuffer, offset: i32) {
        self.bind.vertexBuffer = vbuf.raw();
        self.bind.vertexDeclaration = vbuf.decl.clone();
        self.bind.vertexOffset = offset;
        self.is_updated = true; // so it's always true. is it ok?
    }

    // pub fn clear(&mut self) { }

    /// Cooredponds to `GraphicsDevice.PrepareVertexBindingArray`.
    ///
    /// Unlike FNA, we assume that we only use one `VertexBufferBinding`.
    pub fn apply_vertex_buffer_bindings(&mut self, device: &mut fna3d::Device, base_vertex: i32) {
        device.apply_vertex_buffer_bindings(&[self.bind], self.is_updated, base_vertex);
        self.is_updated = false;
    }
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
