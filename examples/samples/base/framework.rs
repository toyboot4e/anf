//! A 2D framework built on top of ANF
//!
//! User provide concrete _context_ and _user data_.
//!
//! This framework should be modified by user's needs.

use sdl2::event::Event;

use anf::engine::{
    app::{WindowConfig, WindowHandle},
    draw::*,
    lifecycle::{AnfFramework, AnfLifecycle, AnfResult},
    time::TimeStep,
};

pub trait SampleContextLifecycle {
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

    fn debug_render(&mut self) {}
}

pub trait SampleUserDataLifecycle<T> {
    #[allow(unused_variables)]
    fn event(&mut self, cx: &mut T, ev: &Event) -> AnfResult<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn update(&mut self, cx: &mut T) -> AnfResult<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn render(&mut self, cx: &mut T) -> AnfResult<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn debug_render(&mut self, cx: &mut T) -> AnfResult<()> {
        Ok(())
    }
}

/// Creates sample context/user-data lifecycle on top of [`AnfLifecycle`]
pub struct SampleFramework<T: SampleContextLifecycle, U: SampleUserDataLifecycle<T>> {
    cx: T,
    user: U,
}

impl<T: SampleContextLifecycle, U: SampleUserDataLifecycle<T>> SampleFramework<T, U> {
    pub fn run(
        cfg: WindowConfig,
        cx: impl FnOnce(WindowHandle, &WindowConfig, DrawContext) -> T,
        state: impl FnOnce(&mut T) -> U,
    ) -> AnfResult<()> {
        AnfFramework::from_cfg(cfg).run(|win, cfg, dcx| {
            let mut cx = cx(win, cfg, dcx);
            let user = state(&mut cx);
            Self { user, cx }
        })
    }
}

/// Sample context/user-data lifecycle implementation built on top of [`AnfLifecycle`]
impl<T: SampleContextLifecycle, U: SampleUserDataLifecycle<T>> AnfLifecycle
    for SampleFramework<T, U>
{
    fn event(&mut self, ev: &Event) -> AnfResult<()> {
        self.cx.event(ev)?;
        Ok(())
    }

    fn update(&mut self, time_step: TimeStep) -> AnfResult<()> {
        self.cx.update(time_step)?;
        self.user.update(&mut self.cx)?;
        Ok(())
    }

    fn render(&mut self, time_step: TimeStep) -> AnfResult<()> {
        self.cx.render(time_step)?;
        self.user.render(&mut self.cx)?;
        self.cx.debug_render();
        self.user.debug_render(&mut self.cx)?;
        Ok(())
    }

    fn on_end_frame(&mut self) -> AnfResult<()> {
        self.cx.on_end_frame()?;
        Ok(())
    }
}
