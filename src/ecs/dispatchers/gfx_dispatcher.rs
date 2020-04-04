use specs::prelude::*;

use crate::ecs;
use ecs::systems::gfx::RenderSystem;

pub fn gfx_build_dispatcher<'a, 'b>() -> specs::Dispatcher<'a, 'b> {
    return DispatcherBuilder::new()
        .with_barrier()
        .with(RenderSystem, "render system", &[])
        .build();
}
