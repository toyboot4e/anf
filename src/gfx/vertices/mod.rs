//! `Graphics/Vertices/*`
//!
//! ## enums
//!
//! - [x] BufferUsage.cs
//! - [x] VertexElementFormat.cs
//! - [x] VertexElementUsage.cs
//!
//! ## classes
//!
//! - [x] DynamicIndexBuffer.cs
//! - [x] DynamicVertexBuffer.cs
//! - [x] IndexBuffer.cs
//! - [ ] IndexElementSize.cs
//! - [ ] IVertexType.cs
//! - [x] VertexBuffer.cs
//! - [x] VertexBufferBinding.cs
//! - [x] VertexDeclaration.cs
//! - [ ] VertexDeclarationCache.cs
//! - [x] VertexElement.cs
//! - [ ] VertexPositionColor.cs
//! - [ ] VertexPositionColorTexture.cs
//! - [ ] VertexPositionNormalTexture.cs
//! - [ ] VertexPositionTexture.cs

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
