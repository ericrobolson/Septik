// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

extern crate specs;
use specs::prelude::*;

use crate::cb_math;
use cb_math::FUint;

use crate::cb_simulation::components::character_components;
use character_components::{
    ArmorComponent, HitPointsComponent, MoveSpeedComponent, RangedAttackComponent,
    UnitBaseComponent,
};

use crate::cb_simulation::components::physics_components;
use physics_components::{TransformComponent, VelocityComponent};

use crate::cb_simulation::components::gfx_components;
use gfx_components::SpriteComponent;

pub fn new_unit(world: &mut specs::World) {
    // RTS components
    let armor = ArmorComponent::new(FUint::from_num(20), FUint::from_num(20));
    let hit_points = HitPointsComponent::new(FUint::from_num(20), FUint::from_num(20));
    let move_speed = MoveSpeedComponent::new(FUint::from_num(10));
    let ranged_atk =
        RangedAttackComponent::new(FUint::from_num(1), FUint::from_num(20), FUint::from_num(1));
    let base = UnitBaseComponent::new(FUint::from_num(100));

    // Physics components
    let transform = TransformComponent::new();
    let velocity = VelocityComponent::new();

    // Gfx Components
    let sprite = SpriteComponent::new();

    world
        .create_entity()
        .with(armor)
        .with(move_speed)
        .with(ranged_atk)
        .with(base)
        .with(transform)
        .with(velocity)
        .with(sprite)
        .build();
}
