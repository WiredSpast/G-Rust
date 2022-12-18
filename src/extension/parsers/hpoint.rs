use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct HPoint {
    pub x: i32,
    pub y: i32,
    pub z: f32
}

impl Display for HPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl HPoint {
    pub fn new_2d(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            z: 0.0
        }
    }

    pub fn new_3d(x: i32, y: i32, z: f32) -> Self {
        Self {
            x,
            y,
            z
        }
    }
}