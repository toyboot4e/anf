//! ANF sample games

// --------------------------------------------------------------------------------
// modules

// framework
pub mod base;

// libraries
pub mod render;
pub mod rl;
pub mod utils;

// specific games
pub mod games;

// --------------------------------------------------------------------------------
// main

use anf::engine::prelude::*;
use base::{context::Context, framework::SampleFramework};

mod scene;
use scene::SceneBasedGameData;

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
