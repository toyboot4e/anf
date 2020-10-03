//! Virtual input, bundles of input states
//!
//! Virtual inputs are defined as queries because
//!
//! # Usage
//!
//! It's good for typical input abstraction. For example, your "select key" may be any of enter,
//! space, some gamepad button or even left click. Then virtual input is perfect for bundling them.
//!
//! However, they are not generic enough. For example, you might want to handle left click in a
//! different way from enter key. Then you have to build your custom input system like UI commands,
//! maybe on top of virtual input.

use crate::{
    engine::lifecycle::AnfLifecycle,
    input::{
        axis::{Dir4, Dir8},
        Key, Keyboard, Mouse, MouseInput,
    },
};

pub struct InputBundle {
    keys: KeyBundle,
    mouse: MouseBundle,
}

/// Some value that is decided by a set of [`Key`]'s state
#[derive(Debug, Clone)]
pub struct KeyBundle {
    keys: Vec<Key>,
}

impl KeyBundle {
    pub fn is_down(&self, kbd: &Keyboard) -> bool {
        kbd.is_any_key_down(&self.keys)
    }
}

#[derive(Debug, Clone)]
pub struct MouseBundle {
    inputs: Vec<MouseInput>,
}

impl MouseBundle {
    pub fn is_down(&self, mouse: &Mouse) -> bool {
        mouse.is_any_down(&self.inputs)
    }
}

/// Negative or positive in one direction
#[derive(Debug, Clone)]
pub struct AxisInput {
    pos: KeyBundle,
    neg: KeyBundle,
}

/// One of [`Dir4`]
#[derive(Debug, Clone)]
pub struct FourDirButton {
    x: AxisInput,
    y: AxisInput,
}

/// One of [`Dir8`]
#[derive(Debug, Clone)]
pub struct EightDirButton {
    x: AxisInput,
    y: AxisInput,
}
