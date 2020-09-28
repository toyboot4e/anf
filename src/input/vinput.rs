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

use crate::{engine::lifecycle::AnfLifecycle, input::Key};

pub struct InputBundle {
    key: KeyBundle,
}

/// Some value that is decided by a set of [`Key`]'s state
#[derive(Debug, Clone)]
pub struct KeyBundle {
    keys: Vec<Key>,
    is_down: bool,
}

impl KeyBundle {
    // pub fn is_down(&self) -> bool {
    //     //
    // }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    Up,
    Down,
    None,
}

/// Negative or positive in one direction
#[derive(Debug, Clone)]
pub struct AxisInput {
    pos: KeyBundle,
    neg: KeyBundle,
}

/// Positive, negative or neutral
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Sign {
    Pos,
    Neg,
    Neutral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dir4 {
    Up,
    Right,
    Down,
    Left,
}

/// North north east, .., or north west
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
