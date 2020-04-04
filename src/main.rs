// Macro enabled crates
#[macro_use]
pub mod macros;

// Non macro crates
pub mod ecs;
mod game_interface;
pub mod types;

fn main() {
    let mut sim = game_interface::SkGameInterface::new();

    while sim.should_exit() == false {
        sim.advance();
        sim.render();
    }
}
