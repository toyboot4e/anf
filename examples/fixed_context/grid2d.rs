//! Desrete geometry types

/// Screen bounds in pixels
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect2i {
    pos: Vec2i,
    size: Vec2u,
}

impl Rect2i {
    pub fn new(xs: impl Into<[u32; 2]>, ws: impl Into<[u32; 2]>) -> Self {
        let xs = xs.into();
        let ws = ws.into();
        Self {
            pos: Vec2i::new(xs[0] as i32, xs[1] as i32),
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
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

/// Point in pixels
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2u {
    pub x: u32,
    pub y: u32,
}

impl Vec2i {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Vec2u {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

// pub struct Camera {
//     scroll: Vec2i,
//     // rot,
// }
