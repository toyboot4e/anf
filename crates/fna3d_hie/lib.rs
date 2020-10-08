//! Hierarchy, the wrapper of [`fna3d`]
//!
//! * TODO: full-featured resource binding object (buffers and textures)
//! ** textures with name `fs_textures`
//! * TODO: full-featured pipeline state object

pub mod buffers;
mod pipeline;
mod shader;

pub use crate::{pipeline::Pipeline, shader::Shader};
