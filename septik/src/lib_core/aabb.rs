use super::*;

use super::math::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Aabb {
    pub min: Vec3d,
    pub max: Vec3d,
}

impl Aabb {
    pub fn new(min: Vec3d, max: Vec3d) -> Self {
        Self { min: min, max: max }
    }
}
