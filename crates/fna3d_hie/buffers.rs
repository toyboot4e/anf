//! GPU buffers, wrappers of [`*mut fna3d::Buffer`](fna3d::Buffer)
//!
//! * TODO: explain what "dynamic" buffer means and what options are available

use fna3h::{
    buf::{Buffer, BufferUsage, IndexElementSize, SetDataOptions, VertexDeclaration},
    draw::VertexBufferBinding,
    Device,
};

/// Marker to represent vertex data that can be sent to GPU memory
///
/// You might have to set `#[repr(C)]` to your vertex struct.
pub trait VertexData {}

// --------------------------------------------------------------------------------
// Index buffer (ibuf)

/// Handle of index buffer in GPU memory
///
/// "Typed" with [`IndexElementSize`]
#[derive(Debug)]
pub struct GpuIndexBuffer {
    raw: *mut Buffer,
    n_indices: u32,
    usage: BufferUsage,
    elem_size: IndexElementSize,
}

impl GpuIndexBuffer {
    pub fn raw(&self) -> *mut Buffer {
        self.raw
    }

    pub fn elem_size(&self) -> IndexElementSize {
        self.elem_size
    }

    pub fn new(
        device: &Device,
        index_elem_size: IndexElementSize,
        n_indices: u32,
        usage: BufferUsage,
        is_dynamic: bool, // TODO: what is this. why not make a `StaticIndexBuffer`
    ) -> Self {
        let n_bytes = match index_elem_size {
            IndexElementSize::Bits16 => 2,
            IndexElementSize::Bits32 => 4,
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

    /// Sends index buffer to GPU memory
    pub fn upload_indices<T>(&mut self, device: &Device, offset_in_bytes: u32, data: &[T]) {
        device.set_index_buffer_data(self.raw(), offset_in_bytes, data, SetDataOptions::None);
    }
}

// --------------------------------------------------------------------------------
// Vertex buffer (vbuf)

/// Handle to upload vertex data to GPU
///
/// "Typed" with [`VertexDeclaration`]
#[derive(Debug)]
pub struct GpuVertexBuffer {
    raw: *mut Buffer,
    n_vertices: u32,
    usage: BufferUsage,
    decl: VertexDeclaration,
}

/// Handle to upload vertex data to GPU
///
/// "typed" with [`VertexDeclaration`]
#[derive(Debug)]
pub struct GpuDynamicVertexBuffer {
    pub inner: GpuVertexBuffer,
}

impl GpuVertexBuffer {
    pub fn raw(&self) -> *mut Buffer {
        self.raw
    }

    pub fn new(
        device: &Device,
        decl: VertexDeclaration,
        n_vertices: u32,
        usage: BufferUsage,
        is_dynamic: bool,
    ) -> Self {
        let size_in_bytes = n_vertices * decl.vertexStride as u32;
        assert!(size_in_bytes < 2u32.pow(31));
        let raw = device.gen_vertex_buffer(is_dynamic, usage, size_in_bytes);

        GpuVertexBuffer {
            n_vertices,
            usage,
            decl,
            raw,
        }
    }

    /// Sets vertex data to the GPU buffer
    pub fn upload_vertices<T: VertexData>(
        &mut self,
        device: &Device,
        offset_in_bytes: u32,
        vdata: &mut [T],
    ) {
        device.set_vertex_buffer_data(self.raw, offset_in_bytes, vdata, SetDataOptions::None);
    }
}

impl GpuDynamicVertexBuffer {
    pub fn raw(&self) -> *mut Buffer {
        self.inner.raw()
    }

    pub fn new(
        device: &Device,
        decl: VertexDeclaration,
        n_vertices: u32,
        usage: BufferUsage,
    ) -> Self {
        Self {
            inner: GpuVertexBuffer::new(device, decl, n_vertices, usage, true),
        }
    }

    /// Sends vertex data to GPU memory
    ///
    /// The vertex data is send by bytes so it can be something like `Quad([Vertex; 4])`.
    pub fn upload_vertices<T: VertexData>(
        &mut self,
        device: &Device,
        buf_offset_in_bytes: u32,
        vtx_data: &[T],
        opts: SetDataOptions,
    ) {
        device.set_vertex_buffer_data(self.inner.raw(), buf_offset_in_bytes, vtx_data, opts);
    }
}

/// Handle to upload vertex attributes to GPU
///
/// * TODO: what is instance drawing/frequency
/// * TODO: multiples slots
#[derive(Debug)]
pub struct GpuVertexAttributes {
    bind: VertexBufferBinding,
    // is_updated: bool,
}

impl GpuVertexAttributes {
    pub fn new(decl: VertexDeclaration) -> Self {
        GpuVertexAttributes {
            bind: VertexBufferBinding {
                vertexBuffer: std::ptr::null_mut(),
                vertexDeclaration: decl,
                vertexOffset: 0,
                instanceFrequency: 0,
            },
            // is_updated: false,
        }
    }

    pub fn reset_vertex_attributes(&mut self, vbuf: &mut GpuVertexBuffer, base_vertex: u32) {
        self.bind.vertexBuffer = vbuf.raw();
        self.bind.vertexDeclaration = vbuf.decl.clone();
        self.bind.vertexOffset = base_vertex as i32;
    }

    pub fn upload_vertex_attributes(&mut self, device: &Device, base_vertex: u32) {
        device.apply_vertex_buffer_bindings(&[self.bind], true, base_vertex);
    }
}
