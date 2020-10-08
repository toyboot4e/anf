//! imgui-rs renderer in Rust-FNA3D
//!
//! Based on the [glium renderer] in the imgui-rs [repository].
//!
//! TODO: re-export imgui types via prelude
//!
//! [glium renderer]: https://github.com/Gekkio/imgui-rs/tree/master/imgui-glium-renderer
//! [repository]: https://github.com/Gekkio/imgui-rs

mod fna3d_renderer;
mod helper;
mod sdl2_backend;

pub use fna3d_renderer::{ImGuiRendererError, Result};
pub use helper::Fna3dImgui;

/// SpriteEffect.fxb
pub const SHARDER: &[u8] = include_bytes!("../SpriteEffect.fxb");

/// mplus 1p regular
pub const JP_FONT: &[u8] = include_bytes!("../mplus-1p-regular.ttf");
