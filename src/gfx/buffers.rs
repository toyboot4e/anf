//! Vertex/index buffer
//!
//! Internally, they are wrappers of `*mut fna3d::Buffer`; they are dynamically "typed" with
//! attributes.
//!
//! * TODO: explain what "dynamic" buffer means and what options are available

/// Marker to represent vertex data that can be sent to GPU memory
pub trait VertexData {}

// --------------------------------------------------------------------------------
// Index buffer (ibuf)

/// Index buffer that indexes `VertexBuffer`
///
/// "Typed" with `fna3d::IndexElementSize`
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
        is_dynamic: bool, // TODO: what is this. why not make a `StaticIndexBuffer`
    ) -> Self {
        let n_bytes = match index_elem_size {
            fna3d::IndexElementSize::Bits16 => 2,
            fna3d::IndexElementSize::Bits32 => 4,
        };
        let size_in_bytes = n_indices * n_bytes;
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

// --------------------------------------------------------------------------------
// Vertex buffer (vbuf)

/// Vertex buffer that is indexed by `IndexBuffer`
///
/// "Typed" with `fna3d::VertexDeclaration`
#[derive(Debug)]
pub struct VertexBufferData {
    raw: *mut fna3d::Buffer,
    pub n_vertices: u32,
    pub usage: fna3d::BufferUsage,
    pub decl: fna3d::VertexDeclaration,
}

/// Dynamic vertex buffer
///
/// Dynamically "typed" with `fna3d::VertexDeclaration`.
#[derive(Debug)]
pub struct DynamicVertexBuffer {
    pub(crate) inner: VertexBufferData,
}

impl VertexBufferData {
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

        VertexBufferData {
            n_vertices,
            usage,
            decl,
            raw,
        }
    }

    /// Sets vertex data to the GPU buffer
    pub fn set_data<T: VertexData>(
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
            inner: VertexBufferData::new(device, decl, n_vertices, usage, true),
        }
    }

    /// Sets vertex data to the GPU buffer
    pub fn set_data<T: VertexData>(
        &mut self,
        device: &mut fna3d::Device,
        buf_offset_in_bytes: u32,
        vdata: &mut [T],
        opts: fna3d::SetDataOptions,
    ) {
        device.set_vertex_buffer_data(self.inner.raw(), buf_offset_in_bytes, vdata, opts);
    }
}
