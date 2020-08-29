//! ANF is an FNA-like 2D framework powered by FNA3D
//!
//! WIP: It offers game loop, `TextureGen`  and `Batcher`.
//!
//! ANF is also intended to introduce FNA3D so the documentation goes into internals details.
//!
//! # TODOs:
//!
//! * TODO: free memory on neessary
//! * TODO: copy FNA3D to output
//! * TODO: copy `assets/` to output
//! * TODO: FPS
//! * TODO: `Texture2D` with or without lifetime
//! * TODO: content loader (cache `Teture2D`)

pub use fna3d;
pub use sdl2;

pub mod app;
pub mod gfx;
pub mod vfs;
