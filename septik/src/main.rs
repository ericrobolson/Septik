pub mod ecs;
pub mod lib_core;
pub mod platform_specifc;
pub mod util;

fn main() {
    let mut wingfx = platform_specifc::WindowGfxBuilder::build();
    let mut world = ecs::World::new();

    loop {
        let inputs = wingfx.poll_input();
        world.register_player_inputs(&inputs);

        world.dispatch();

        wingfx.render(&world);
    }
}
