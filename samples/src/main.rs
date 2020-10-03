use anf::engine::prelude::*;

use anf_samples::{
    base::{context::Context, framework::SampleFramework},
    scene::SceneBasedGameData,
};

fn main() -> AnfResult<()> {
    env_logger::init();
    SampleFramework::run(self::config(), Context::init, SceneBasedGameData::init)
}

/// Creates initial window configuration
pub fn config() -> WindowConfig {
    WindowConfig {
        title: "ANF samples".to_string(),
        w: 1280,
        h: 720,
        ..Default::default()
    }
}
