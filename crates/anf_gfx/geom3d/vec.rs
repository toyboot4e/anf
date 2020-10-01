use auto_ops::*;

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

    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

/// Constants
impl Vec3f {
    const fn def(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub const fn one() -> Self {
        Self::def(1.0, 1.0, 1.0)
    }

    pub const fn unit_x() -> Self {
        Self::def(1.0, 0.0, 0.0)
    }

    pub const fn unit_y() -> Self {
        Self::def(0.0, 1.0, 0.0)
    }

    pub const fn unit_z() -> Self {
        Self::def(0.0, 0.0, 1.0)
    }

    pub const fn up() -> Self {
        Self::def(0.0, 1.0, 0.0)
    }

    pub const fn down() -> Self {
        Self::def(0.0, -1.0, 0.0)
    }

    pub const fn right() -> Self {
        Self::def(1.0, 0.0, 0.0)
    }

    pub const fn left() -> Self {
        Self::def(-1.0, 0.0, 0.0)
    }

    pub const fn forward() -> Self {
        Self::def(0.0, 0.0, -1.0)
    }

    pub const fn backward() -> Self {
        Self::def(0.0, 0.0, 1.0)
    }
}

impl Vec3f {
    /// Cross product
    pub fn cross(&self, other: &Vec3f) -> Self {
        Self {
            x: self.y * other.z - other.y * self.z,
            y: -(self.x * other.z - other.x * self.z),
            z: self.x * other.y - other.x * self.y,
        }
    }

    /// Dot product
    pub fn dot(&self, other: &Vec3f) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn normalize(&self) -> Self {
        let mut v = self.clone();
        v.normalize_mut();
        v
    }

    pub fn normalize_mut(&mut self) {
        let factor = 1.0 / ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt();

        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
    }
}

impl_op_ex!(-|me: &Vec3f| -> Vec3f { Vec3f::new(-me.x, -me.y, -me.z) });

// Vec3f, f32
impl_op_ex!(*|lhs: &Vec3f, rhs: &f32| -> Vec3f {
    Vec3f::new(lhs.x * rhs, lhs.y * rhs, lhs.z * rhs)
});
impl_op_ex!(/|lhs: &Vec3f, rhs: &f32| -> Vec3f { Vec3f::new( lhs.x / rhs, lhs.y / rhs, lhs.z / rhs) });
impl_op_ex!(*= |lhs: &mut Vec3f, rhs: &f32| { lhs.x *= rhs; lhs.y *= rhs; lhs.z *= rhs; });
impl_op_ex!(/= |lhs: &mut Vec3f, rhs: &f32| { lhs.x /= rhs; lhs.y /= rhs; lhs.z /= rhs; });

// Vec3f, Vec3f
impl_op_ex!(+ |lhs: &Vec3f, rhs: &Vec3f| -> Vec3f { Vec3f::new( lhs.x + rhs.x, lhs.y + rhs.y, lhs.z + rhs.z) });
impl_op_ex!(-|lhs: &Vec3f, rhs: &Vec3f| -> Vec3f {
    Vec3f::new(lhs.x - rhs.x, lhs.y - rhs.y, lhs.z - rhs.z)
});
impl_op_ex!(*|lhs: &Vec3f, rhs: &Vec3f| -> Vec3f {
    Vec3f::new(lhs.x * rhs.x, lhs.y * rhs.y, lhs.z * rhs.z)
});
impl_op_ex!(/ |lhs: &Vec3f, rhs: &Vec3f| -> Vec3f { Vec3f::new( lhs.x / rhs.x, lhs.y / rhs.y, lhs.z / rhs.z) });

impl_op_ex!(+= |lhs: &mut Vec3f, rhs: &Vec3f| { lhs.x += rhs.x; lhs.y += rhs.y; lhs.z += rhs.z; });
impl_op_ex!(-= |lhs: &mut Vec3f, rhs: &Vec3f| { lhs.x -= rhs.x; lhs.y -= rhs.y; lhs.z -= rhs.z; });
impl_op_ex!(*= |lhs: &mut Vec3f, rhs: &Vec3f| { lhs.x *= rhs.x; lhs.y *= rhs.y; lhs.z *= rhs.z; });
impl_op_ex!(/= |lhs: &mut Vec3f, rhs: &Vec3f| { lhs.x /= rhs.x; lhs.y /= rhs.y; lhs.z /= rhs.z; });

impl From<[f32; 3]> for Vec3f {
    fn from(xs: [f32; 3]) -> Self {
        Self::new(xs[0], xs[1], xs[2])
    }
}

impl From<&[f32; 3]> for Vec3f {
    fn from(xs: &[f32; 3]) -> Self {
        Self::new(xs[0], xs[1], xs[2])
    }
}
