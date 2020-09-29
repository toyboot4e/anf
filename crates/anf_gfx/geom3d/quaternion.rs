use auto_ops::*;

use crate::geom3d::{Mat3f, Vec3f};

#[derive(Debug, Clone, Default)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    /// Rotation component
    pub w: f32,
}

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    pub const fn identity() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    /// From [x, y, z] and rotation components
    pub fn from_vec(v: Vec3f, scalar: f32) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
            w: scalar,
        }
    }

    pub fn conjugate(&self) -> Self {
        Self::new(-self.x, -self.y, -self.z, self.w)
    }

    /// Returns the magnitude of the quaternion components
    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    /// Returns the squared magnitude of the quaternion components
    pub fn len_squared(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w)
    }

    /// Scales the quaternion magnitude to unit length.
    pub fn normalize(&mut self) {
        let num = 1.0 / self.len();
        self.x *= num;
        self.y *= num;
        self.z *= num;
        self.w *= num;
    }
}

impl_op_ex!(+ |lhs: &Quaternion, rhs: &Quaternion| -> Quaternion{
    Quaternion::new(lhs.x+rhs.x,lhs.y+rhs.y,lhs.z+rhs.z,lhs.w+rhs.w)
});

impl Quaternion {
    /// Creates a new quaternion that contains concatenation between two quaternion.
    pub fn cat(value1: Self, value2: Self) -> Self {
        let x1 = value1.x;
        let y1 = value1.y;
        let z1 = value1.z;
        let w1 = value1.w;

        let x2 = value2.x;
        let y2 = value2.y;
        let z2 = value2.z;
        let w2 = value2.w;

        Self {
            x: ((x2 * w1) + (x1 * w2)) + ((y2 * z1) - (z2 * y1)),
            y: ((y2 * w1) + (y1 * w2)) + ((z2 * x1) - (x2 * z1)),
            z: ((z2 * w1) + (z1 * w2)) + ((x2 * y1) - (y2 * x1)),
            w: (w2 * w1) - (((x2 * x1) + (y2 * y1)) + (z2 * z1)),
        }
    }

    pub fn from_axis_angle(axis: Vec3f, rad: f32) -> Self {
        let half = rad * 0.5;
        let sin = half.sin();
        let cos = half.cos();

        Self {
            x: axis.x * sin,
            y: axis.y * sin,
            z: axis.z * sin,
            w: cos,
        }
    }

    pub fn from_rot_matrix(mat: Mat3f) -> Self {
        let scale = mat.m11 + mat.m22 + mat.m33;

        let mut result = Self::default();

        if scale > 0.0 {
            let mut sqrt = (scale + 1.0).sqrt();
            result.w = sqrt * 0.5;
            sqrt = 0.5 / sqrt;

            result.x = (mat.m23 - mat.m32) * sqrt;
            result.y = (mat.m31 - mat.m13) * sqrt;
            result.z = (mat.m12 - mat.m21) * sqrt;
        } else if (mat.m11 >= mat.m22) && (mat.m11 >= mat.m33) {
            let sqrt = (1.0 + mat.m11 - mat.m22 - mat.m33).sqrt();
            let half = 0.5 / sqrt;

            result.x = 0.5 * sqrt;
            result.y = (mat.m12 + mat.m21) * half;
            result.z = (mat.m13 + mat.m31) * half;
            result.w = (mat.m23 - mat.m32) * half;
        } else if mat.m22 > mat.m33 {
            let sqrt = (1.0 + mat.m22 - mat.m11 - mat.m33).sqrt();
            let half = 0.5 / sqrt;

            result.x = (mat.m21 + mat.m12) * half;
            result.y = 0.5 * sqrt;
            result.z = (mat.m32 + mat.m23) * half;
            result.w = (mat.m31 - mat.m13) * half;
        } else {
            let sqrt = (1.0 + mat.m33 - mat.m11 - mat.m22).sqrt();
            let half = 0.5 / sqrt;

            result.x = (mat.m31 + mat.m13) * half;
            result.y = (mat.m32 + mat.m23) * half;
            result.z = 0.5 * sqrt;
            result.w = (mat.m12 - mat.m21) * half;
        }

        result
    }
}
