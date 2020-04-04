use specs::prelude::*;

use crate::ecs;
use ecs::systems::{collision_system, position_system};

pub fn sim_build_dispatcher<'a, 'b>() -> specs::Dispatcher<'a, 'b> {
    return DispatcherBuilder::new()
        .with(collision_system::CollisionSystem, "collision system", &[])
        .with_barrier()
        .with(position_system::PositionSystem, "position system", &[])
        .build();
}
