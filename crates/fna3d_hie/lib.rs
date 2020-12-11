//! Even higher layer to [`fna3h`] (or `fna3d`)

pub mod buf;
pub mod pass;

mod pip;

pub use crate::pip::{Pipeline, Shader};

// macros are always exported to the root of the crate

/// Creates index buffer for quadliterals
///
/// Vertex order: left-up, right-up, left-down and right-down.
#[macro_export]
macro_rules! gen_quad_indices {
    ( $n_quads:expr ) => {{
        let mut indices = [0; 6 * $n_quads as usize];

        for n in 0..$n_quads as i16 {
            let (i, v) = (n * 6, n * 4);
            indices[i as usize] = v as i16;
            indices[(i + 1) as usize] = v + 1 as i16;
            indices[(i + 2) as usize] = v + 2 as i16;
            indices[(i + 3) as usize] = v + 3 as i16;
            indices[(i + 4) as usize] = v + 2 as i16;
            indices[(i + 5) as usize] = v + 1 as i16;
        }

        indices
    }};
}

// pub struct StaticMesh {
//     pub vbuf: GpuDynamicVertexBuffer,
//     pub ibuf: GpuIndexBuffer,
//     pub imgs: [*mut Textue; 8],
// }
