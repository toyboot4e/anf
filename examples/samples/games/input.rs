use anf::engine::prelude::*;

use crate::base::{context::Context, framework::SampleUserDataLifecycle};

pub struct InputGameData {}

impl SampleUserDataLifecycle<Context> for InputGameData {
    fn render(&mut self, cx: &mut Context) -> AnfResult<()> {
        anf::gfx::clear_frame(&mut cx.dcx, fna3d::Color::rgb(150, 177, 86));
        Ok(())
    }
}

pub enum Shape {
    Circle { r: f32 },
}

pub fn new_game(win: &WindowHandle, dcx: &mut DrawContext) -> InputGameData {
    InputGameData {}
}
