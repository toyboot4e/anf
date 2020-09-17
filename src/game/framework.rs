use crate::{
    game::{app::*, draw::*},
    vfs,
};

/// Entry point of ANF game loop with [`DrawContext`]
pub struct AnfFramework {
    cfg: WindowConfig,
    window: WindowHandle,
    events: sdl2::EventPump,
    dcx: DrawContext,
}

impl AnfFramework {
    pub fn from_cfg(cfg: WindowConfig) -> Self {
        let (mut window, dcx) = {
            // construct SDL window handle and FNA3D device
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

    pub fn run<T: AnfAppLifecycle>(
        self,
        user_data_constructor: impl FnOnce(WindowHandle, &WindowConfig, DrawContext) -> T,
    ) -> AnfAppResult {
        let AnfFramework {
            cfg,
            window,
            dcx,
            mut events,
        } = self;

        let mut app = AnfApp::new();
        let mut state = user_data_constructor(window, &cfg, dcx);
        app.run(&mut events, &mut state)
    }
}
