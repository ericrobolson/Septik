#[macro_use]
extern crate pest_derive;

pub mod ecs;
pub mod lib_core;
pub mod platform_specifc;
pub mod scripting;
pub mod util;

use std::io;
use std::io::prelude::*;

fn main() {
    let mut slisp = scripting::Slisp::new();
    let mut wingfx = platform_specifc::WindowGfxBuilder::build();
    let mut world = ecs::World::new();

    let repl = true;

    if repl {
        loop {
            print!("-> ");
            io::stdout().flush().unwrap();

            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                let script = line.unwrap();
                let result = slisp.read_str(script);

                match result {
                    Ok(value) => {
                        println!("READ: {:?}", value);
                        let eval = slisp.eval(value);

                        match eval {
                            Ok(value) => {
                                println!("EVAL: {:?}", value);
                            }
                            Err(error) => {
                                println!("!! -> {}", error);
                            }
                        }
                    }
                    Err(error) => {
                        println!("!! -> {}", error);
                    }
                }

                print!("-> ");
                io::stdout().flush().unwrap();
            }
        }
    }

    loop {
        let inputs = wingfx.poll_input();
        world.register_player_inputs(&inputs);

        world.dispatch();

        wingfx.render(&world);
    }
}
