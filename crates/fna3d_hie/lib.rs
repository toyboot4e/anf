//! Hierarchy, the wrapper of [`fna3d`]
//!
//! * TODO: full-featured resource binding object (buffers and textures)
//! ** textures with name `fs_textures`
//! * TODO: full-featured pipeline state object

pub mod buffers;
mod pipeline;
mod shader;

pub use crate::{pipeline::Pipeline, shader::Shader};

// macros are always exported to the root of the crate

/// Creates index buffer for quadliterals
#[macro_export]
macro_rules! gen_quad_indices {
    ( $n_quads:expr ) => {{
        let mut indices = [0; 6 * $n_quads];

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
