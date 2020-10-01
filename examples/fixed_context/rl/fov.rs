//! Field of view for orthogonal grid world

use crate::utils::grid2d::Vec2i;

pub trait FovWrite {
    fn light(&mut self, pos: Vec2i);
}

/// Map bounds and opacities
pub trait OpacityMap {
    fn is_opaque(&self, pos: Vec2i) -> bool;
    fn contains(&self, pos: Vec2i) -> bool;
}

#[derive(Debug, Clone)]
pub struct FovData {
    data: Vec<bool>,
    radius: u32,
    /// Where the character is
    origin: Vec2i,
}

impl FovData {
    pub fn new(max_radius: usize) -> Self {
        let edge = max_radius * 2 + 1;
        let data = vec![false; edge * edge];

        Self {
            data,
            origin: Vec2i::default(),

            radius: 0,
        }
    }

    fn ix(&self, mut pos: Vec2i) -> usize {
        let edge = self.radius * 2 + 1;
        pos += Vec2i::new(self.radius as i32, self.radius as i32);
        (pos.x as u32 + pos.y as u32 * edge) as usize
    }

    pub fn is_in_view(&self, pos: Vec2i) -> bool {
        let delta = pos - self.origin;
        if delta.len_king() > self.radius {
            return false; // out of scope
        }

        let ix = self.ix(pos);
        self.data[ix]
    }

    pub fn update(&mut self, r: u32, origin: Vec2i, opa: &mut impl OpacityMap) {
        // rebind
        self.radius = r;
        self.origin = origin;

        // TODO: resize if needed

        for i in 0..self.data.len() {
            self.data[i] = false;
        }

        self::update_fov(r, origin, self, opa);
        self.light(origin);
    }
}

impl FovWrite for FovData {
    fn light(&mut self, pos: Vec2i) {
        let delta = pos - self.origin;
        if delta.len_king() > self.radius {
            return; // out of scope
        }

        let ix = self.ix(pos);
        self.data[ix] = true;
    }
}

fn update_fov(r: u32, origin: Vec2i, fov: &mut impl FovWrite, opa: &impl OpacityMap) {
    for oct in &Octant::clockwise() {
        let mut scx = ScanContext::new(r, origin, *oct, fov, opa);
        let mut scanner = Scanner::new();
        scanner.run(1, &mut scx);
    }
}

// --------------------------------------------------------------------------------
// Internals

struct ScanContext<'a, T: FovWrite, U: OpacityMap> {
    /// Radius
    r: u32,
    /// Where the character is
    origin: Vec2i,
    /// Octant
    oct: OctantContext,
    fov: &'a mut T,
    opa: &'a U,
}

impl<'a, T: FovWrite, U: OpacityMap> ScanContext<'a, T, U> {
    pub fn new(r: u32, origin: Vec2i, oct: Octant, fov: &'a mut T, opa: &'a U) -> Self {
        Self {
            r,
            origin,
            oct: OctantContext::from_octant(oct),
            fov,
            opa,
        }
    }

    pub fn rc2pos(&self, row: i32, col: i32) -> Vec2i {
        self.origin + row * self.oct.row + col * self.oct.col
    }
}

struct Scanner {
    /// Slope = col / row in range [0, 1]
    slopes: [f32; 2],
}

impl Scanner {
    fn new() -> Self {
        Self { slopes: [0.0, 1.0] }
    }

    pub fn run<T: FovWrite, U: OpacityMap>(&mut self, row_from: u32, scx: &mut ScanContext<T, U>) {
        for row in row_from..scx.r {
            if !self.scan_row(row, scx) {
                break;
            }
        }
    }

    fn col_range(&self, row: u32, r: u32) -> [u32; 2] {
        let from = self.slopes[0] * row as f32;
        let to = {
            let to = self.slopes[1] * row as f32;
            let to_max = ((r as f32 + 0.5) * (r as f32 + 0.5) - row as f32 * row as f32).sqrt();
            std::cmp::min(to.floor() as u32, to_max.floor() as u32)
        };
        [from.ceil() as u32, to]
    }

    fn scan_row<T: FovWrite, U: OpacityMap>(
        &mut self,
        row: u32,
        scx: &mut ScanContext<T, U>,
    ) -> bool {
        if self.slopes[0] > self.slopes[1] {
            return false;
        }

        let mut state = ScanState::Initial;

        let cols = self.col_range(row, scx.r);
        for col in cols[0]..=cols[1] {
            let pos = scx.rc2pos(row as i32, col as i32);
            if !scx.opa.contains(pos) {
                return true;
            }

            if scx.opa.is_opaque(pos) {
                if state == ScanState::Transparent {
                    let mut sub = Self {
                        // left-up
                        slopes: [self.slopes[0], (col as f32 - 0.5) / (row as f32 + 0.5)],
                    };
                    sub.run(row + 1, scx);
                }

                state = ScanState::Opaque;
            } else {
                if state == ScanState::Opaque {
                    // right-down
                    self.slopes[0] = (col as f32 + 0.5) / (row as f32 - 0.5);
                }

                state = ScanState::Transparent;
            }

            scx.fov.light(pos);
        }

        // permissive scan only for opaque cell
        let col = (self.slopes[1] * row as f32).ceil() as u32;
        if col > cols[1] {
            let pos = scx.rc2pos(row as i32, col as i32);
            if scx.opa.is_opaque(pos) {
                scx.fov.light(pos);
                // left-up
                self.slopes[1] = (col as f32 - 0.5) / (row as f32 + 0.5);
            }
        }

        state == ScanState::Transparent
    }
}

#[derive(PartialEq)]
enum ScanState {
    /// Initial scan
    Initial,
    /// Previous scan was on opaquecell
    Opaque,
    /// Previous scan was on transparent cell
    Transparent,
}

struct OctantContext {
    row: Vec2i,
    col: Vec2i,
}

impl OctantContext {
    pub fn from_octant(oct: Octant) -> Self {
        let units = oct.to_units();
        Self {
            row: units[0],
            col: units[1],
        }
    }
}

/// Clockwise
#[derive(Debug, Clone, Copy)]
enum Octant {
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

impl Octant {
    pub fn to_units(&self) -> [Vec2i; 2] {
        match self {
            Octant::A => [Vec2i::new(0, -1), Vec2i::new(1, 0)],
            Octant::B => [Vec2i::new(1, 0), Vec2i::new(0, -1)],
            Octant::C => [Vec2i::new(1, 0), Vec2i::new(0, 1)],
            Octant::D => [Vec2i::new(0, 1), Vec2i::new(1, 0)],
            Octant::E => [Vec2i::new(0, 1), Vec2i::new(-1, 0)],
            Octant::F => [Vec2i::new(-1, 0), Vec2i::new(0, 1)],
            Octant::G => [Vec2i::new(-1, 0), Vec2i::new(0, -1)],
            Octant::H => [Vec2i::new(0, -1), Vec2i::new(-1, 0)],
        }
    }

    pub const fn clockwise() -> [Self; 8] {
        use Octant::*;
        [A, B, C, D, E, F, G, H]
    }
}
