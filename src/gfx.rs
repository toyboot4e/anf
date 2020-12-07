//! Graphics data types
//!
//! These structs implement draw API traits in [`crate::engine::draw`] so they can be drawn via
//! [`DrawContext`].
//!
//! [`DrawContext`]: crate::engine::draw::DrawContext

pub use ::{
    anf_gfx::{
        geom2d, geom3d,
        texture::{SpriteData, SubTextureData2d, Texture2dDrop, TextureData2d},
    },
    fna3h::{draw::pass::ClearOptions, Color, Device},
};

#[cfg(feature = "font")]
pub use fna3d_fontstash as font;

pub mod prelude {
    //! All of the 2D graphics data types (not 3D)
    pub use anf_gfx::{
        geom2d::*,
        texture::{SpriteData, SubTextureData2d, Texture2dDrop, TextureData2d},
    };

    pub use fna3h::Color;
}

/// Clears the frame buffer, that is, the screen
pub fn clear_frame(device: &fna3h::Device, clear_color: Color) {
    device.clear(ClearOptions::TARGET, clear_color.to_vec4(), 0.0, 0);
}
