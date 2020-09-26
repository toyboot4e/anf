//! Example tiled game

use std::fs;

use anf::{engine::prelude::*, gfx::prelude::*, vfs};
use sdl2::event::Event;

pub use tiled::{Image, Layer, Map, Tile, Tileset};

use crate::{
    base::{context::Context, framework::SampleUserDataLifecycle},
    grid2d::Rect2i,
    render::tiled as tiled_render,
};

pub struct TiledGameData {
    map: Map,
    texture: TextureData2d,
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
        tiled_render::render_tiled(
            &mut cx.dcx,
            &self.map,
            &self.texture,
            Rect2i::new([0, 0], [1280, 720]),
        );
        Ok(())
    }

    #[allow(unused_variables)]
    fn debug_render(&mut self, cx: &mut Context) -> AnfResult<()> {
        Ok(())
    }
}

pub fn new_game(win: &WindowHandle, dcx: &mut DrawContext) -> TiledGameData {
    let path = vfs::path("map/tmx/1.tmx");
    let file = fs::File::open(&path).unwrap();

    let tiles = TextureData2d::from_path(dcx.as_mut(), vfs::path("map/images/nekura_1/m_mura.png"))
        .unwrap();

    TiledGameData {
        map: tiled::parse_with_path(file, &path).unwrap(),
        texture: tiles,
    }
}
