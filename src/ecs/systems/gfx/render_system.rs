use specs::prelude::*;

pub struct RenderSystem;

impl<'a> System<'a> for RenderSystem {
    type SystemData = ();

    fn run(&mut self, (): Self::SystemData) {}
}
