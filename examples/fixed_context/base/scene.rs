use anf::engine::prelude::*;

use imgui::{im_str, ImString};
use sdl2::event::Event;

use crate::{
    base::{context::Context, framework::SampleUserDataLifecycle},
    games,
};

pub struct SceneBasedGameData {
    scenes: Vec<Box<dyn SampleUserDataLifecycle<Context>>>,
    current: usize,
}

impl SceneBasedGameData {
    pub fn init(cx: &mut Context) -> Self {
        Self {
            scenes: vec![
                Box::new(games::pong::new_game(&cx.win, &mut cx.dcx)),
                Box::new(games::tiles::new_game(&cx.win, &mut cx.dcx)),
            ],
            current: 0,
        }
    }
}

impl SampleUserDataLifecycle<Context> for SceneBasedGameData {
    #[allow(unused_variables)]
    fn event(&mut self, cx: &mut Context, ev: &Event) -> AnfResult<()> {
        self.scenes[self.current].event(cx, ev)?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn update(&mut self, cx: &mut Context) -> AnfResult<()> {
        self.scenes[self.current].update(cx)?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn render(&mut self, cx: &mut Context) -> AnfResult<()> {
        // cx.dcx.as_mut().set_render_targets(render_targets, num_render_targets, depth_stencil_buffer, depth_format, preserve_target_contents)
        self.scenes[self.current].render(cx)?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn debug_render(&mut self, cx: &mut Context) -> AnfResult<()> {
        let (ui, fin) = {
            let size = cx.win.screen_size(); // (u32, u32)
            let size = [size.0 as f32, size.1 as f32];
            let scale = [1.0, 1.0];
            let dt = 0.016; // FIXME:
            cx.imgui.frame(&cx.win, size, scale, dt)
        };

        let w = imgui::Window::new(im_str!("scenes"))
            .size([120.0, 200.0], imgui::Condition::Once)
            .position([1280.0 - 200.0, 000.0], imgui::Condition::Once)
            .resizable(false);

        w.build(&ui, || {
            let size = [100.0, 20.0];
            for (i, name) in LIST.iter().enumerate() {
                let name: ImString = name.to_string().into();
                if ui.button(&name, size) {
                    self.current = i;
                }
            }
        });

        fin.render(ui, cx.win.as_ref(), cx.dcx.as_mut())?;

        Ok(())
    }
}

const LIST: &[&str] = &["Pong", "Tiled"];
