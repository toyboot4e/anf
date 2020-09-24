//! ANF requires user to offer state and context. This example uses

mod context;
mod framework;
mod games;

use anf::prelude::*;

use self::{context::Context, framework::SampleFramework, games::pong::PongGameData};

fn main() -> AnfResult<()> {
    env_logger::init();
    SampleFramework::run(self::config(), Context::init, PongGameData::from_cx)
}

pub fn config() -> WindowConfig {
    WindowConfig {
        title: "ANF samples".to_string(),
        w: 1280,
        h: 720,
        ..Default::default()
    }
}
