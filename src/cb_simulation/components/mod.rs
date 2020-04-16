// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

// Note; do this using ECS

extern crate specs;
use specs::prelude::*;

use crate::cb_math;
use cb_math::cb_range::CbNormalizedRange;

pub mod audio;
pub mod network_component;
pub mod physics_components;

/// A simple trait for linking components to the world
pub trait ComponentLinker {
    /// Register the set of components for the world
    fn register_components(world: &mut World);
}

pub struct BitStream {}
