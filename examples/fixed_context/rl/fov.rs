use crate::utils::grid2d::Vec2i;

/// Clockwise
pub enum Octant {
    /// NEN
    A,
    /// ENE
    B,
    /// ESE
    C,
    /// SES
    D,
    E,
    F,
    G,
    H,
}

pub trait Fov {
    fn is_in_fov(&self, pos: Vec2i);
}

pub struct RelativeFovData {
    data: Vec<bool>,
}
