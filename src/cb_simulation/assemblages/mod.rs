// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

extern crate specs;
use specs::prelude::*;

use crate::cb_math;
use cb_math::FUint;

use crate::cb_simulation::components;
use components::network_component::NetworkedComponent;
use components::physics_components::{CircleComponent, PositionComponent, VelocityComponent};

pub trait Assemblage {
    fn id() -> u32;
}

pub fn new_player(world: &mut specs::World) {
    // Physics components
    let position = PositionComponent::new(100, 300);
    let velocity = VelocityComponent::new(0, 0);
    let circle = CircleComponent::new(10);

    // Let networkedComponent
    let network = NetworkedComponent::new();

    // TODO: what about a 'ghostable' component, that contains the assemblage id?

    world
        .create_entity()
        .with(position)
        .with(velocity)
        .with(circle)
        .with(network)
        .build();
}
