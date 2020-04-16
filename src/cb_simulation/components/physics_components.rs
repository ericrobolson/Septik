// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

extern crate specs;
use specs::prelude::*;

use crate::cb_simulation;

use super::ComponentLinker;

init_components![
    PhysicsComponentsLinker,
    (CircleComponent, PositionComponent, VelocityComponent)
];

pub struct PositionComponent {
    x: i32,
    y: i32,
}

impl PositionComponent {
    pub fn new(x: i32, y: i32) -> Self {
        return Self { x: x, y: y };
    }
}

pub struct VelocityComponent {
    x: i32,
    y: i32,
}

impl VelocityComponent {
    pub fn new(x: i32, y: i32) -> Self {
        return Self { x: x, y: y };
    }
}

pub struct CircleComponent {
    radius: i32,
}

impl CircleComponent {
    pub fn new(r: i32) -> Self {
        return Self { radius: r };
    }
}
