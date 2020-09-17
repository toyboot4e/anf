//! Pre-defined game loop for arbitrary user data

// The internals need refactoring

use sdl2::{event::Event, EventPump};

use crate::game::app::time::{GameClock, TimeStep};

/// Return type of [`AnfApp::run`]
pub type AnfAppResult = std::result::Result<(), Box<dyn std::error::Error>>;

// --------------------------------------------------------------------------------
// Game loop

/// Game loop that drives [`AnfAppLifecycle`]
#[derive(Debug)]
pub struct AnfApp {
    clock: GameClock,
}

impl AnfApp {
    pub fn new() -> Self {
        Self {
            clock: GameClock::new(),
        }
    }
}

/// Driving lifecycle state
impl AnfApp {
    pub fn run(
        &mut self,
        events: &mut EventPump,
        state: &mut impl AnfAppLifecycle,
    ) -> AnfAppResult {
        while self.tick_one_frame(events, state) {}
        Ok(())
    }

    /// Returns `true` if we continue to the next frame
    fn tick_one_frame(&mut self, events: &mut EventPump, state: &mut impl AnfAppLifecycle) -> bool {
        if !self.pump_events(state, events) {
            return false;
        }

        for time_step in self.clock.tick() {
            state.update(time_step);
        }

        let time_step = self.clock.timestep();
        state.render(time_step);

        state.on_next_frame();

        true
    }

    fn pump_events(&mut self, state: &mut impl AnfAppLifecycle, events: &mut EventPump) -> bool {
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
}

/// Corresponds to `Game` in FNA
pub trait AnfAppLifecycle {
    #[allow(unused_variables)]
    fn event(&mut self, ev: &Event) {}
    #[allow(unused_variables)]
    fn update(&mut self, time_step: TimeStep) {}
    #[allow(unused_variables)]
    fn render(&mut self, time_step: TimeStep) {}
    fn on_next_frame(&mut self) {}
}
