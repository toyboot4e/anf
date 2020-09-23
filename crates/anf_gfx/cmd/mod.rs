//! Base to build sprite push API

mod push_cmd;
mod push_params;

pub use push_cmd::{QuadPushBinding, SpritePushCommand};
pub use push_params::QuadPush;

pub mod prelude {
    pub use super::push_cmd::{QuadPushBuilder, Sprite, SubTexture2d};
    pub use super::push_params::{DrawPolicy, QuadPush, Texture2d};
}
