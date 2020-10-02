use auto_ops::*;

use crate::geom3d::{Plane3f, Quaternion, Vec3f};

/// 4x4 matrix for 3D world
///
/// ANF thinks **position vectors are row vectors**. So most matrices in mathmatical textbooks are
/// transposed.
///
/// # System
///
/// * Right-handed coordinate system
/// * Row-major matrix notation (position vectors are row vectors)
/// * Row-major memory layout (m_11, m_12, m_13, ..)
///
/// # Warning
///
/// This is very likely buggy. TODO: debug
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Mat3f {
    /// First row, first column
    pub m11: f32,
    /// Second row, second column
    pub m12: f32,
    pub m13: f32,
    pub m14: f32,
    //
    pub m21: f32,
    pub m22: f32,
    pub m23: f32,
    pub m24: f32,
    //
    pub m31: f32,
    pub m32: f32,
    pub m33: f32,
    pub m34: f32,
    //
    pub m41: f32,
    pub m42: f32,
    pub m43: f32,
    pub m44: f32,
}

impl Mat3f {
    pub fn new(
        m11: f32,
        m12: f32,
        m13: f32,
        m14: f32,
        m21: f32,
        m22: f32,
        m23: f32,
        m24: f32,
        m31: f32,
        m32: f32,
        m33: f32,
        m34: f32,
        m41: f32,
        m42: f32,
        m43: f32,
        m44: f32,
    ) -> Self {
        Self {
            m11,
            m12,
            m13,
            m14,
            m21,
            m22,
            m23,
            m24,
            m31,
            m32,
            m33,
            m34,
            m41,
            m42,
            m43,
            m44,
        }
    }

    /// The backward vector formed from the third row M31, M32, M33 elements
    pub fn backward(&self) -> Vec3f {
        Vec3f::new(self.m31, self.m32, self.m33)
    }

    /// The backward vector formed from the third row M31, M32, M33 elements
    pub fn set_backward(&mut self, v: &Vec3f) {
        self.m31 = v.x;
        self.m32 = v.y;
        self.m33 = v.z;
    }

    /// The down vector formed from the second row -M21, -M22, -M23 elements.
    pub fn down(&self) -> Vec3f {
        Vec3f::new(-self.m21, -self.m22, -self.m23)
    }

    /// The down vector formed from the second row -M21, -M22, -M23 elements.
    pub fn set_down(&mut self, v: &Vec3f) {
        self.m21 = -v.x;
        self.m22 = -v.y;
        self.m23 = -v.z;
    }

    /// The forward vector formed from the third row -M31, -M32, -M33 elements.
    pub fn forward(&self) -> Vec3f {
        Vec3f::new(-self.m31, -self.m32, -self.m33)
    }

    /// The forward vector formed from the third row -M31, -M32, -M33 elements.
    pub fn set_forward(&mut self, v: &Vec3f) {
        self.m31 = -v.x;
        self.m32 = -v.y;
        self.m33 = -v.z;
    }

    pub const fn identity() -> Self {
        Self {
            m11: 1.0,
            m22: 1.0,
            m33: 1.0,
            m44: 1.0,
            //
            m12: 0.0,
            m13: 0.0,
            m14: 0.0,
            m21: 0.0,
            m23: 0.0,
            m24: 0.0,
            m31: 0.0,
            m32: 0.0,
            m34: 0.0,
            m41: 0.0,
            m42: 0.0,
            m43: 0.0,
        }
    }

    /// The left vector formed from the first row -M11, -M12, -M13 elements.
    pub fn left(&self) -> Vec3f {
        Vec3f::new(-self.m11, -self.m12, -self.m13)
    }

    /// The left vector formed from the first row -M11, -M12, -M13 elements.
    pub fn set_left(&mut self, v: &Vec3f) {
        self.m11 = -v.x;
        self.m12 = -v.y;
        self.m13 = -v.z;
    }

    /// The right vector formed from the first row M11, M12, M13 elements.
    pub fn right(&self) -> Vec3f {
        Vec3f::new(self.m11, self.m12, self.m13)
    }

    /// The right vector formed from the first row M11, M12, M13 elements.
    pub fn set_right(&mut self, v: &Vec3f) {
        self.m11 = v.x;
        self.m12 = v.y;
        self.m13 = v.z;
    }

    /// Position stored in this matrix.
    pub fn translation(&self) -> Vec3f {
        Vec3f::new(self.m41, self.m42, self.m43)
    }

    pub fn set_translation(&mut self, v: &Vec3f) {
        self.m41 = v.x;
        self.m42 = v.y;
        self.m43 = v.z;
    }

    /// The upper vector formed from the second row M21, M22, M23 elements.
    pub fn up(&self) -> Vec3f {
        Vec3f::new(self.m21, self.m22, self.m23)
    }

    pub fn set_up(&mut self, v: &Vec3f) {
        self.m21 = v.x;
        self.m22 = v.y;
        self.m23 = v.z;
    }

    /// Tries to decomposes this matrix to translation, rotation and scale elements
    pub fn to_components(&self) -> Option<(Vec3f, Quaternion, Vec3f)> {
        let tr = Vec3f::new(self.m41, self.m42, self.m43);

        let xs = (self.m11 * self.m12 * self.m13 * self.m14).signum();
        let ys = (self.m21 * self.m22 * self.m23 * self.m24).signum();
        let zs = (self.m31 * self.m32 * self.m33 * self.m34).signum();

        let scale = Vec3f {
            x: xs * (self.m11 * self.m11 + self.m12 * self.m12 + self.m13 * self.m13).sqrt(),
            y: ys * (self.m21 * self.m21 + self.m22 * self.m22 + self.m23 * self.m23).sqrt(),
            z: zs * (self.m31 * self.m31 + self.m32 * self.m32 + self.m33 * self.m33).sqrt(),
        };

        if scale.x < f32::EPSILON || scale.y < f32::EPSILON || scale.z < f32::EPSILON {
            // rotation = Quaternion.Identity;
            return None;
        }

        let m1 = Self::new(
            self.m11 / scale.x,
            self.m12 / scale.x,
            self.m13 / scale.x,
            0.0,
            self.m21 / scale.y,
            self.m22 / scale.y,
            self.m23 / scale.y,
            0.0,
            self.m31 / scale.z,
            self.m32 / scale.z,
            self.m33 / scale.z,
            0.0,
            0.0,
            0.0,
            0.0,
            1.0,
        );

        let rot = Quaternion::from_rot_matrix(m1);

        Some((tr, rot, scale))
    }

    /// Determinant
    pub fn det(&self) -> f32 {
        let num18 = (self.m33 * self.m44) - (self.m34 * self.m43);
        let num17 = (self.m32 * self.m44) - (self.m34 * self.m42);
        let num16 = (self.m32 * self.m43) - (self.m33 * self.m42);
        let num15 = (self.m31 * self.m44) - (self.m34 * self.m41);
        let num14 = (self.m31 * self.m43) - (self.m33 * self.m41);
        let num13 = (self.m31 * self.m42) - (self.m32 * self.m41);

        (((self.m11 * (((self.m22 * num18) - (self.m23 * num17)) + (self.m24 * num16)))
            - (self.m12 * (((self.m21 * num18) - (self.m23 * num15)) + (self.m24 * num14))))
            + (self.m13 * (((self.m21 * num17) - (self.m22 * num15)) + (self.m24 * num13))))
            - (self.m14 * (((self.m21 * num16) - (self.m22 * num14)) + (self.m23 * num13)))
    }
}
/// Constructors
impl Mat3f {
    /// Creates a new matrix for spherical billboarding that rotates around specified object
    /// position.
    pub fn create_billboard(
        obj_pos: Vec3f,
        cam_pos: Vec3f,
        camera_up_vector: Vec3f,
        camera_forward_vector: Option<Vec3f>,
    ) -> Self {
        // resulting matrix
        let mut res = Mat3f::default();

        let mut vector = obj_pos - cam_pos;

        let num = vector.len_squared();
        vector = if num < 0.0001 {
            if let Some(v) = camera_forward_vector {
                -v
            } else {
                Vec3f::forward()
            }
        } else {
            vector * (1.0 / num.sqrt())
        };

        let mut vector3 = camera_up_vector;
        vector3.cross(&vector);
        vector3.normalize_mut();

        let mut vector2 = vector;
        vector2 = vector2.cross(&vector3);

        res.m11 = vector3.x;
        res.m12 = vector3.y;
        res.m13 = vector3.z;
        res.m14 = 0.0;
        res.m21 = vector2.x;
        res.m22 = vector2.y;
        res.m23 = vector2.z;
        res.m24 = 0.0;
        res.m31 = vector.x;
        res.m32 = vector.y;
        res.m33 = vector.z;
        res.m34 = 0.0;
        res.m41 = obj_pos.x;
        res.m42 = obj_pos.y;
        res.m43 = obj_pos.z;
        res.m44 = 1.0;

        res
    }

    /// Creates a new matrix for cylindrical billboarding that rotates around specified axis
    pub fn create_constrained_billboard(
        obj_pos: Vec3f,
        cam_pos: Vec3f,
        rot_axis: Vec3f,
        camera_forward_vector: Option<Vec3f>,
        object_forward_vector: Option<Vec3f>,
    ) -> Self {
        let mut vector2 = obj_pos - cam_pos;
        let num2 = vector2.len_squared();
        vector2 = if num2 < 0.0001 {
            if let Some(v) = camera_forward_vector {
                -v
            } else {
                Vec3f::forward()
            }
        } else {
            vector2 * (1.0 / (num2.sqrt()))
        };

        let vector4 = rot_axis;
        let num = rot_axis.dot(&vector2);

        let (vector, vector3) = if num.abs() <= 0.9982547 {
            let mut vector3 = rot_axis.cross(&vector2);
            vector3.normalize_mut();
            let mut vector = vector3.cross(&vector4);
            vector.normalize_mut();
            (vector, vector3)
        } else {
            let vector = if let Some(f) = object_forward_vector {
                let mut num = rot_axis.dot(&f);
                if num.abs() <= 0.9982547 {
                    f
                } else {
                    num = rot_axis.x * Vec3f::forward().x
                        + rot_axis.y * Vec3f::forward().y
                        + rot_axis.z * Vec3f::forward().z;

                    if num.abs() > 0.9982 {
                        Vec3f::right()
                    } else {
                        Vec3f::forward()
                    }
                }
            } else {
                let mut num = rot_axis.x * Vec3f::forward().x
                    + rot_axis.y * Vec3f::forward().y
                    + rot_axis.z * Vec3f::forward().z;

                if num.abs() > 0.9982547 {
                    Vec3f::right()
                } else {
                    Vec3f::forward()
                }
            };

            let mut vector3 = rot_axis.cross(&vector);
            vector3.normalize_mut();
            let mut vector = vector3.cross(&rot_axis);
            vector.normalize_mut();

            (vector, vector3)
        };

        Self {
            m11: vector3.x,
            m12: vector3.y,
            m13: vector3.z,
            m14: 0.0,
            m21: vector4.x,
            m22: vector4.y,
            m23: vector4.z,
            m24: 0.0,
            m31: vector.x,
            m32: vector.y,
            m33: vector.z,
            m34: 0.0,
            m41: obj_pos.x,
            m42: obj_pos.y,
            m43: obj_pos.z,
            m44: 1.0,
        }
    }

    /// Creates a new matrix which contains the rotation moment around specified axis
    pub fn from_axis_angle(axis: &Vec3f, rad: f32) -> Self {
        let Vec3f { x, y, z } = axis;
        let num2 = rad.sin();
        let num = rad.cos();
        let num11 = x * x;
        let num10 = y * y;
        let num9 = z * z;
        let num8 = x * y;
        let num7 = x * z;
        let num6 = y * z;

        Self {
            m11: num11 + (num * (1.0 - num11)),
            m12: (num8 - (num * num8)) + (num2 * z),
            m13: (num7 - (num * num7)) - (num2 * y),
            m14: 0.0,
            m21: (num8 - (num * num8)) - (num2 * z),
            m22: num10 + (num * (1.0 - num10)),
            m23: (num6 - (num * num6)) + (num2 * x),
            m24: 0.0,
            m31: (num7 - (num * num7)) + (num2 * y),
            m32: (num6 - (num * num6)) - (num2 * x),
            m33: num9 + (num * (1.0 - num9)),
            m34: 0.0,
            m41: 0.0,
            m42: 0.0,
            m43: 0.0,
            m44: 1.0,
        }
    }

    /// Creates a new rotation matrix from a [Quaternion]
    pub fn from_quaternion(q: &Quaternion) -> Self {
        let num9 = q.x * q.x;
        let num8 = q.y * q.y;
        let num7 = q.z * q.z;
        let num6 = q.x * q.y;
        let num5 = q.z * q.w;
        let num4 = q.z * q.x;
        let num3 = q.y * q.w;
        let num2 = q.y * q.z;
        let num = q.x * q.w;

        Self {
            m11: 1.0 - (2.0 * (num8 + num7)),
            m12: 2.0 * (num6 + num5),
            m13: 2.0 * (num4 - num3),
            m14: 0.0,
            m21: 2.0 * (num6 - num5),
            m22: 1.0 - (2.0 * (num7 + num9)),
            m23: 2.0 * (num2 + num),
            m24: 0.0,
            m31: 2.0 * (num4 + num3),
            m32: 2.0 * (num2 - num),
            m33: 1.0 - (2.0 * (num8 + num9)),
            m34: 0.0,
            m41: 0.0,
            m42: 0.0,
            m43: 0.0,
            m44: 1.0,
        }
    }

    // /// Creates a new rotation matrix from the specified yaw, pitch and roll values
    // pub fn from_yaw_pitch_roll(yaw: f32, pitch: f32, roll: f32) -> Self {
    //     let q = Quaternion::from_yaw_pitch_roll(yaw, pitch, roll);
    //     Self::from_quaternion(&q)
    // }

    /// Creates a new viewing matrix
    pub fn lookat(cam_pos: Vec3f, cam_target: Vec3f, cam_up_vec: Vec3f) -> Self {
        let mut a = cam_pos - cam_target;
        a.normalize_mut();
        let mut b = cam_up_vec.cross(&a);
        b.normalize_mut();
        let c = a.cross(&b);

        Self {
            m11: b.x,
            m12: c.x,
            m13: a.x,
            m14: 0.0,
            m21: b.y,
            m22: c.y,
            m23: a.y,
            m24: 0.0,
            m31: b.z,
            m32: c.z,
            m33: a.z,
            m34: 0.0,
            m41: -b.dot(&cam_pos),
            m42: -c.dot(&cam_pos),
            m43: -a.dot(&cam_pos),
            m44: 1.0,
        }
    }

    /// Creates a new projection matrix for orthographic view
    pub fn orthographic(
        w: f32,
        h: f32,
        // z values
        near: f32,
        far: f32,
    ) -> Self {
        Self::orthographic_off_center(0.0, w, h, 0.0, near, far)
    }

    /// Creates a new  matrix for a viewport
    ///
    /// # Row-major and transpose
    ///
    /// The [wiki]'s orthographic matrix is **transposed** in ANF world. This is because NAF is
    /// row-major where position vectors are row vectors.
    ///
    /// # Parameters
    ///
    /// See the implementation of [`Mat3f::orthographic`] to get the picture.
    ///
    /// * `top` < `bottom`
    ///
    /// (left, bottom) points to left-down corner of the screen while (right, top) points to the
    /// right-up corner of the screen. Our y axis goes from up to down so `top` < `bottom`.
    ///
    /// * `near` > `far`
    ///
    /// We're using right-handed coordinate system. In 2D, x axis is goes right, y axis goes down,
    /// and z axis goes from your monitor to you. So `near` > `far`.
    ///
    /// [wiki]: https://en.wikipedia.org/wiki/Orthographic_projection
    pub fn orthographic_off_center(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Self {
        Self {
            m11: (2.0 / (right as f64 - left as f64)) as f32,
            m12: 0.0,
            m13: 0.0,
            m14: 0.0,
            m21: 0.0,
            m22: (2.0 / (top as f64 - bottom as f64)) as f32,
            m23: 0.0,
            m24: 0.0,
            m31: 0.0,
            m32: 0.0,
            m33: (1.0 / (near as f64 - far as f64)) as f32,
            m34: 0.0,
            m41: ((left as f64 + right as f64) / (left as f64 - right as f64)) as f32,
            m42: ((top as f64 + bottom as f64) / (bottom as f64 - top as f64)) as f32,
            m43: (near as f64 / (near as f64 - far as f64)) as f32,
            m44: 1.0,
        }
    }

    /// Creates a new projection matrix for perspective view.
    pub fn perspective(w: f32, h: f32, near_plane_distance: f32, far_plane_distance: f32) -> Self {
        assert!(near_plane_distance > 0.0, "near_plane_distance <= 0");
        assert!(far_plane_distance > 0.0, "far_plane_distance <= 0");
        assert!(
            near_plane_distance < far_plane_distance,
            "near_plane_distance >= far_plane_distance"
        );

        Self {
            m11: (2.0 * near_plane_distance) / w,
            m22: (2.0 * near_plane_distance) / h,
            m33: far_plane_distance / (near_plane_distance - far_plane_distance),
            m34: -1.0,
            m43: ((near_plane_distance * far_plane_distance)
                / (near_plane_distance - far_plane_distance)),
            ..Default::default()
        }
    }

    /// Creates a new projection matrix for perspective view with field of view.
    pub fn perspective_fov(
        fov: f32,
        aspect_ratio: f32,
        near_plane_distance: f32,
        far_plane_distance: f32,
    ) -> Self {
        assert!(fov > 0.0 && fov < 3.141593, "fieldOfView <= 0 or >= PI");
        assert!(near_plane_distance > 0.0, "nearPlaneDistance <= 0");
        assert!(far_plane_distance > 0.0, "farPlaneDistance <= 0");
        assert!(
            near_plane_distance < far_plane_distance,
            "nearPlaneDistance >= farPlaneDistance"
        );

        let num = 1.0 / (fov * 0.5).tan();
        let num9 = num / aspect_ratio;
        Self {
            m11: num9,
            m22: num,
            m33: far_plane_distance / (near_plane_distance - far_plane_distance),
            m34: -1.0,
            m43: ((near_plane_distance * far_plane_distance)
                / (near_plane_distance - far_plane_distance)),
            ..Default::default()
        }
    }

    /// Creates a new projection matrix for customized perspective view.
    pub fn perspective_off_center(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near_plane_distance: f32,
        far_plane_distance: f32,
    ) -> Self {
        assert!(near_plane_distance > 0.0, "nearPlaneDistance <= 0");
        assert!(far_plane_distance > 0.0, "farPlaneDistance <= 0");
        assert!(
            near_plane_distance < far_plane_distance,
            "nearPlaneDistance >= farPlaneDistance"
        );

        Self {
            m11: (2.0 * near_plane_distance) / (right - left),
            m22: (2.0 * near_plane_distance) / (top - bottom),
            m31: (left + right) / (right - left),
            m32: (top + bottom) / (top - bottom),
            m33: far_plane_distance / (near_plane_distance - far_plane_distance),
            m34: -1.0,
            m43: ((near_plane_distance * far_plane_distance)
                / (near_plane_distance - far_plane_distance)),
            ..Default::default()
        }
    }

    /// Rotation around x axis
    pub fn from_rot_x(rad: f32) -> Self {
        let mut res = Self::identity();
        let val1 = rad.cos();
        let val2 = rad.sin();
        res.m22 = val1;
        res.m23 = val2;
        res.m32 = -val2;
        res.m33 = val1;
        res
    }

    /// Rotation around y axis
    pub fn from_rot_y(rad: f32) -> Self {
        let mut res = Self::identity();
        let val1 = rad.cos();
        let val2 = rad.sin();
        res.m11 = val1;
        res.m13 = -val2;
        res.m31 = val2;
        res.m33 = val1;
        res
    }

    /// Rotation around z axis
    pub fn from_rot_z(rad: f32) -> Self {
        let mut res = Self::identity();
        let val1 = rad.cos();
        let val2 = rad.sin();
        res.m11 = val1;
        res.m12 = val2;
        res.m21 = -val2;
        res.m22 = val1;
        res
    }

    pub fn from_scale(x: f32, y: f32, z: f32) -> Self {
        Self {
            m11: x,
            m22: y,
            m33: z,
            m44: 1.0,
            ..Default::default()
        }
    }

    /// Creates a new palne as if casting a shadow from a specified light source.
    pub fn from_shadow(light_direction: Vec3f, plane: Plane3f) -> Self {
        let dot = plane.normal.x * light_direction.x
            + plane.normal.y * light_direction.y
            + plane.normal.z * light_direction.z;
        let x = -plane.normal.x;
        let y = -plane.normal.y;
        let z = -plane.normal.z;
        let d = -plane.d;

        Self {
            m11: (x * light_direction.x) + dot,
            m12: x * light_direction.y,
            m13: x * light_direction.z,
            m14: 0.0,
            m21: y * light_direction.x,
            m22: (y * light_direction.y) + dot,
            m23: y * light_direction.z,
            m24: 0.0,
            m31: z * light_direction.x,
            m32: z * light_direction.y,
            m33: (z * light_direction.z) + dot,
            m34: 0.0,
            m41: d * light_direction.x,
            m42: d * light_direction.y,
            m43: d * light_direction.z,
            m44: dot,
        }
    }

    pub fn from_translation(v: Vec3f) -> Self {
        Self {
            m11: 1.0,
            m22: 1.0,
            m33: 1.0,
            m44: 1.0,
            m41: v.x,
            m42: v.y,
            m43: v.z,
            ..Default::default()
        }
    }

    pub fn from_reflection(value: Plane3f) -> Self {
        let mut plane = value;
        plane.normalize_mut();

        let (x, y, z) = (plane.normal.x, plane.normal.y, plane.normal.z);
        let num3 = -2.0 * x;
        let num2 = -2.0 * y;
        let num = -2.0 * z;

        Self {
            m11: (num3 * x) + 1.0,
            m12: num2 * x,
            m13: num * x,
            m14: 0.0,
            m21: num3 * y,
            m22: (num2 * y) + 1.0,
            m23: num * y,
            m24: 0.0,
            m31: num3 * z,
            m32: num2 * z,
            m33: (num * z) + 1.0,
            m34: 0.0,
            m41: num3 * plane.d,
            m42: num2 * plane.d,
            m43: num * plane.d,
            m44: 1.0,
        }
    }

    pub fn from_world(pos: Vec3f, forward: Vec3f, up: Vec3f) -> Self {
        let mut z = forward.normalize();

        let mut x = forward.cross(&up);
        x.normalize_mut();

        let mut y = x.cross(&forward);
        y.normalize_mut();

        let mut res = Self::default();
        res.set_right(&x);
        res.set_up(&y);
        res.set_forward(&z);
        res.set_translation(&pos);
        res.m44 = 1.0;
        res
    }
}

impl Mat3f {
    /// Creates a new matrix which contains inversion of the specified matrix
    pub fn inv(&self) -> Self {
        /*
         * Use Laplace expansion theorem to calculate the inverse of a 4x4 matrix.
         *
         * 1. Calculate the 2x2 determinants needed the 4x4 determinant based on
         *    the 2x2 determinants.
         * 3. Create the adjugate matrix, which satisfies: A * adj(A) = det(A) * I.
         * 4. Divide adjugate matrix with the determinant to find the inverse.
         */
        let num1 = self.m11;
        let num2 = self.m12;
        let num3 = self.m13;
        let num4 = self.m14;
        let num5 = self.m21;
        let num6 = self.m22;
        let num7 = self.m23;
        let num8 = self.m24;
        let num9 = self.m31;
        let num10 = self.m32;
        let num11 = self.m33;
        let num12 = self.m34;
        let num13 = self.m41;
        let num14 = self.m42;
        let num15 = self.m43;
        let num16 = self.m44;
        let num17 = (num11 as f64 * num16 as f64 - num12 as f64 * num15 as f64) as f32;
        let num18 = (num10 as f64 * num16 as f64 - num12 as f64 * num14 as f64) as f32;
        let num19 = (num10 as f64 * num15 as f64 - num11 as f64 * num14 as f64) as f32;
        let num20 = (num9 as f64 * num16 as f64 - num12 as f64 * num13 as f64) as f32;
        let num21 = (num9 as f64 * num15 as f64 - num11 as f64 * num13 as f64) as f32;
        let num22 = (num9 as f64 * num14 as f64 - num10 as f64 * num13 as f64) as f32;
        let num23 = (num6 as f64 * num17 as f64 - num7 as f64 * num18 as f64
            + num8 as f64 * num19 as f64) as f32;
        let num24 = -(num5 as f64 * num17 as f64 - num7 as f64 * num20 as f64
            + num8 as f64 * num21 as f64) as f32;
        let num25 = (num5 as f64 * num18 as f64 - num6 as f64 * num20 as f64
            + num8 as f64 * num22 as f64) as f32;
        let num26 = -(num5 as f64 * num19 as f64 - num6 as f64 * num21 as f64
            + num7 as f64 * num22 as f64) as f32;
        let num27 = (1.0
            / (num1 as f64 * num23 as f64
                + num2 as f64 * num24 as f64
                + num3 as f64 * num25 as f64
                + num4 as f64 * num26 as f64)) as f32;

        let mut result = Self::default();

        result.m21 = num24 * num27;
        result.m31 = num25 * num27;
        result.m41 = num26 * num27;
        result.m12 = (-(num2 as f64 * num17 as f64 - num3 as f64 * num18 as f64
            + num4 as f64 * num19 as f64)
            * num27 as f64) as f32;
        result.m22 = ((num1 as f64 * num17 as f64 - num3 as f64 * num20 as f64
            + num4 as f64 * num21 as f64)
            * num27 as f64) as f32;
        result.m32 = (-(num1 as f64 * num18 as f64 - num2 as f64 * num20 as f64
            + num4 as f64 * num22 as f64)
            * num27 as f64) as f32;
        result.m42 = ((num1 as f64 * num19 as f64 - num2 as f64 * num21 as f64
            + num3 as f64 * num22 as f64)
            * num27 as f64) as f32;

        let num28 = (num7 as f64 * num16 as f64 - num8 as f64 * num15 as f64) as f32;
        let num29 = (num6 as f64 * num16 as f64 - num8 as f64 * num14 as f64) as f32;
        let num30 = (num6 as f64 * num15 as f64 - num7 as f64 * num14 as f64) as f32;
        let num31 = (num5 as f64 * num16 as f64 - num8 as f64 * num13 as f64) as f32;
        let num32 = (num5 as f64 * num15 as f64 - num7 as f64 * num13 as f64) as f32;
        let num33 = (num5 as f64 * num14 as f64 - num6 as f64 * num13 as f64) as f32;
        result.m13 = ((num2 as f64 * num28 as f64 - num3 as f64 * num29 as f64
            + num4 as f64 * num30 as f64)
            * num27 as f64) as f32;
        result.m23 = (-(num1 as f64 * num28 as f64 - num3 as f64 * num31 as f64
            + num4 as f64 * num32 as f64)
            * num27 as f64) as f32;
        result.m33 = ((num1 as f64 * num29 as f64 - num2 as f64 * num31 as f64
            + num4 as f64 * num33 as f64)
            * num27 as f64) as f32;
        result.m43 = (-(num1 as f64 * num30 as f64 - num2 as f64 * num32 as f64
            + num3 as f64 * num33 as f64)
            * num27 as f64) as f32;
        let num34 = (num7 as f64 * num12 as f64 - num8 as f64 * num11 as f64) as f32;
        let num35 = (num6 as f64 * num12 as f64 - num8 as f64 * num10 as f64) as f32;
        let num36 = (num6 as f64 * num11 as f64 - num7 as f64 * num10 as f64) as f32;
        let num37 = (num5 as f64 * num12 as f64 - num8 as f64 * num9 as f64) as f32;
        let num38 = (num5 as f64 * num11 as f64 - num7 as f64 * num9 as f64) as f32;
        let num39 = (num5 as f64 * num10 as f64 - num6 as f64 * num9 as f64) as f32;
        result.m14 = (-(num2 as f64 * num34 as f64 - num3 as f64 * num35 as f64
            + num4 as f64 * num36 as f64)
            * num27 as f64) as f32;
        result.m24 = ((num1 as f64 * num34 as f64 - num3 as f64 * num37 as f64
            + num4 as f64 * num38 as f64)
            * num27 as f64) as f32;
        result.m34 = (-(num1 as f64 * num35 as f64 - num2 as f64 * num37 as f64
            + num4 as f64 * num39 as f64)
            * num27 as f64) as f32;
        result.m44 = ((num1 as f64 * num36 as f64 - num2 as f64 * num38 as f64
            + num3 as f64 * num39 as f64)
            * num27 as f64) as f32;

        result
    }

    /// Creates a new matrix that contains linear interpolation of the values in specified matrixes.
    pub fn lerp_mut(&mut self, other: &Mat3f, amount: f32) {
        self.m11 = self.m11 + ((other.m11 - self.m11) * amount);
        self.m12 = self.m12 + ((other.m12 - self.m12) * amount);
        self.m13 = self.m13 + ((other.m13 - self.m13) * amount);
        self.m14 = self.m14 + ((other.m14 - self.m14) * amount);
        self.m21 = self.m21 + ((other.m21 - self.m21) * amount);
        self.m22 = self.m22 + ((other.m22 - self.m22) * amount);
        self.m23 = self.m23 + ((other.m23 - self.m23) * amount);
        self.m24 = self.m24 + ((other.m24 - self.m24) * amount);
        self.m31 = self.m31 + ((other.m31 - self.m31) * amount);
        self.m32 = self.m32 + ((other.m32 - self.m32) * amount);
        self.m33 = self.m33 + ((other.m33 - self.m33) * amount);
        self.m34 = self.m34 + ((other.m34 - self.m34) * amount);
        self.m41 = self.m41 + ((other.m41 - self.m41) * amount);
        self.m42 = self.m42 + ((other.m42 - self.m42) * amount);
        self.m43 = self.m43 + ((other.m43 - self.m43) * amount);
        self.m44 = self.m44 + ((other.m44 - self.m44) * amount);
    }

    /// Creates a new matrix that contains linear interpolation of the values in specified matrixes.
    pub fn lerp(&self, mat: &Self, amount: f32) -> Self {
        let mut m = self.clone();
        m.lerp_mut(mat, amount);
        m
    }
}

/// Arithmatic operators
impl Mat3f {
    /// Creates a new matrix that contains a multiplication of two matrix.
    ///
    /// c_{i,j} = a_{i, k} b_{k, j}
    pub fn multiply(matrix1: &Self, matrix2: &Self) -> Self {
        Self {
            m11: matrix1.m11 * matrix2.m11
                + matrix1.m12 * matrix2.m21
                + matrix1.m13 * matrix2.m31
                + matrix1.m14 * matrix2.m41,
            m12: matrix1.m11 * matrix2.m12
                + matrix1.m12 * matrix2.m22
                + matrix1.m13 * matrix2.m32
                + matrix1.m14 * matrix2.m42,
            m13: matrix1.m11 * matrix2.m13
                + matrix1.m12 * matrix2.m23
                + matrix1.m13 * matrix2.m33
                + matrix1.m14 * matrix2.m43,
            m14: matrix1.m11 * matrix2.m14
                + matrix1.m12 * matrix2.m24
                + matrix1.m13 * matrix2.m34
                + matrix1.m14 * matrix2.m44,
            m21: matrix1.m21 * matrix2.m11
                + matrix1.m22 * matrix2.m21
                + matrix1.m23 * matrix2.m31
                + matrix1.m24 * matrix2.m41,
            m22: matrix1.m21 * matrix2.m12
                + matrix1.m22 * matrix2.m22
                + matrix1.m23 * matrix2.m32
                + matrix1.m24 * matrix2.m42,
            m23: matrix1.m21 * matrix2.m13
                + matrix1.m22 * matrix2.m23
                + matrix1.m23 * matrix2.m33
                + matrix1.m24 * matrix2.m43,
            m24: matrix1.m21 * matrix2.m14
                + matrix1.m22 * matrix2.m24
                + matrix1.m23 * matrix2.m34
                + matrix1.m24 * matrix2.m44,
            m31: matrix1.m31 * matrix2.m11
                + matrix1.m32 * matrix2.m21
                + matrix1.m33 * matrix2.m31
                + matrix1.m34 * matrix2.m41,
            m32: matrix1.m31 * matrix2.m12
                + matrix1.m32 * matrix2.m22
                + matrix1.m33 * matrix2.m32
                + matrix1.m34 * matrix2.m42,
            m33: matrix1.m31 * matrix2.m13
                + matrix1.m32 * matrix2.m23
                + matrix1.m33 * matrix2.m33
                + matrix1.m34 * matrix2.m43,
            m34: matrix1.m31 * matrix2.m14
                + matrix1.m32 * matrix2.m24
                + matrix1.m33 * matrix2.m34
                + matrix1.m34 * matrix2.m44,
            m41: matrix1.m41 * matrix2.m11
                + matrix1.m42 * matrix2.m21
                + matrix1.m43 * matrix2.m31
                + matrix1.m44 * matrix2.m41,
            m42: matrix1.m41 * matrix2.m12
                + matrix1.m42 * matrix2.m22
                + matrix1.m43 * matrix2.m32
                + matrix1.m44 * matrix2.m42,
            m43: matrix1.m41 * matrix2.m13
                + matrix1.m42 * matrix2.m23
                + matrix1.m43 * matrix2.m33
                + matrix1.m44 * matrix2.m43,
            m44: matrix1.m41 * matrix2.m14
                + matrix1.m42 * matrix2.m24
                + matrix1.m43 * matrix2.m34
                + matrix1.m44 * matrix2.m44,
        }
    }

    pub fn divide_mut(&mut self, other: &Self) {
        self.m11 /= other.m11;
        self.m12 /= other.m12;
        self.m13 /= other.m13;
        self.m14 /= other.m14;
        self.m21 /= other.m21;
        self.m22 /= other.m22;
        self.m23 /= other.m23;
        self.m24 /= other.m24;
        self.m31 /= other.m31;
        self.m32 /= other.m32;
        self.m33 /= other.m33;
        self.m34 /= other.m34;
        self.m41 /= other.m41;
        self.m42 /= other.m42;
        self.m43 /= other.m43;
        self.m44 /= other.m44;
    }

    pub fn divide(&self, other: &Self) -> Self {
        let mut m = self.clone();
        m.divide_mut(other);
        m
    }
}

impl_op_ex!(-|me: &Mat3f| -> Mat3f {
    Mat3f {
        m11: -me.m11,
        m12: -me.m12,
        m13: -me.m13,
        m14: -me.m14,
        m21: -me.m21,
        m22: -me.m22,
        m23: -me.m23,
        m24: -me.m24,
        m31: -me.m31,
        m32: -me.m32,
        m33: -me.m33,
        m34: -me.m34,
        m41: -me.m41,
        m42: -me.m42,
        m43: -me.m43,
        m44: -me.m44,
    }
});

impl_op_ex!(+ |lhs: &Mat3f, rhs: &Mat3f| -> Mat3f {
    Mat3f::new(
    lhs.m11 +lhs.m11 ,
    lhs.m12 +lhs.m12 ,
    lhs.m13 +lhs.m13 ,
    lhs.m14 +lhs.m14 ,
    lhs.m21 +lhs.m21 ,
    lhs.m22 +lhs.m22 ,
    lhs.m23 +lhs.m23 ,
    lhs.m24 +lhs.m24 ,
    lhs.m31 +lhs.m31 ,
    lhs.m32 +lhs.m32 ,
    lhs.m33 +lhs.m33 ,
    lhs.m34 +lhs.m34 ,
    lhs.m41 +lhs.m41 ,
    lhs.m42 +lhs.m42 ,
    lhs.m43 +lhs.m43 ,
    lhs.m44 +lhs.m44 ,
    )
});

impl_op_ex!(+= |lhs: &mut Mat3f, rhs: &Mat3f| {
    lhs.m11 +=lhs.m11;
    lhs.m12 +=lhs.m12;
    lhs.m13 +=lhs.m13;
    lhs.m14 +=lhs.m14;
    lhs.m21 +=lhs.m21;
    lhs.m22 +=lhs.m22;
    lhs.m23 +=lhs.m23;
    lhs.m24 +=lhs.m24;
    lhs.m31 +=lhs.m31;
    lhs.m32 +=lhs.m32;
    lhs.m33 +=lhs.m33;
    lhs.m34 +=lhs.m34;
    lhs.m41 +=lhs.m41;
    lhs.m42 +=lhs.m42;
    lhs.m43 +=lhs.m43;
    lhs.m44 +=lhs.m44;
});

impl_op_ex!(-|a: &Mat3f, b: &Mat3f| -> Mat3f {
    Mat3f {
        m11: a.m11 - b.m11,
        m12: a.m12 - b.m12,
        m13: a.m13 - b.m13,
        m14: a.m14 - b.m14,
        m21: a.m21 - b.m21,
        m22: a.m22 - b.m22,
        m23: a.m23 - b.m23,
        m24: a.m24 - b.m24,
        m31: a.m31 - b.m31,
        m32: a.m32 - b.m32,
        m33: a.m33 - b.m33,
        m34: a.m34 - b.m34,
        m41: a.m41 - b.m41,
        m42: a.m42 - b.m42,
        m43: a.m43 - b.m43,
        m44: a.m44 - b.m44,
    }
});

impl_op_ex!(-= |lhs: &mut Mat3f, rhs: &Mat3f| {
    lhs.m11 -= lhs.m11;
    lhs.m12 -= lhs.m12;
    lhs.m13 -= lhs.m13;
    lhs.m14 -= lhs.m14;
    lhs.m21 -= lhs.m21;
    lhs.m22 -= lhs.m22;
    lhs.m23 -= lhs.m23;
    lhs.m24 -= lhs.m24;
    lhs.m31 -= lhs.m31;
    lhs.m32 -= lhs.m32;
    lhs.m33 -= lhs.m33;
    lhs.m34 -= lhs.m34;
    lhs.m41 -= lhs.m41;
    lhs.m42 -= lhs.m42;
    lhs.m43 -= lhs.m43;
    lhs.m44 -= lhs.m44;
});

impl_op_ex!(*|me: &Mat3f, scale: f32| -> Mat3f {
    Mat3f {
        m11: me.m11 * scale,
        m12: me.m12 * scale,
        m13: me.m13 * scale,
        m14: me.m14 * scale,
        m21: me.m21 * scale,
        m22: me.m22 * scale,
        m23: me.m23 * scale,
        m24: me.m24 * scale,
        m31: me.m31 * scale,
        m32: me.m32 * scale,
        m33: me.m33 * scale,
        m34: me.m34 * scale,
        m41: me.m41 * scale,
        m42: me.m42 * scale,
        m43: me.m43 * scale,
        m44: me.m44 * scale,
    }
});

impl_op_ex!(*= |me: &mut Mat3f, scale: f32| {
    me.m11 *= scale;
    me.m12 *= scale;
    me.m13 *= scale;
    me.m14 *= scale;
    me.m21 *= scale;
    me.m22 *= scale;
    me.m23 *= scale;
    me.m24 *= scale;
    me.m31 *= scale;
    me.m32 *= scale;
    me.m33 *= scale;
    me.m34 *= scale;
    me.m41 *= scale;
    me.m42 *= scale;
    me.m43 *= scale;
    me.m44 *= scale;
});

impl Mat3f {
    /// Swap the matrix rows and columns
    pub fn transpose(&mut self) -> Self {
        let mut m = Mat3f::default();

        m.m11 = self.m11;
        m.m12 = self.m21;
        m.m13 = self.m31;
        m.m14 = self.m41;

        m.m21 = self.m12;
        m.m22 = self.m22;
        m.m23 = self.m32;
        m.m24 = self.m42;

        m.m31 = self.m13;
        m.m32 = self.m23;
        m.m33 = self.m33;
        m.m34 = self.m43;

        m.m41 = self.m14;
        m.m42 = self.m24;
        m.m43 = self.m34;
        m.m44 = self.m44;
        m
    }

    pub fn transform_quaternion(&self, rot: &Quaternion) -> Self {
        let rot = Self::from_quaternion(rot);
        Self::multiply(&self, &rot)
    }
}

#[cfg(test)]
mod test {
    use super::Mat3f;
    use std::mem::size_of;

    fn test_size() {
        assert_eq!(size_of::<Mat3f>(), size_of::<f32>() * 16);
    }
}
