use specs::prelude::*;

use crate::types;
use types::Coordinate2d;

init_components![VelocityComponent, TransformComponent];

pub struct VelocityComponent(Coordinate2d);

impl VelocityComponent {
    pub fn new() -> Self {
        return Self(Coordinate2d::zero());
    }
}

pub struct TransformComponent {
    pub world_position: Coordinate2d,
    pub rotation: Coordinate2d,
    pub scale: Coordinate2d,
}

impl TransformComponent {
    pub fn new() -> Self {
        return Self {
            world_position: Coordinate2d::zero(),
            rotation: Coordinate2d::zero(),
            scale: Coordinate2d::one(),
        };
    }
}
