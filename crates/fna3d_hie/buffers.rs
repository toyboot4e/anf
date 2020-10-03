//! GPU buffers, wrappers of [`*mut fna3d::Buffer`](fna3d::Buffer)
//!
//! * TODO: explain what "dynamic" buffer means and what options are available

/// Marker to represent vertex data that can be sent to GPU memory
pub trait VertexData {}

// --------------------------------------------------------------------------------
// Index buffer (ibuf)

/// Handle of index buffer in GPU memory
///
/// "Typed" with [`fna3d::IndexElementSize`]
#[derive(Debug)]
pub struct GpuIndexBuffer {
    raw: *mut fna3d::Buffer,
    n_indices: u32,
    usage: fna3d::BufferUsage,
    elem_size: fna3d::IndexElementSize,
}

impl GpuIndexBuffer {
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

    /// Sends index buffer to GPU memory
    pub fn upload_indices<T>(
        &mut self,
        device: &mut fna3d::Device,
        offset_in_bytes: u32,
        data: &[T],
    ) {
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

/// Handle to upload vertex data to GPU
///
/// "Typed" with [`fna3d::VertexDeclaration`]
#[derive(Debug)]
pub struct GpuVertexBuffer {
    raw: *mut fna3d::Buffer,
    pub n_vertices: u32,
    pub usage: fna3d::BufferUsage,
    pub decl: fna3d::VertexDeclaration,
}

/// Handle to upload vertex data to GPU
///
/// "typed" with [`fna3d::VertexDeclaration`]
#[derive(Debug)]
pub struct GpuDynamicVertexBuffer {
    pub inner: GpuVertexBuffer,
}

impl GpuVertexBuffer {
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

impl GpuDynamicVertexBuffer {
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
            inner: GpuVertexBuffer::new(device, decl, n_vertices, usage, true),
        }
    }

    /// Sends vertex data to GPU memory
    pub fn upload_vertices<T: VertexData>(
        &mut self,
        device: &mut fna3d::Device,
        buf_offset_in_bytes: u32,
        vdata: &mut [T],
        opts: fna3d::SetDataOptions,
    ) {
        device.set_vertex_buffer_data(self.inner.raw(), buf_offset_in_bytes, vdata, opts);
    }
}

/// Handle to upload vertex attributes to GPU
///
/// * TODO: what is instance drawing/frequency
/// * TODO: multiples slots
#[derive(Debug)]
pub struct GpuVertexAttributes {
    bind: fna3d::VertexBufferBinding,
    // is_updated: bool,
}

impl GpuVertexAttributes {
    pub fn new(decl: fna3d::VertexDeclaration) -> Self {
        GpuVertexAttributes {
            bind: fna3d::VertexBufferBinding {
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

    pub fn upload_vertex_attributes(&mut self, device: &mut fna3d::Device, base_vertex: u32) {
        device.apply_vertex_buffer_bindings(&[self.bind], true, base_vertex);
    }
}
