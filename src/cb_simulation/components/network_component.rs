// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

extern crate specs;
use specs::prelude::*;

use crate::cb_simulation;

use super::ComponentLinker;

init_components![NetworkComponentsLinker, (NetworkedComponent)];

pub struct NetworkedComponent {}

impl NetworkedComponent {
    pub fn new() -> Self {
        return Self {};
    }
}
