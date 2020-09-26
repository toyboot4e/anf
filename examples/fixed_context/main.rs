//! ANF sample games

// framework
mod base;
mod grid2d;
mod render;

// specific games
mod games;

use anf::engine::prelude::*;

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
