//! Mouse state
//!
//! * `x1`: first extended mouse button

use sdl2::event::Event;

use super::Double;
use crate::engine::prelude::*;

pub struct Mouse {
    window: *mut sdl2::sys::SDL_Window,
    /// Mouse position and buttons
    mouses: Double<MouseSnapshot>,
    /// Mouse wheels
    wheels: Double<i32>,
}

impl AnfLifecycle for Mouse {
    fn event(&mut self, ev: &Event) -> AnfResult<()> {
        match ev {
            Event::MouseWheel { y, .. } => {
                // 120 units per notch
                self.wheels.b += y * 120;
            }
            _ => {}
        }

        Ok(())
    }

    fn update(&mut self, _time_step: TimeStep) -> AnfResult<()> {
        let mut x = 0;
        let mut y = 0;

        let support_global_mouse_mode = true;

        let flags = unsafe {
            if sdl2::sys::SDL_GetRelativeMouseMode() == sdl2::sys::SDL_bool::SDL_TRUE {
                sdl2::sys::SDL_GetRelativeMouseState(&mut x, &mut y)
            } else if support_global_mouse_mode {
                let flags = sdl2::sys::SDL_GetGlobalMouseState(&mut x, &mut y);
                let (mut wx, mut wy) = (0, 0);
                sdl2::sys::SDL_GetWindowPosition(self.window, &mut wx, &mut wy);
                x -= wx;
                y -= wy;
                flags
            } else {
                // inaccurate
                sdl2::sys::SDL_GetMouseState(&mut x, &mut y)
            }
        };

        // TODO: consider resolution scale
        // x = (i32) ((f32) x * INTERNAL_BackBufferWidth / INTERNAL_WindowWidth);
        // y = (i32) ((f32) y * INTERNAL_BackBufferHeight / INTERNAL_WindowHeight);

        let snapshot = MouseSnapshot { x, y, flags };
        self.mouses.b = snapshot;

        Ok(())
    }

    fn on_end_frame(&mut self) -> AnfResult<()> {
        self.mouses.a = self.mouses.b.clone();
        self.wheels.a = self.wheels.b.clone();

        Ok(())
    }
}

impl Mouse {
    pub fn x(&self) -> i32 {
        self.mouses.b.x()
    }

    pub fn y(&self) -> i32 {
        self.mouses.b.y()
    }

    pub fn is_left_down(&self) -> bool {
        self.mouses.b.is_left_down()
    }

    pub fn is_mid_down(&self) -> bool {
        self.mouses.b.is_mid_down()
    }

    pub fn is_right_down(&self) -> bool {
        self.mouses.b.is_right_down()
    }

    pub fn is_x1_down(&self) -> bool {
        self.mouses.b.is_x1_down()
    }

    pub fn is_x2_down(&self) -> bool {
        self.mouses.b.is_x2_down()
    }
}

impl Mouse {
    pub fn pos(&self) -> [i32; 2] {
        [self.mouses.b.x(), self.mouses.b.y()]
    }

    pub fn pos_delta(&self) -> [i32; 2] {
        [
            self.mouses.b.x() - self.mouses.a.x(),
            self.mouses.b.y() - self.mouses.a.y(),
        ]
    }

    // TODO: scaled mouse position, multiplying resolution scale

    pub fn is_left_pressed(&self) -> bool {
        self.mouses.b.is_left_down() && !self.mouses.a.is_left_down()
    }

    pub fn is_left_released(&self) -> bool {
        !self.mouses.b.is_left_down() && self.mouses.a.is_left_down()
    }

    pub fn is_right_pressed(&self) -> bool {
        self.mouses.b.is_right_down() && !self.mouses.a.is_right_down()
    }

    pub fn is_right_released(&self) -> bool {
        !self.mouses.b.is_right_down() && self.mouses.a.is_right_down()
    }

    pub fn is_mid_pressed(&self) -> bool {
        self.mouses.b.is_mid_down() && !self.mouses.a.is_mid_down()
    }

    pub fn is_mid_released(&self) -> bool {
        !self.mouses.b.is_mid_down() && self.mouses.a.is_mid_down()
    }

    pub fn is_x1_pressed(&self) -> bool {
        self.mouses.b.is_x1_down() && !self.mouses.a.is_x1_down()
    }

    pub fn is_x1_released(&self) -> bool {
        !self.mouses.b.is_x1_down() && self.mouses.a.is_x1_down()
    }

    pub fn is_x2_pressed(&self) -> bool {
        self.mouses.b.is_x2_down() && !self.mouses.a.is_x2_down()
    }

    pub fn is_x2_released(&self) -> bool {
        !self.mouses.b.is_x2_down() && self.mouses.a.is_x2_down()
    }
}

/// Represents a mouse state with cursor position and button press information.
///
/// Basically `sdl2::mouse::MouseState` but backbuffer size and mouse mode.
///
/// * Relative mouse position is relative to the window
/// * Global mouse position is relative to the top-left corner of the desktop
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MouseSnapshot {
    pub x: i32,
    pub y: i32,
    flags: u32,
}

impl MouseSnapshot {
    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    fn mask(button: u32) -> u32 {
        1 << (button - 1)
    }

    pub fn is_left_down(&self) -> bool {
        (self.flags & Self::mask(sdl2::sys::SDL_BUTTON_LEFT)) != 0
    }

    pub fn is_mid_down(&self) -> bool {
        (self.flags & Self::mask(sdl2::sys::SDL_BUTTON_MIDDLE)) != 0
    }

    pub fn is_right_down(&self) -> bool {
        (self.flags & Self::mask(sdl2::sys::SDL_BUTTON_RIGHT)) != 0
    }

    pub fn is_x1_down(&self) -> bool {
        (self.flags & Self::mask(sdl2::sys::SDL_BUTTON_X1)) != 0
    }

    pub fn is_x2_down(&self) -> bool {
        (self.flags & Self::mask(sdl2::sys::SDL_BUTTON_X2)) != 0
    }
}
