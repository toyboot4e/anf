//! Quad rendering command and API

mod params;
mod params_build;

// data types
pub use self::{
    params::{DrawPolicy, QuadParams},
    params_build::{QuadPush, SpritePush},
};

pub mod traits {
    pub use super::{
        params::Texture2d,
        params_build::{OnSpritePush, QuadParamsBuilder, Sprite, SubTexture2d},
    };
}
