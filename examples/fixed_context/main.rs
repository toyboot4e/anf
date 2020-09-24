//! ANF requires user to offer state and context. This example uses

mod base;
mod games;
mod grid2d;

use anf::prelude::*;

use base::{context::Context, framework::SampleFramework, scene::SceneBasedGameData};

fn main() -> AnfResult<()> {
    env_logger::init();
    SampleFramework::run(self::config(), Context::init, SceneBasedGameData::init)
}

pub fn config() -> WindowConfig {
    WindowConfig {
        title: "ANF samples".to_string(),
        w: 1280,
        h: 720,
        ..Default::default()
    }
}
