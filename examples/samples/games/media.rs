//! Media

use anf::{engine::prelude::*, vfs};
use soloud::{audio::Wav, prelude::*, Soloud};

use crate::base::{context::Context, framework::SampleUserDataLifecycle};

pub struct MediaGameData {
    audio: Soloud,
    select: Wav,
}

impl SampleUserDataLifecycle<Context> for MediaGameData {
    fn update(&mut self, cx: &mut Context) -> AnfResult<()> {
        // TODO: timer
        // self.audio.play(&self.select);

        Ok(())
    }

    fn render(&mut self, cx: &mut Context) -> AnfResult<()> {
        anf::gfx::clear_frame(&mut cx.dcx, fna3d::Color::rgb(150, 177, 86));
        Ok(())
    }
}

pub enum Shape {
    Circle { r: f32 },
}

pub fn new_game(win: &WindowHandle, dcx: &mut DrawContext) -> MediaGameData {
    let audio = Soloud::default().expect("failed to init Soloud");

    let path = vfs::path("sounds/select.wav");
    let select = Wav::from_path(&path).unwrap();

    MediaGameData {
        audio,
        select: select,
    }
}
