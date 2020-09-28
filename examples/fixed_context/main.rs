//! ANF sample games

// framework
pub mod base;

pub mod render;
pub mod rl;
pub mod utils;

// specific games
pub mod games;

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
