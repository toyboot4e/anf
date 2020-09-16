//! Base to build sprite push API

mod push;
mod push_cmd;

pub use push::QuadPush;
pub use push_cmd::{QuadPushBinding, SpritePushCommand};

pub mod prelude {
    pub use super::push::{DrawPolicy, QuadPush, Texture2D};
    pub use super::push_cmd::{QuadPushBuilder, Sprite, SubTexture2D};
}
