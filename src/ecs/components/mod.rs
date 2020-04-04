use specs::prelude::*;

pub mod gfx;
pub mod physics;
pub mod units;

/// Register all components to the world
pub fn register(world: &mut World) {
    physics::register(world);
    units::register(world);
    gfx::register(world);
}
