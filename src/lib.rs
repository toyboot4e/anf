//! ANF is an FNA-like 2D game framework in Rust powered by FNA3D
//!
//! ANF is also intended to introduce FNA3D so the documentation goes into internals details.
//!
//! * TODO: free memory on neessary
//! * TODO: copy FNA3D to output
//! * TODO: copy `assets/` to output
//! * TODO: FPS

pub use fna3d;
pub use sdl2;

pub mod gfx;
pub mod vfs;
pub mod window;

use std::time::Duration;

/// State with methods called from the game loop (`run_loop`)
pub trait State {
    fn update(&mut self);
    fn render(&mut self);
    fn handle_event(&mut self, ev: &sdl2::event::Event) -> StateUpdateResult;
}

pub enum StateUpdateResult {
    Continue,
    Quit,
}

/// The final notification from the application returned by `run_loop`
pub type GameResult = std::result::Result<(), Box<dyn std::error::Error>>;

/// The game loop
pub fn run_loop(state: &mut impl State, scx: &mut sdl2::Sdl) -> GameResult {
    let mut events = scx.event_pump().unwrap();
    log::trace!("Start game loop");

    'main_loop: loop {
        // pump events
        for ev in events.poll_iter() {
            match state.handle_event(&ev) {
                StateUpdateResult::Quit => break 'main_loop,
                StateUpdateResult::Continue => {}
            }
        }

        state.update();
        state.render();

        // FIXME: timestep handling & `GameTime`
        let fps = 60;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / fps));
    }

    Ok(())
}
