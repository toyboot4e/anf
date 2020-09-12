//! Geometry primitives
//!
//! Vectors and rectangles can be converted from/to arrays.

// https://docs.rs/auto_ops/
use auto_ops::*;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Vec2f {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn round(&mut self) {
        self.x = self.x.round();
        self.y = self.y.round();
    }
}

// Vec2f, f32
impl_op_ex!(*|lhs: &Vec2f, rhs: &f32| -> Vec2f {
    Vec2f {
        x: lhs.x * rhs,
        y: lhs.y * rhs,
    }
});

impl_op_ex!(/|lhs: &Vec2f, rhs: &f32| -> Vec2f {
    Vec2f {
        x: lhs.x / rhs,
        y: lhs.y / rhs,
    }
});

impl_op_ex!(*= |lhs: &mut Vec2f, rhs: &f32| {
    lhs.x *= rhs;
    lhs.y *= rhs;
});

impl_op_ex!(/= |lhs: &mut Vec2f, rhs: &f32| {
    lhs.x /= rhs;
    lhs.y /= rhs;
});

// Vec2f, Vec2f
impl_op_ex!(+ |lhs: &Vec2f, rhs: &Vec2f| -> Vec2f {
    Vec2f {
        x: lhs.x + rhs.x,
        y: lhs.y + rhs.y,
    }
});

impl_op_ex!(-|lhs: &Vec2f, rhs: &Vec2f| -> Vec2f {
    Vec2f {
        x: lhs.x - rhs.x,
        y: lhs.y - rhs.y,
    }
});

impl_op_ex!(*|lhs: &Vec2f, rhs: &Vec2f| -> Vec2f {
    Vec2f {
        x: lhs.x * rhs.x,
        y: lhs.y * rhs.y,
    }
});

impl_op_ex!(/ |lhs: &Vec2f, rhs: &Vec2f| -> Vec2f {
    Vec2f {
        x: lhs.x / rhs.x,
        y: lhs.y / rhs.y,
    }
});

impl_op_ex!(+= |lhs: &mut Vec2f, rhs: &Vec2f| {
    lhs.x += rhs.x;
    lhs.y += rhs.y;
});

impl_op_ex!(-= |lhs: &mut Vec2f, rhs: &Vec2f| {
    lhs.x -= rhs.x;
    lhs.y -= rhs.y;
});

impl_op_ex!(*= |lhs: &mut Vec2f, rhs: &Vec2f| {
    lhs.x *= rhs.x;
    lhs.y *= rhs.y;
});

impl_op_ex!(/= |lhs: &mut Vec2f, rhs: &Vec2f| {
    lhs.x /= rhs.x;
    lhs.y /= rhs.y;
});

impl From<[f32; 2]> for Vec2f {
    fn from(xs: [f32; 2]) -> Self {
        Self { x: xs[0], y: xs[1] }
    }
}

impl From<&[f32; 2]> for Vec2f {
    fn from(xs: &[f32; 2]) -> Self {
        Self { x: xs[0], y: xs[1] }
    }
}

impl Into<[f32; 2]> for Vec2f {
    fn into(self) -> [f32; 2] {
        [self.x, self.y]
    }
}

impl Into<[f32; 2]> for &Vec2f {
    fn into(self) -> [f32; 2] {
        [self.x, self.y]
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

// Vec3f, f32
impl_op_ex!(*|lhs: &Vec3f, rhs: &f32| -> Vec3f {
    Vec3f {
        x: lhs.x * rhs,
        y: lhs.y * rhs,
        z: lhs.z * rhs,
    }
});

impl_op_ex!(/|lhs: &Vec3f, rhs: &f32| -> Vec3f {
    Vec3f {
        x: lhs.x / rhs,
        y: lhs.y / rhs,
        z: lhs.z / rhs,
    }
});

impl_op_ex!(*= |lhs: &mut Vec3f, rhs: &f32| {
    lhs.x *= rhs;
    lhs.y *= rhs;
    lhs.z *= rhs;
});

impl_op_ex!(/= |lhs: &mut Vec3f, rhs: &f32| {
    lhs.x /= rhs;
    lhs.y /= rhs;
    lhs.z /= rhs;
});

// Vec3f, Vec3f
impl_op_ex!(+ |lhs: &Vec3f, rhs: &Vec3f| -> Vec3f {
    Vec3f {
        x: lhs.x + rhs.x,
        y: lhs.y + rhs.y,
        z: lhs.z + rhs.z,
    }
});

impl_op_ex!(-|lhs: &Vec3f, rhs: &Vec3f| -> Vec3f {
    Vec3f {
        x: lhs.x - rhs.x,
        y: lhs.y - rhs.y,
        z: lhs.z - rhs.z,
    }
});

impl_op_ex!(*|lhs: &Vec3f, rhs: &Vec3f| -> Vec3f {
    Vec3f {
        x: lhs.x * rhs.x,
        y: lhs.y * rhs.y,
        z: lhs.z * rhs.z,
    }
});

impl_op_ex!(/ |lhs: &Vec3f, rhs: &Vec3f| -> Vec3f {
    Vec3f {
        x: lhs.x / rhs.x,
        y: lhs.y / rhs.y,
        z: lhs.z / rhs.z,
    }
});

impl_op_ex!(+= |lhs: &mut Vec3f, rhs: &Vec3f| {
    lhs.x += rhs.x;
    lhs.y += rhs.y;
    lhs.z += rhs.z;
});

impl_op_ex!(-= |lhs: &mut Vec3f, rhs: &Vec3f| {
    lhs.x -= rhs.x;
    lhs.y -= rhs.y;
    lhs.z -= rhs.z;
});

impl_op_ex!(*= |lhs: &mut Vec3f, rhs: &Vec3f| {
    lhs.x *= rhs.x;
    lhs.y *= rhs.y;
    lhs.z *= rhs.z;
});

impl_op_ex!(/= |lhs: &mut Vec3f, rhs: &Vec3f| {
    lhs.x /= rhs.x;
    lhs.y /= rhs.y;
    lhs.z /= rhs.z;
});

/// Top-left point and size
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Rect2f {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect2f {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn normalized() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 1.0,
            h: 1.0,
        }
    }

    pub fn left_up(&self) -> Vec2f {
        Vec2f {
            x: self.x,
            y: self.y,
        }
    }

    pub fn size(&self) -> Vec2f {
        Vec2f {
            x: self.w,
            y: self.h,
        }
    }
}

impl From<[Vec2f; 2]> for Rect2f {
    fn from(xs: [Vec2f; 2]) -> Self {
        Self {
            x: xs[0].x,
            y: xs[0].y,
            w: xs[1].x,
            h: xs[1].y,
        }
    }
}

impl From<[f32; 4]> for Rect2f {
    fn from(xs: [f32; 4]) -> Self {
        Self {
            x: xs[0],
            y: xs[1],
            w: xs[2],
            h: xs[3],
        }
    }
}

impl Into<[f32; 4]> for Rect2f {
    fn into(self) -> [f32; 4] {
        [self.x, self.y, self.w, self.h]
    }
}

impl Into<[f32; 4]> for &Rect2f {
    fn into(self) -> [f32; 4] {
        [self.x, self.y, self.w, self.h]
    }
}

fna3d::bitflags::bitflags! {
    pub struct Flips: u8 {
        /// Render the sprite as it is
        const NONE = 0;
        /// Render the sprite reversed along the X axis
        const H = 1;
        /// Render the sprite reversed along the Y axis
        const V = 2;
        const HV = 3;
    }
}

/// Top-left and bottom-right
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct Skew2f {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

impl Skew2f {
    pub fn reversed(&self) -> Self {
        Self {
            x1: -self.x1,
            y1: -self.y1,
            x2: -self.x2,
            y2: -self.y2,
        }
    }
}

/// Top-left and bottom-right
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub struct Rot2f {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}
