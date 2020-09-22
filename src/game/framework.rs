//! Thin layer of game loop with `DrawContext`

// The internals need refactoring

use sdl2::{event::Event, EventPump};

use crate::{
    game::{app::*, draw::*, time::*},
    vfs,
};

/// Return type of ANF game
pub type AnfGameResult = std::result::Result<(), Box<dyn std::error::Error>>;

/// Where we manage user framework context
pub trait AnfLifecycle {
    #[allow(unused_variables)]
    fn event(&mut self, ev: &Event) {}
    #[allow(unused_variables)]
    fn update(&mut self, time_step: TimeStep) {}
    #[allow(unused_variables)]
    fn render(&mut self, time_step: TimeStep) {}
    fn on_end_frame(&mut self) {}
}

/// The entry point of the ANF game loop
pub struct AnfFramework {
    cfg: WindowConfig,
    window: WindowHandle,
    events: sdl2::EventPump,
    dcx: DrawContext,
}

impl AnfFramework {
    pub fn from_cfg(cfg: WindowConfig) -> Self {
        let (mut window, dcx) = {
            let (window, device, params) = init_app(&cfg);
            let dcx = DrawContext::new(device, vfs::default_shader(), params);
            (window, dcx)
        };
        let events = window.event_pump().unwrap();
        Self {
            cfg,
            window,
            events,
            dcx,
        }
    }

    pub fn run<T: AnfLifecycle>(
        self,
        user_data_constructor: impl FnOnce(WindowHandle, &WindowConfig, DrawContext) -> T,
    ) -> AnfGameResult {
        let AnfFramework {
            cfg,
            window,
            dcx,
            mut events,
        } = self;

        let mut clock = GameClock::new();
        let mut state = user_data_constructor(window, &cfg, dcx);

        Self::visit(&mut clock, &mut events, &mut state)
    }

    fn visit(
        clock: &mut GameClock,
        events: &mut EventPump,
        state: &mut impl AnfLifecycle,
    ) -> AnfGameResult {
        while tick_one_frame(clock, events, state) {}
        Ok(())
    }
}

/// Returns `true` if we continue to the next frame
fn tick_one_frame(
    clock: &mut GameClock,
    events: &mut EventPump,
    state: &mut impl AnfLifecycle,
) -> bool {
    if !self::pump_events(state, events) {
        return false;
    }

    // FIXME: provide handle of game clock (so that FPS can be changed)
    for time_step in clock.tick() {
        state.update(time_step);
    }

    let time_step = clock.timestep();
    state.render(time_step);

    state.on_end_frame();

    true
}

fn pump_events(state: &mut impl AnfLifecycle, events: &mut EventPump) -> bool {
    for ev in events.poll_iter() {
        match ev {
            Event::Quit { .. } => return false,
            ev => {
                state.event(&ev);
            }
        }
    }
    true
}
