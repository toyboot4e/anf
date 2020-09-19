//! ANF requires user to offer state and context. This example uses

mod context;
mod games;

use anf::game::{AnfGame, AnfGameResult};
use context::Context;

fn main() -> AnfGameResult {
    env_logger::init();
    AnfGame::run(
        games::pong::config(),
        Context::new,
        games::pong::PongGameData::from_cx,
    )
}
