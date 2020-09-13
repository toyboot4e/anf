//! Runs user state

use crate::{
    framework::{
        app::{App, SdlWindowHandle},
        time::{GameClock, TimeStep},
    },
    gfx::api::DrawContext,
    vfs,
};
use sdl2::{event::Event, EventPump};
use std::time::Duration;

/// User data driven by the ANF game loop
pub trait GameState {
    #[allow(unused_variables)]
    fn update(&mut self, ts: TimeStep) {}
    #[allow(unused_variables)]
    fn render(&mut self, ts: TimeStep, dcx: &mut DrawContext) {}
    #[allow(unused_variables)]
    fn listen_event(&mut self, ev: &Event) {}
}

/// Runs application with user's [`GameState`]
pub fn run_game<T: GameState>(app: App, state: T) -> GameResult {
    GameLoop::new(state, app).run()
}

/// Return type of [`run_game`]
pub type GameResult = std::result::Result<(), Box<dyn std::error::Error>>;

/// Drives user's [`GameState`]
struct GameLoop<T: GameState> {
    win: SdlWindowHandle,
    clock: GameClock,
    dcx: DrawContext,
    state: T,
}

/// Device initialization
/// ---
impl<T: GameState> GameLoop<T> {
    fn new(state: T, mut src: App) -> Self {
        self::init_device(&mut src.device, &src.params);
        GameLoop {
            win: src.win,
            clock: GameClock::new(),
            dcx: DrawContext::new(src.device, vfs::default_shader()),
            state,
        }
    }
}

/// Initializes the graphics devices
///
/// FNA3D requires us to set viewport/rasterizer/blend state. **If this is skipped, we can't
/// draw anything** (we only can clear the screen)
fn init_device(
    device: &mut fna3d::Device,
    // batcher: &mut Batcher,
    params: &fna3d::PresentationParameters,
) {
    let viewport = fna3d::Viewport {
        x: 0,
        y: 0,
        w: params.backBufferWidth as i32,
        h: params.backBufferHeight as i32,
        minDepth: 0.0,
        maxDepth: 1.0, // TODO: what's this
    };
    device.set_viewport(&viewport);

    let rst = fna3d::RasterizerState::default();
    device.apply_rasterizer_state(&rst);

    let bst = fna3d::BlendState::alpha_blend();
    device.set_blend_state(&bst);

    // let dst = fna3d::DepthStencilState::default();
    // device.set_depth_stencil_state(&dst);
}

/// Internal type for the game loop
enum UpdateResult {
    Continue,
    Quit,
}

impl<T: GameState> GameLoop<T> {
    fn run(mut self) -> GameResult {
        let mut events = self.win.sdl.event_pump().unwrap();
        log::trace!("Start ANF game loop");

        while self.tick_one_frame(&mut events) {}

        Ok(())
    }

    /// Returns if we continue next frame
    fn tick_one_frame(&mut self, events: &mut EventPump) -> bool {
        // pump events
        for ev in events.poll_iter() {
            match self.handle_event(&ev) {
                UpdateResult::Quit => return false,
                UpdateResult::Continue => {}
                _ => self.state.listen_event(&ev),
            }
        }

        for ts in self.clock.tick() {
            self.state.update(ts);
        }
        self.render(self.clock.timestep());

        // FIXME: timestep handling & `GameTime`
        let fps = 60;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / fps));

        true
    }

    fn render(&mut self, ts: TimeStep) {
        self.state.render(ts, &mut self.dcx);
        self.dcx
            .device
            .swap_buffers(None, None, self.win.raw_window as *mut _);
    }

    /// Just quits on `Escape` key down for now
    fn handle_event(&mut self, ev: &Event) -> UpdateResult {
        match ev {
            Event::Quit { .. } => UpdateResult::Quit,
            ev => {
                self.state.listen_event(&ev);
                UpdateResult::Continue
            }
        }
    }
}
