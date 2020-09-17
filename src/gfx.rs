//! Graphics, data for drawing
//!
//! Graphics API is under [`crate::app::prelude`].

pub use anf_gfx::{
    geom,
    texture::{SpriteData, SubTextureData2D, TextureData2D},
};

pub mod prelude {
    pub use anf_gfx::{
        geom::*,
        texture::{SpriteData, SubTextureData2D, TextureData2D},
    };
}

/// Clears the frame buffer, that is, the screen
pub fn clear_frame(device: &mut impl AsMut<fna3d::Device>, clear_color: fna3d::Color) {
    device
        .as_mut()
        .clear(fna3d::ClearOptions::TARGET, clear_color, 0.0, 0);
}
