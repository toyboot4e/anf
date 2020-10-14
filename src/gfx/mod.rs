//! Graphics data types
//!
//! These structs implement draw API traits in [`crate::engine::draw`] so they can be drawn via
//! [`DrawContext`].
//!
//! [`DrawContext`]: crate::engine::draw::DrawContext

pub use ::{
    anf_gfx::{
        geom2d, geom3d,
        texture::{SpriteData, SubTextureData2d, TextureData2d},
    },
    fna3d::Color,
};

#[cfg(feature = "font")]
pub mod font;

pub mod prelude {
    //! All of the 2D graphics data types (not 3D)
    pub use anf_gfx::{
        geom2d::*,
        texture::{SpriteData, SubTextureData2d, TextureData2d},
    };

    pub use fna3d::Color;
}

/// Clears the frame buffer, that is, the screen
pub fn clear_frame(device: &fna3d::Device, clear_color: fna3d::Color) {
    device.clear(fna3d::ClearOptions::TARGET, clear_color.as_vec4(), 0.0, 0);
}
