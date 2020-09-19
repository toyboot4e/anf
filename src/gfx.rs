//! Data for drawing
//!
//! These structs implement draw API traits in [`crate::game::draw`] so they can be pushed via
//! [`crate::game::draw::DrawContext`].

pub use anf_gfx::{
    geom2d,
    texture::{SpriteData, SubTextureData2D, TextureData2D},
};

pub mod prelude {
    //! All of the graphics data types
    pub use anf_gfx::{
        geom2d::*,
        texture::{SpriteData, SubTextureData2D, TextureData2D},
    };
}

/// Clears the frame buffer, that is, the screen
pub fn clear_frame(device: &mut impl AsMut<fna3d::Device>, clear_color: fna3d::Color) {
    device
        .as_mut()
        .clear(fna3d::ClearOptions::TARGET, clear_color, 0.0, 0);
}
