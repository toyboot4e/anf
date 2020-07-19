//! Index buffer

use super::AnyVertexData;

/// Index buffer to draw primitives (triangles)
#[derive(Debug)]
pub struct IndexBuffer {
    raw: *mut fna3d::Buffer,
    n_indices: u32,
    usage: fna3d::BufferUsage,
    elem_size: fna3d::IndexElementSize,
}

impl IndexBuffer {
    pub fn raw(&self) -> *mut fna3d::Buffer {
        self.raw
    }

    pub fn elem_size(&self) -> fna3d::IndexElementSize {
        self.elem_size
    }

    pub fn new(
        device: &mut fna3d::Device,
        index_elem_size: fna3d::IndexElementSize,
        n_indices: u32,
        usage: fna3d::BufferUsage,
        is_dynamic: bool, // TODO: why not make a `StaticIndexBuffer`
    ) -> Self {
        let stride = match index_elem_size {
            fna3d::IndexElementSize::Bits16 => 2,
            fna3d::IndexElementSize::Bits32 => 4,
        };
        let size_in_bytes = n_indices * stride;
        let buf = device.gen_index_buffer(is_dynamic, usage, size_in_bytes);

        Self {
            raw: buf,
            n_indices,
            usage,
            elem_size: index_elem_size,
        }
    }

    pub fn set_data<T>(&mut self, device: &mut fna3d::Device, offset_in_bytes: u32, data: &[T]) {
        device.set_index_buffer_data(
            self.raw(),
            offset_in_bytes,
            data,
            fna3d::SetDataOptions::None,
        );
    }
}
