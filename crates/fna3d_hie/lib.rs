//! Hierarchy, the wrapper of [`fna3d`]
//!
//! * TODO: full-featured resource binding object (buffers and textures)
//! ** textures with name `fs_textures`
//! * TODO: full-featured pipeline state object

mod shader;
pub use shader::Shader;

mod pipeline;
pub use pipeline::Pipeline;

pub mod buffers;
