/*!

Graphics data types

These structs implement draw API traits in [`crate::engine::draw`] so they can be drawn via
[`DrawContext`].

[`DrawContext`]: crate::engine::draw::DrawContext

*/

pub use {
    anf_gfx::{
        geom2d, geom3d,
        texture::{SpriteData, SubTextureData2d, Texture2dDrop, TextureData2d},
    },
    fna3h::{draw::pass::ClearOptions, Color, Device, Vec4},
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

pub struct ClearCommand {
    pub color: Option<Vec4>,
    pub depth: Option<f32>,
    pub stencil: Option<i32>,
}

impl ClearCommand {
    pub fn run(&self, device: &Device) {
        let mut opts = ClearOptions::empty();

        opts.set(ClearOptions::TARGET, self.color.is_some());
        opts.set(ClearOptions::DEPTH_BUFFER, self.depth.is_some());
        opts.set(ClearOptions::STENCIL, self.stencil.is_some());

        if opts == ClearOptions::empty() {
            return;
        }

        let color = self.color.unwrap_or_default();
        let stencil = self.stencil.unwrap_or_default();
        let depth = self.depth.unwrap_or_default();

        device.clear(opts, color, depth, stencil);
    }

    /// Clears the frame buffer, that is, the screen
    pub fn color(device: &fna3h::Device, clear_color: Color) {
        device.clear(ClearOptions::TARGET, clear_color.to_vec4(), 0.0, 0);
    }
}
