//! Event/update/render lifecycle
//!
//! Build your own lifecycle on top of it.

// The internals need refactoring

use sdl2::{event::Event, EventPump};

use crate::engine::{app::*, draw::*, time::*};

const DEFAULT_SHADER: &[u8] = include_bytes!("SpriteEffect.fxb");

/// Return type of ANF game
pub type AnfResult<T> = anyhow::Result<T>;

/// Lifecycle provided by [`AnfFramework`]
///
/// Users are encouraged to build their own framework on top of it maybe specifying stages such as
/// `debug_render`.
pub trait AnfLifecycle {
    // TODO: lifecycle with `EventPump` with window?
    #[allow(unused_variables)]
    fn event(&mut self, ev: &Event) -> AnfResult<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn update(&mut self, time_step: TimeStep) -> AnfResult<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn render(&mut self, time_step: TimeStep) -> AnfResult<()> {
        Ok(())
    }

    fn on_end_frame(&mut self) -> AnfResult<()> {
        Ok(())
    }
}

/// Runs the primitive [`AnfLifecycle`]
///
/// The entry point of ANF application.
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
            let dcx = DrawContext::new(device, DEFAULT_SHADER, params);
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
    ) -> AnfResult<()> {
        let AnfFramework {
            cfg,
            window,
            dcx,
            mut events,
        } = self;

        let mut clock = GameClock::new();
        let mut state = user_data_constructor(window, &cfg, dcx);

        self::visit_lifecycle(&mut clock, &mut events, &mut state)
    }
}

fn visit_lifecycle(
    clock: &mut GameClock,
    events: &mut EventPump,
    state: &mut impl AnfLifecycle,
) -> AnfResult<()> {
    while tick_one_frame(clock, events, state)? {}
    Ok(())
}

/// Returns `true` if we continue to the next frame
fn tick_one_frame(
    clock: &mut GameClock,
    events: &mut EventPump,
    state: &mut impl AnfLifecycle,
) -> AnfResult<bool> {
    if !self::pump_events(state, events)? {
        return Ok(false);
    }

    for time_step in clock.tick() {
        state.update(time_step)?;
    }

    let time_step = clock.timestep();
    state.render(time_step)?;

    state.on_end_frame()?;

    Ok(true)
}

fn pump_events(state: &mut impl AnfLifecycle, events: &mut EventPump) -> AnfResult<bool> {
    for ev in events.poll_iter() {
        match ev {
            Event::Quit { .. } => return Ok(false),
            ev => {
                state.event(&ev)?;
            }
        }
    }
    Ok(true)
}
