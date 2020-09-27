//! Desrete geometry types

// https://docs.rs/auto_ops/
use auto_ops::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Dir4 {
    N,
    E,
    S,
    W,
}

impl Dir4 {
    pub fn x_sign(&self) -> i32 {
        match self {
            Dir4::N | Dir4::S => 0,
            Dir4::W => -1,
            Dir4::E => 1,
        }
    }

    pub fn y_sign(&self) -> i32 {
        match self {
            Dir4::W | Dir4::E => 0,
            Dir4::N => -1,
            Dir4::S => 1,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

impl Dir8 {
    pub fn x_sign(&self) -> i32 {
        match self {
            Dir8::N | Dir8::S => 0,
            Dir8::W | Dir8::NW | Dir8::SW => -1,
            Dir8::E | Dir8::NE | Dir8::SE => 1,
        }
    }

    pub fn y_sign(&self) -> i32 {
        match self {
            Dir8::W | Dir8::E => 0,
            Dir8::N | Dir8::NW | Dir8::NE => -1,
            Dir8::S | Dir8::SW | Dir8::SE => 1,
        }
    }
}

/// Screen bounds in pixels
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Rect2i {
    pos: Vec2i,
    size: Vec2u,
}

impl Rect2i {
    pub fn new(xs: impl Into<[i32; 2]>, ws: impl Into<[u32; 2]>) -> Self {
        let xs = xs.into();
        let ws = ws.into();
        Self {
            pos: Vec2i::new(xs[0], xs[1]),
            size: Vec2u::new(ws[0], ws[1]),
        }
    }

    pub fn size(&self) -> Vec2u {
        self.size
    }

    pub fn left_up(&self) -> Vec2i {
        self.pos
    }

    pub fn right_up(&self) -> Vec2i {
        Vec2i::new(self.pos.x + self.size.x as i32, self.pos.y)
    }

    pub fn left_down(&self) -> Vec2i {
        Vec2i::new(self.pos.x, self.pos.y + self.size.y as i32)
    }

    pub fn right_down(&self) -> Vec2i {
        Vec2i::new(
            self.pos.x + self.size.x as i32,
            self.pos.y + self.size.y as i32,
        )
    }
}

/// Size/point in pixels
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl_op_ex!(-|me: &Vec2i| -> Vec2i { Vec2i::new(-me.x, -me.y) });

impl_op_ex!(+ |lhs: &Vec2i, rhs: &Vec2i| -> Vec2i { Vec2i::new(lhs.x + rhs.x, lhs.y + rhs.y) });
impl_op_ex!(-|lhs: &Vec2i, rhs: &Vec2i| -> Vec2i { Vec2i::new(lhs.x - rhs.x, lhs.y - rhs.y) });
impl_op_ex!(*|lhs: &Vec2i, rhs: &Vec2i| -> Vec2i { Vec2i::new(lhs.x * rhs.x, lhs.y * rhs.y) });
impl_op_ex!(/ |lhs: &Vec2i, rhs: &Vec2i| -> Vec2i { Vec2i::new(lhs.x / rhs.x, lhs.y / rhs.y) });

impl_op_ex!(+= |lhs: &mut Vec2i, rhs: &Vec2i| { lhs.x += rhs.x; lhs.y += rhs.y; });
impl_op_ex!(-= |lhs: &mut Vec2i, rhs: &Vec2i| { lhs.x -= rhs.x; lhs.y -= rhs.y; });
impl_op_ex!(*= |lhs: &mut Vec2i, rhs: &Vec2i| { lhs.x *= rhs.x; lhs.y *= rhs.y; });
impl_op_ex!(/= |lhs: &mut Vec2i, rhs: &Vec2i| { lhs.x /= rhs.x; lhs.y /= rhs.y; });

impl_op_ex!(*|lhs: &Vec2i, rhs: &i32| -> Vec2i { Vec2i::new(lhs.x * rhs, lhs.y * rhs) });
impl_op_ex!(/|lhs: &Vec2i, rhs: &i32| -> Vec2i { Vec2i::new(lhs.x / rhs, lhs.y / rhs) });
impl_op_ex!(*= |lhs: &mut Vec2i, rhs: &i32| { lhs.x *= rhs; lhs.y *= rhs; });
impl_op_ex!(/= |lhs: &mut Vec2i, rhs: &i32| { lhs.x /= rhs; lhs.y /= rhs; });

impl Vec2i {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Point in pixels
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vec2u {
    pub x: u32,
    pub y: u32,
}

impl Vec2u {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

impl_op_ex!(+ |lhs: &Vec2u, rhs: &Vec2u| -> Vec2u { Vec2u::new(lhs.x + rhs.x, lhs.y + rhs.y) });
impl_op_ex!(-|lhs: &Vec2u, rhs: &Vec2u| -> Vec2u { Vec2u::new(lhs.x - rhs.x, lhs.y - rhs.y) });
impl_op_ex!(*|lhs: &Vec2u, rhs: &Vec2u| -> Vec2u { Vec2u::new(lhs.x * rhs.x, lhs.y * rhs.y) });
impl_op_ex!(/ |lhs: &Vec2u, rhs: &Vec2u| -> Vec2u { Vec2u::new(lhs.x / rhs.x, lhs.y / rhs.y) });

impl_op_ex!(+= |lhs: &mut Vec2u, rhs: &Vec2u| { lhs.x += rhs.x; lhs.y += rhs.y; });
impl_op_ex!(-= |lhs: &mut Vec2u, rhs: &Vec2u| { lhs.x -= rhs.x; lhs.y -= rhs.y; });
impl_op_ex!(*= |lhs: &mut Vec2u, rhs: &Vec2u| { lhs.x *= rhs.x; lhs.y *= rhs.y; });
impl_op_ex!(/= |lhs: &mut Vec2u, rhs: &Vec2u| { lhs.x /= rhs.x; lhs.y /= rhs.y; });

impl_op_ex!(*|lhs: &Vec2u, rhs: &u32| -> Vec2u { Vec2u::new(lhs.x * rhs, lhs.y * rhs) });
impl_op_ex!(/|lhs: &Vec2u, rhs: &u32| -> Vec2u { Vec2u::new(lhs.x / rhs, lhs.y / rhs) });
impl_op_ex!(*= |lhs: &mut Vec2u, rhs: &u32| { lhs.x *= rhs; lhs.y *= rhs; });
impl_op_ex!(/= |lhs: &mut Vec2u, rhs: &u32| { lhs.x /= rhs; lhs.y /= rhs; });
