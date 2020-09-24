//! Example tiled game

use anf::prelude::*;
use sdl2::event::Event;

use crate::{context::Context, framework::SampleUserDataLifecycle};

pub struct TiledGameData {
    //
}

impl SampleUserDataLifecycle<Context> for TiledGameData {
    #[allow(unused_variables)]
    fn event(&mut self, cx: &mut Context, ev: &Event) -> AnfResult<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn update(&mut self, cx: &mut Context) -> AnfResult<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn render(&mut self, cx: &mut Context) -> AnfResult<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn debug_render(&mut self, cx: &mut Context) -> AnfResult<()> {
        Ok(())
    }
}

pub fn new_game(win: &WindowHandle, dcx: &mut DrawContext) -> TiledGameData {
    TiledGameData {}
}
