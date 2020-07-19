//! `VertexBuffer` and `IndexBuffer`

// use nalgebra::Ve

mod ibuf;
mod vbuf;

pub use ibuf::*;
pub use vbuf::*;

// TODO: add element data
/// Guard
///
/// A vertex data is composed of `fna3d::VertexElement`s which are dynamically "typed" with
/// `fna3d::VertexElement`
pub trait AnyVertexData {}
