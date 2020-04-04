use specs::prelude::*;

pub struct PositionSystem;

impl<'a> System<'a> for PositionSystem {
    type SystemData = ();

    fn run(&mut self, (): Self::SystemData) {}
}
