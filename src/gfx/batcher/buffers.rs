//! Internal buffers of `Batcher`

use crate::gfx::{
    batcher::batch_data::batch_internals::*,
    vertices::{DynamicVertexBuffer, IndexBuffer, VertexBuffer},
};

// TODO: user proper name
/// Vertex/index buffer
///
/// `IndexBuffer` is rather static because we only draw rectangle sprites represented as two
/// triangles. Since index pattern is cyclic and static, `IndexBuffer` is automatically generated
/// to fill buffer and you can forget about it after creating `ViBuffers`.
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
            ColoredVertexData::decl(),
            MAX_VERTICES as u32,
            fna3d::BufferUsage::WriteOnly,
        );

        let mut ibuf = IndexBuffer::new(
            device,
            INDEX_ELEM_SIZE,
            MAX_INDICES as u32,
            fna3d::BufferUsage::WriteOnly, // what is this
            false,
        );

        ibuf.set_data(device, 0, &gen_index_array());

        ViBuffers { vbuf, ibuf }
    }
}
