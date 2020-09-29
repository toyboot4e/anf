use crate::geom3d::{Mat3f, Vec3f};

#[derive(Debug, Clone, PartialEq)]
pub struct Plane3f {
    pub normal: Vec3f,
    pub d: f32,
}

impl Plane3f {
    pub fn new(v: Vec3f, d: f32) -> Self {
        Self { normal: v, d }
    }

    pub fn from_three_vecs(a: Vec3f, b: Vec3f, c: Vec3f) -> Self {
        let ab = b - a;
        let ac = c - a;
        let cross = ab.cross(&ac);

        let normal = cross.normalize();
        let d = -normal.dot(&a);

        Self { normal, d }
    }

    // pub fn dot(Vector4 value)->Self {
    // 	return
    // 		self.normal.x * value.x +
    // 		self.normal.y * value.y +
    // 		self.normal.z * value.z +
    // 		self.D * value.W
    // 	;
    // }

    pub fn dot_coordinate(&self, v: Vec3f) -> f32 {
        self.normal.x * v.x + self.normal.y * v.y + self.normal.z * v.z + self.d
    }

    pub fn dot_normal(&self, value: Vec3f) -> f32 {
        self.normal.x * value.x + self.normal.y * value.y + self.normal.z * value.z
    }

    pub fn normalize_mut(&mut self) {
        let len = self.normal.len();
        let factor = 1.0 / len;

        self.normal *= factor;
        self.d *= factor;
    }
}
