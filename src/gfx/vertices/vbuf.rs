//! Vertex buffer

use super::AnyVertexData;

// --------------------------------------------------------------------------------
// Declarations

// TODO: add lifetime?

#[derive(Debug)]
pub struct VertexBuffer {
    raw: *mut fna3d::Buffer,
    pub n_vertices: u32,
    pub usage: fna3d::BufferUsage,
    pub decl: fna3d::VertexDeclaration,
}

/// Dynamic vertex buffer
///
/// Actually type `T` in `set_data<T>` is `VertexInfo` in `batch_internals`.
#[derive(Debug)]
pub struct DynamicVertexBuffer {
    pub inner: VertexBuffer,
}

// --------------------------------------------------------------------------------
// impls

impl VertexBuffer {
    pub fn raw(&self) -> *mut fna3d::Buffer {
        self.raw
    }

    pub fn new(
        device: &mut fna3d::Device,
        decl: fna3d::VertexDeclaration,
        n_vertices: u32,
        usage: fna3d::BufferUsage,
        is_dynamic: bool,
    ) -> Self {
        let size_in_bytes = n_vertices * decl.vertexStride as u32;
        assert!(size_in_bytes < 2u32.pow(31));
        let raw = device.gen_vertex_buffer(is_dynamic, usage, size_in_bytes);

        Self {
            n_vertices,
            usage,
            decl,
            raw,
        }
    }

    /// Sets vertex data to thsi buffer
    pub fn set_data<T>(
        &mut self,
        device: &mut fna3d::Device,
        offset_in_bytes: u32,
        vdata: &mut [T],
    ) {
        device.set_vertex_buffer_data(
            self.raw,
            offset_in_bytes,
            vdata,
            fna3d::SetDataOptions::None,
        );
    }
}

impl DynamicVertexBuffer {
    pub fn raw(&self) -> *mut fna3d::Buffer {
        self.inner.raw()
    }

    pub fn new(
        device: &mut fna3d::Device,
        decl: fna3d::VertexDeclaration,
        n_vertices: u32,
        usage: fna3d::BufferUsage,
    ) -> Self {
        Self {
            inner: VertexBuffer::new(device, decl, n_vertices, usage, true),
        }
    }

    /// Sets vertex data to thsi buffer
    pub fn set_data<T: AnyVertexData>(
        &mut self,
        device: &mut fna3d::Device,
        buf_offset_in_bytes: u32,
        vdata: &mut [T],
        opts: fna3d::SetDataOptions,
    ) {
        device.set_vertex_buffer_data(self.inner.raw(), buf_offset_in_bytes, vdata, opts);
    }
}
