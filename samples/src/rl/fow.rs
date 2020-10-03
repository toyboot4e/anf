//! Fog of war, i.e., shadows on map

/// Fog of war
pub struct Fow {
    map_size: [usize; 2],
    data: Vec<bool>,
}

impl Fow {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            map_size: [w, h],
            data: vec![false; w * h],
        }
    }
}
