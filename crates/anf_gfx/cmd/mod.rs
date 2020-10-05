//! Quad rendering command and API

mod push_cmd;
mod push_params;

pub use push_cmd::{QuadPush, SpritePush};
pub use push_params::QuadParams;

pub mod prelude {
    //! Traits to push quadliterals

    pub use super::push_cmd::{QuadParamsBuilder, Sprite, SubTexture2d};
    pub use super::push_params::{DrawPolicy, QuadParams, Texture2d};
}
