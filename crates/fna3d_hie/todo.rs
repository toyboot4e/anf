use crate::buffers::*;

#[derive(Debug)]
pub struct DynamicMesh {
    ibuf: GpuIndexBuffer,
    vbug: GpuDynamicVertexBuffer,
}
