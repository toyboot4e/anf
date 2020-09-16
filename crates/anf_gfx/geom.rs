//! Geometry primitives
//!
//! They have public fields so they we can easily be modified, but they're not indexed as the
//! drawcback.
//!
//! # Intent
//!
//! Return `SomeGeometryType` to provide geomrty information with cozy interface.
//!
//! Accept `impl Into<SomeGeometryType>` as a variety of input types:
//!
//! ```
//! let a: Rect2f = [0.0, 0.0, 128.0, 72.0];
//! let b: Rect2f = [(0.0, 0.0), (128.0, 72.0)].into();
//! let c: [f32; 4] = a.into();
//!
//! let size: Vec2f = [200.0, 300.0].into();
//! let d: Rect2f = ([0.0, 0.0], size).into();
//! ```

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

    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    pub fn one() -> Self {
        Self { x: 1.0, y: 1.0 }
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

impl From<(f32, f32)> for Vec2f {
    fn from(xs: (f32, f32)) -> Self {
        Self { x: xs.0, y: xs.1 }
    }
}

impl From<&(f32, f32)> for Vec2f {
    fn from(xs: &(f32, f32)) -> Self {
        Self { x: xs.0, y: xs.1 }
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

impl Into<(f32, f32)> for Vec2f {
    fn into(self) -> (f32, f32) {
        (self.x, self.y)
    }
}

impl Into<(f32, f32)> for &Vec2f {
    fn into(self) -> (f32, f32) {
        (self.x, self.y)
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

    pub fn unit() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            w: 1.0,
            h: 1.0,
        }
    }

    pub fn size(&self) -> Vec2f {
        Vec2f {
            x: self.w,
            y: self.h,
        }
    }
}

/// Primitive
impl Rect2f {
    // scalars
    pub fn left(&self) -> f32 {
        self.x
    }

    pub fn right(&self) -> f32 {
        self.x + self.w
    }

    pub fn up(&self) -> f32 {
        self.y
    }

    pub fn down(&self) -> f32 {
        self.y + self.h
    }

    pub fn set_left(&mut self, x: f32) {
        self.x = x;
    }

    pub fn set_right(&mut self, x: f32) {
        self.x = x - self.w;
    }

    pub fn set_up(&mut self, y: f32) {
        self.y = y;
    }

    pub fn set_down(&mut self, y: f32) {
        self.y = y - self.h;
    }

    // vectors
    pub fn left_up(&self) -> Vec2f {
        Vec2f {
            x: self.x,
            y: self.y,
        }
    }

    pub fn right_up(&self) -> Vec2f {
        Vec2f {
            x: self.x + self.w,
            y: self.y,
        }
    }

    pub fn left_down(&self) -> Vec2f {
        Vec2f {
            x: self.x,
            y: self.y + self.h,
        }
    }

    pub fn right_down(&self) -> Vec2f {
        Vec2f {
            x: self.x + self.w,
            y: self.y + self.h,
        }
    }

    pub fn set_left_up(&mut self, pos: impl Into<[f32; 2]>) {
        let pos = pos.into();
        self.x = pos[0];
        self.y = pos[1];
    }

    pub fn set_right_up(&mut self, pos: impl Into<[f32; 2]>) {
        let pos = pos.into();
        self.x = pos[0] - self.w;
        self.y = pos[1];
    }

    pub fn set_left_down(&mut self, pos: impl Into<[f32; 2]>) {
        let pos = pos.into();
        self.x = pos[0];
        self.y = pos[1] - self.h;
    }

    pub fn set_right_down(&mut self, pos: impl Into<[f32; 2]>) {
        let pos = pos.into();
        self.x = pos[0] - self.w;
        self.y = pos[1] - self.h;
    }
}

/// More semantic
impl Rect2f {
    pub fn center(&self) -> Vec2f {
        (self.left_up() + self.right_down()) / 2.0
    }

    /// Origin in pixels from origin in normalized coordinates
    pub fn origin_px(&self, origin: impl Into<Vec2f>) -> Vec2f {
        self.left_up() + self.size() * origin.into()
    }

    /// Sets the position of the center
    pub fn set_center(&mut self, pos: impl Into<[f32; 2]>) {
        let pos = pos.into();
        self.x = pos[0] - self.w / 2.0;
        self.y = pos[1] - self.h / 2.0;
    }

    /// Sets the position of the origin specified with normalized coordinates
    pub fn set_origin(&mut self, pos: impl Into<[f32; 2]>, origin: impl Into<[f32; 2]>) {
        let pos = pos.into();
        let origin = origin.into();
        self.x = pos[0] - self.w * origin[0];
        self.y = pos[1] - self.h * origin[1];
    }

    // mutations
    pub fn translate(&mut self, v: impl Into<Vec2f>) {
        let v = v.into();
        self.x += v.x;
        self.y += v.y;
    }

    pub fn clamp_x(&mut self, min: f32, max: f32) {
        if self.left() < min {
            self.set_left(min);
        }
        if self.right() > max {
            self.set_right(max)
        }
    }

    pub fn clamp_y(&mut self, min: f32, max: f32) {
        if self.up() < min {
            self.set_up(min);
        }
        if self.down() > max {
            self.set_down(max)
        }
    }
}

/// ([x, y], [w, h]) -> Rect2f
impl<T, U> From<(T, U)> for Rect2f
where
    T: Into<[f32; 2]>,
    U: Into<[f32; 2]>,
{
    fn from(xs: (T, U)) -> Self {
        let (xy, wh) = xs;
        let xy = xy.into();
        let wh = wh.into();
        Self {
            x: xy[0],
            y: xy[1],
            w: wh[0],
            h: wh[1],
        }
    }
}

// [[x, y], [w, h]] -> Rect2f
//
// confclits with the preceding impl
// impl<T: Into<[f32; 2]> + Copy> From<[T; 2]> for Rect2f {
//     fn from(xs: [T; 2]) -> Self {
//         let xy = xs[0].clone().into();
//         let wh = xs[1].clone().into();
//         Self {
//             x: xy[0],
//             y: xy[1],
//             w: wh[0],
//             h: wh[1],
//         }
//     }
// }

/// [(x, y), (w, h)] -> Rect2f
impl<T> From<[T; 2]> for Rect2f
where
    T: Into<(f32, f32)> + Copy,
{
    fn from(xs: [T; 2]) -> Self {
        let xy = xs[0].clone().into();
        let wh = xs[1].clone().into();
        Self {
            x: xy.0,
            y: xy.1,
            w: wh.0,
            h: wh.1,
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

/// Skew matrix
///
/// Top-left and bottom-right.
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

/// Rotation matrix expanded from a radian value
///
/// Use radian to store rotation. Top-left and bottom-right.
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub(crate) struct Rot2f {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

impl Rot2f {
    pub fn from_rad(rad: f32) -> Self {
        // TODO: what is this..
        if rad >= f32::EPSILON {
            let sin = rad.sin();
            let cos = rad.cos();
            Self {
                x1: cos,
                y1: sin,
                x2: -sin,
                y2: cos,
            }
        } else {
            Self {
                x1: 1.0,
                y1: 0.0,
                x2: 0.0,
                y2: 1.0,
            }
        }
    }
}
