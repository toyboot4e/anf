//! Virtual input, bundles of input states
//!
//! # Usage
//!
//! It's good for typical input abstraction. For example, your "select key" may be any of enter,
//! space, gamepad button or even left click. Then virtual input is perfect for bundling them.
//!
//! However, they are not generic enough. For example, you might want to handle left click in a
//! different way from enter key. Then you have to build your custom input system like UI commands,
//! maybe on top of virtual input.

use crate::input::Key;

pub struct VKey {
    keys: Vec<Key>,
}

pub struct AxisInput {
    pos: VKey,
    neg: VKey,
}

pub enum Dir4 {
    Up,
    Right,
    Down,
    Left,
}

pub enum Dir8 {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

pub struct EightDirInput {
    x: AxisInput,
    y: AxisInput,
}

pub struct FourDirInput {
    x: AxisInput,
    y: AxisInput,
}
