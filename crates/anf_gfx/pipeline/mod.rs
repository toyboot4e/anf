//! Rendering pipeline
//!
//! The rendering cycle is descried in the [`fna3d::Device`] documentation.
//!
//! [`fna3d::Device`]: crate::fna3d::Device
//!
//! * TODO: `Material`?

mod shader;
pub use shader::Shader;

mod pipeline;
pub use pipeline::Pipeline;
