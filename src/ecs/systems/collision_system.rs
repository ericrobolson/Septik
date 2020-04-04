use specs::prelude::*;

pub struct CollisionSystem;

impl<'a> System<'a> for CollisionSystem {
    type SystemData = ();

    fn run(&mut self, (): Self::SystemData) {}
}
