use specs::prelude::*;

use crate::ecs;
use ecs::{components, dispatchers};

pub struct SkGameInterface<'a, 'b> {
    gfx_dispatcher: specs::Dispatcher<'a, 'b>,
    sim_dispatcher: specs::Dispatcher<'a, 'b>,
    world: World,
}

impl<'a, 'b> SkGameInterface<'a, 'b> {
    pub fn new() -> Self {
        let mut world = World::new();
        components::register(&mut world);

        let gfx_dispatcher = dispatchers::gfx_build_dispatcher();
        let sim_dispatcher = dispatchers::sim_build_dispatcher();

        return Self {
            world: world,
            gfx_dispatcher: gfx_dispatcher,
            sim_dispatcher: sim_dispatcher,
        };
    }

    pub fn should_exit(&self) -> bool {
        return false;
    }

    fn should_run(&self) -> bool {
        return true;
    }

    pub fn advance(&mut self) {
        if self.should_run() {
            self.sim_dispatcher.dispatch(&self.world);
            self.world.maintain();
        }
    }

    pub fn render(&mut self) {
        self.gfx_dispatcher.dispatch(&self.world);
        //self.world.maintain();
    }
}
