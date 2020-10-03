//! imgui-rs renderer in Rust-FNA3D
//!
//! Based on the [glium renderer] in the imgui-rs [repository].
//!
//! TODO: re-export imgui types via prelude
//!
//! [glium renderer]: https://github.com/Gekkio/imgui-rs/tree/master/imgui-glium-renderer
//! [repository]: https://github.com/Gekkio/imgui-rs

#[cfg(feature = "sdl2")]
mod sdl2_backend;
#[cfg(feature = "sdl2")]
mod sdl2_helper;

mod renderer;
pub use renderer::{ImGuiRenderer, ImGuiRendererError, RcTexture2d, Result, Texture2d};
pub use sdl2_backend::ImguiSdl2;
pub use sdl2_helper::{Fna3dImgui, Fna3dImguiPart};

// bundle binaries
const SHARDER: &[u8] = include_bytes!("../SpriteEffect.fxb");
const JP_FONT: &[u8] = include_bytes!("../mplus-1p-regular.ttf");
