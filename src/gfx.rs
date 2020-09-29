//! Data for drawing
//!
//! These structs implement draw API traits in [`crate::engine::draw`] so they can be pushed via
//! [`DrawContext`].
//!
//! [`DrawContext`]: crate::engine::draw::DrawContext

pub use anf_gfx::{
    geom2d, geom3d,
    texture::{SpriteData, SubTextureData2d, TextureData2d},
};

pub mod prelude {
    //! All of the graphics data types
    pub use anf_gfx::{
        geom2d::*,
        texture::{SpriteData, SubTextureData2d, TextureData2d},
    };
}

/// Clears the frame buffer, that is, the screen
pub fn clear_frame(device: &mut impl AsMut<fna3d::Device>, clear_color: fna3d::Color) {
    device
        .as_mut()
        .clear(fna3d::ClearOptions::TARGET, clear_color, 0.0, 0);
}
