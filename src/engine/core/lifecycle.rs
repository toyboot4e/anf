//! The primitive lifecycle

use ::{
    sdl2::{event::Event, EventPump},
    std::time::Duration,
};

use crate::engine::{
    core::{
        clock::*,
        window::{WindowConfig, WindowHandle},
    },
    draw::*,
};

/// ANF framework return type
pub type AnfResult<T> = ::anyhow::Result<T>;

/// Primitive lifecycle run by [`AnfFramework`]
///
/// Users are encouraged to build their own framework on top of it, maybe specifying stages such as
/// `debug_render`.
pub trait AnfLifecycle {
    // TODO: lifecycle with `EventPump` with window?
    #[allow(unused_variables)]
    fn event(&mut self, ev: &Event) -> AnfResult<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn update(&mut self, dt: Duration) -> AnfResult<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn render(&mut self, dt: Duration) -> AnfResult<()> {
        Ok(())
    }

    fn on_end_frame(&mut self) -> AnfResult<()> {
        Ok(())
    }
}

/// The entry point of ANF application; runs a game that implemenents [`AnfLifecycle`]
pub struct AnfFramework {
    cfg: WindowConfig,
    window: WindowHandle,
    events: sdl2::EventPump,
    dcx: DrawContext,
}

impl AnfFramework {
    pub fn from_cfg(cfg: WindowConfig) -> Self {
        let (mut window, dcx) = {
            let (window, device, params) = crate::engine::core::init(&cfg);
            let dcx = DrawContext::new(device, crate::engine::embedded::SPRITE_EFFECT, params);
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
        gen_user_data: impl FnOnce(WindowHandle, &WindowConfig, DrawContext) -> T,
    ) -> AnfResult<()> {
        let AnfFramework {
            cfg,
            window,
            dcx,
            mut events,
        } = self;

        let mut state = gen_user_data(window, &cfg, dcx);

        self::run_game_loop(&mut events, &mut state)
    }
}

fn run_game_loop(events: &mut EventPump, state: &mut impl AnfLifecycle) -> AnfResult<()> {
    // HACK: skip the first 1 frame so that the window opens
    if self::pump_events(state, events)? {
        unreachable!();
    }

    let mut clock = GameClock::new();

    loop {
        if self::pump_events(state, events)? {
            return Ok(()); // close the game window
        }

        for dt in clock.tick() {
            state.update(dt)?;
        }

        let time_step = clock.timestep_draw();
        state.render(time_step)?;

        state.on_end_frame()?;
    }
}

/// Returns `true` if the window should be closed
fn pump_events(state: &mut impl AnfLifecycle, events: &mut EventPump) -> AnfResult<bool> {
    for ev in events.poll_iter() {
        match ev {
            Event::Quit { .. } => return Ok(true),
            ev => {
                state.event(&ev)?;
            }
        }
    }
    Ok(false)
}
