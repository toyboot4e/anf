//! Internal buffers
//!
//! We use `i16` for index buffers as defined in `batch_internals`

use crate::gfx::{
    batcher::batch::batch_internals::*,
    texture::Texture2D,
    vertices::{DynamicVertexBuffer, IndexBuffer, VertexBuffer},
};

// TODO: user proper name
/// Buffer objects for actual drawing
///
/// `IndexBuffer` is rather static because we only draw rectangle sprites represented as two
/// triangles. Each triangle is specified with three vertices and three indices. Since indices
/// pattern is cyclic and static, it's automatically generated and you can forget about
/// `IndexBuffer` after creating `ViBuffers`.
///
/// Component of `SpriteBatch` in XNA.
#[derive(Debug)]
pub struct ViBuffers {
    pub vbuf: DynamicVertexBuffer,
    pub ibuf: IndexBuffer,
    // effect: *mut fna3d::Effect;
}

fn gen_index_array() -> [i16; MAX_INDICES] {
    let mut data = [0; MAX_INDICES];
    // for each texture, we need two triangles (six indices)
    for n in 0..MAX_SPRITES as i16 {
        let (i, v) = (n * 6, n * 4);
        data[i as usize] = v as i16;
        data[(i + 1) as usize] = v + 1 as i16;
        data[(i + 2) as usize] = v + 2 as i16;
        data[(i + 3) as usize] = v + 3 as i16;
        data[(i + 4) as usize] = v + 2 as i16;
        data[(i + 5) as usize] = v + 1 as i16;
    }
    data
}

impl ViBuffers {
    pub fn from_device(device: &mut fna3d::Device) -> Self {
        // let mut device = fna3d::Device::from_params(&mut params, true);
        // device.reset_backbuffer(&mut params);

        let vbuf = DynamicVertexBuffer::new(
            device,
            VertexData::decl(),
            MAX_VERTICES as u32,
            fna3d::BufferUsage::WriteOnly,
        );

        let mut ibuf = IndexBuffer::new(
            device,
            fna3d::IndexElementSize::Bits16, // WE USE `i16` FOR INDEX BUFFERS
            MAX_INDICES as u32,
            fna3d::BufferUsage::WriteOnly, // what is this
            false,
        );

        ibuf.set_data(device, 0, &gen_index_array());

        ViBuffers { vbuf, ibuf }
    }
}

// --------------------------------------------------------------------------------

// TODO: proper name
/// Tracks `SamplerState` modifications
pub struct GlState {
    pub samplers: Vec<fna3d::SamplerState>,
    pub v_samplers: Vec<fna3d::SamplerState>,
}

impl GlState {
    pub fn from_device(device: &fna3d::Device) -> Self {
        let (max_textures, max_vertex_textures) = device.get_max_texture_slots();
        log::info!("device max_textures: {}", max_textures);
        log::info!("device max_vertex_textures: {}", max_vertex_textures);
        assert!(
            max_textures != 0 && max_vertex_textures != 0,
            "Error on max texture slots. FNA3D may have been compiled in a wrong way: max_textures={}, max_vertex_textures={}",
            max_textures, max_vertex_textures
        );
        Self {
            samplers: vec![fna3d::SamplerState::linear_wrap(); max_textures],
            v_samplers: vec![fna3d::SamplerState::linear_wrap(); max_vertex_textures],
        }
    }

    pub fn set_texture(&mut self, device: &mut fna3d::Device, texture: &Texture2D) {
        let i = 0;
        device.verify_sampler(i as i32, texture.raw(), &mut self.samplers[i]);
        device.verify_vertex_sampler(i as i32, texture.raw(), &mut self.v_samplers[i]);
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

// --------------------------------------------------------------------------------

// TODO: FIXME: what is this. is this not needed?
/// Vertex buffer bindings
#[derive(Debug)]
pub struct VBinds {
    bind: fna3d::VertexBufferBinding,
    is_updated: bool,
}

impl VBinds {
    pub fn new(decl: fna3d::VertexDeclaration) -> Self {
        VBinds {
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
    pub fn update(&mut self, vbuf: &mut VertexBuffer, offset: i32) {
        self.bind.vertexBuffer = vbuf.raw();
        self.bind.vertexDeclaration = vbuf.decl.clone();
        self.bind.vertexOffset = offset;
        self.is_updated = true;
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
