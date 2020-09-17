//! [`AnfFramework`] and [`AnfResult`]

use std::time::{Duration, Instant};

use sdl2::{event::Event, sys::SDL_Window, EventPump};

use crate::{app::prelude::*, vfs};

// --------------------------------------------------------------------------------
// Public types

/// Return type of [`AnfFramework::run`]
pub type AnfResult = std::result::Result<(), Box<dyn std::error::Error>>;

/// Drives user state that implements [`AnfLifecycle`]
pub struct AnfFramework {
    cfg: WindowConfig,
    window: WindowHandle,
    events: sdl2::EventPump,
    game_loop: AnfGameLoop,
}

impl AnfFramework {
    pub fn default() -> Self {
        Self::with_cfg(WindowConfig::default())
    }

    pub fn with_cfg(cfg: WindowConfig) -> Self {
        let (mut window, game_loop) = {
            // construct SDL window handle and FNA3D device
            let (window, device, params) = self::init_window(&cfg);

            let game_loop = {
                let dcx = DrawContext::new(device, vfs::default_shader(), params);
                AnfGameLoop::new(window.raw_window(), dcx)
            };

            (window, game_loop)
        };

        let events = window.event_pump().unwrap();

        Self {
            cfg,
            window,
            events,
            game_loop,
        }
    }

    pub fn run<T: AnfLifecycle>(
        self,
        user_data_constructor: impl FnOnce(WindowHandle, &WindowConfig, &mut DrawContext) -> T,
    ) -> AnfResult {
        let AnfFramework {
            cfg,
            window,
            mut game_loop,
            mut events,
        } = self;

        let mut state = user_data_constructor(window, &cfg, &mut game_loop.dcx);
        while game_loop.tick_one_frame(&mut state, &mut events) {}
        Ok(())
    }
}

/// Returns `(window, device, params): (SdlWindowHandle, fna3d::Device, fna3d::PresentationParameters)`
fn init_window(cfg: &WindowConfig) -> (WindowHandle, fna3d::Device, fna3d::PresentationParameters) {
    // setup FNA3D
    log::info!("FNA version {}", fna3d::linked_version());
    fna3d::utils::hook_log_functions_default();

    let win = WindowHandle::from_cfg(&cfg);
    let (params, device) = create_fna3d_device(cfg, win.raw_window());
    return (win, device, params);

    fn create_fna3d_device(
        cfg: &WindowConfig,
        win: *mut SDL_Window,
    ) -> (fna3d::PresentationParameters, fna3d::Device) {
        let params = {
            let mut params = fna3d::utils::default_params_from_window_handle(win as *mut _);
            params.backBufferWidth = cfg.w as i32;
            params.backBufferHeight = cfg.h as i32;
            params
        };
        let mut device = fna3d::Device::from_params(params, cfg.is_debug);
        init_device(&mut device, &params);
        return (params, device);

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
        }
    }
}

// --------------------------------------------------------------------------------
// Game loop

/// Drives user state that implements [`AnfLifecycle`]
#[derive(Debug)]
struct AnfGameLoop {
    raw_window: *mut SDL_Window,
    pub clock: GameClock,
    pub dcx: DrawContext,
}

impl AnfGameLoop {
    pub fn new(raw_window: *mut SDL_Window, dcx: DrawContext) -> Self {
        Self {
            raw_window,
            clock: GameClock::new(),
            dcx,
        }
    }
}

impl AsMut<fna3d::Device> for AnfGameLoop {
    fn as_mut(&mut self) -> &mut fna3d::Device {
        self.dcx.as_mut()
    }
}

/// Driving lifecycle state
impl AnfGameLoop {
    /// The game loop. Returns `true` if we continue to the next frame
    pub fn tick_one_frame(
        &mut self,
        state: &mut impl AnfLifecycle,
        events: &mut EventPump,
    ) -> bool {
        if !self.pump_events(state, events) {
            return false;
        }
        self.update(state);
        self.render(state);
        state.on_next_frame();

        true
    }

    fn pump_events(&mut self, state: &mut impl AnfLifecycle, events: &mut EventPump) -> bool {
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

    fn update(&mut self, state: &mut impl AnfLifecycle) {
        let screen = self.dcx.screen();
        for time_step in self.clock.tick() {
            let ucx = UpdateContext {
                time_step,
                screen: screen.clone(),
            };
            state.update(&ucx);
        }
    }

    fn render(&mut self, state: &mut impl AnfLifecycle) {
        self.dcx.time_step = self.clock.timestep();
        state.render(&mut self.dcx);
        self.dcx
            .as_mut()
            .swap_buffers(None, None, self.raw_window as *mut _);
    }
}

/// Creates frames
#[derive(Debug, Clone)]
struct GameClock {
    // states
    time_step: TimeStep,
    accum: Duration,
    total: Duration,
    last_time: Instant,
    lag: u32,
    is_slow: bool,
    // configuration
    is_fixed_timestep: bool,
    updates_per_sec: f64,
}

impl GameClock {
    pub fn new() -> Self {
        Self {
            time_step: TimeStep::new(),
            accum: Duration::new(0, 0),
            total: Duration::new(0, 0),
            last_time: Instant::now(),
            lag: 0,
            is_slow: false,
            is_fixed_timestep: true,
            updates_per_sec: 60.0,
        }
    }

    /// TODO: is this accurate?
    fn target_elapsed(&self) -> Duration {
        Duration::from_secs_f64(1.0 / self.updates_per_sec)
    }

    const fn max_elapsed() -> Duration {
        Duration::from_millis(500)
    }

    pub fn tick(&mut self) -> GameClockTick {
        let elapsed = {
            let mut elapsed = self.wait_for_next_frame(self.accum);
            // Do not allow any update to take longer than our maximum.
            if elapsed > Self::max_elapsed() {
                elapsed = Self::max_elapsed();
            }
            elapsed
        };
        self.accum = elapsed;

        GameClockTick::new(self)
    }

    pub fn timestep(&self) -> TimeStep {
        self.time_step.clone()
    }

    fn wait_for_next_frame(&mut self, mut elapsed: Duration) -> Duration {
        loop {
            // Advance the accumulated elapsed time.
            let now = Instant::now();
            elapsed += now.duration_since(self.last_time);
            self.last_time = now;

            if !self.is_fixed_timestep {
                return elapsed;
            }

            let target_elapsed = self.target_elapsed();
            if elapsed > target_elapsed {
                break elapsed;
            }

            // sleep (inaccurate but enough for making frames)
            let remaining = target_elapsed - elapsed;
            std::thread::sleep(remaining);
        }
    }
}

/// Iterator of one frame
struct GameClockTick<'a> {
    clock: &'a mut GameClock,
    n_updates: u32,
}

impl<'a> GameClockTick<'a> {
    fn new(clock: &'a mut GameClock) -> Self {
        clock.time_step.elapsed = clock.target_elapsed();
        GameClockTick {
            clock,
            n_updates: 0,
        }
    }

    fn next_fixed(&mut self) -> Option<TimeStep> {
        let target_elapsed = self.clock.target_elapsed();

        // Perform as many full fixed length time steps as we can
        if self.clock.accum >= target_elapsed {
            self.clock.total += target_elapsed;
            self.clock.accum -= target_elapsed;
            self.n_updates += 1;
            return Some(self.clock.time_step.clone());
        }

        // Every update after the first accumulates lag
        if self.n_updates > 0 {
            self.clock.lag += self.n_updates - 1;
        }

        // If we think we are running slowly, wait
        // until the lag clears before resetting it
        match (self.clock.is_slow, self.clock.lag) {
            (true, 0) => self.clock.is_slow = false,
            (false, lag) if lag >= 5 => self.clock.is_slow = true,
            _ => {}
        };

        // Every time we just do one update and one draw,
        // then we are not running slowly, so decrease the lag.
        if self.n_updates == 1 && self.clock.lag > 0 {
            self.clock.lag -= 1;
        }

        self.clock.time_step.elapsed = target_elapsed * self.n_updates;

        None
    }

    fn next_variable(&mut self) -> Option<TimeStep> {
        if self.n_updates > 0 {
            return None;
        }

        // Perform a single variable length update.
        // if forceElapsedTimeToZero {
        //     // When ResetElapsedTime is called, Elapsed is forced to zero and Total is ignored entirely
        //     time.elapsed = Duration::new(0, 0.0);
        //     forceElapsedTimeToZero = false;
        // } else {
        self.clock.time_step.elapsed = self.clock.accum;
        self.clock.total += self.clock.accum;
        // }

        self.clock.time_step.elapsed = Duration::new(0, 0);
        // AssertNotDisposed();
        self.n_updates = 1;

        Some(self.clock.time_step.clone())
    }
}

impl<'a> Iterator for GameClockTick<'a> {
    type Item = TimeStep;

    fn next(&mut self) -> Option<Self::Item> {
        if self.clock.is_fixed_timestep {
            self.next_fixed()
        } else {
            self.next_variable()
        }
    }
}
