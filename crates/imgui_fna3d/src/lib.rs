//! imgui-rs renderer in Rust-FNA3D
//!
//! Based on the [glium renderer] in the imgui-rs [repository].
//!
//! [glium renderer]: https://github.com/Gekkio/imgui-rs/tree/master/imgui-glium-renderer
//! [repository]: https://github.com/Gekkio/imgui-rs

#[cfg(feature = "sdl2")]
mod sdl2_backend;

mod renderer;
pub use renderer::{ImGuiRenderer, ImGuiRendererError, RcTexture, Result, Texture2D};
pub use sdl2_backend::{Fna3dImgui, Fna3dImguiPart};
