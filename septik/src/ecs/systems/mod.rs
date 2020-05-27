pub mod game_engine;
pub mod game_logic;
pub mod platform_specific;

use crate::ecs::{components, World};

use crate::lib_core::{math::FixedNumber, math::Vec3d, Direction, EngineInputs, InputType};

/// This system applies character actions from inputs
pub fn character_action_system(world: &mut World) {
    for e in world.entities() {
        let velocity = &world.velocities[e];
        let engine_inputs = &world.engine_inputs[e];
        let move_speed = &world.move_speeds[e];

        if velocity.is_none() || engine_inputs.is_none() || move_speed.is_none() {
            continue;
        }

        let move_speed = move_speed.as_ref().unwrap();
        let engine_inputs = engine_inputs.as_ref().unwrap();

        let mut movement_vec = Vec3d::default();
        let mut rotation_vec = Vec3d::default();

        for input in &engine_inputs.inputs {
            match input {
                InputType::Held(_, input_type) => {
                    match input_type {
                        EngineInputs::MoveDown => {
                            movement_vec.y += move_speed.value;
                        }
                        EngineInputs::MoveUp => {
                            movement_vec.y -= move_speed.value;
                        }
                        EngineInputs::MoveLeft => {
                            movement_vec.x -= move_speed.value;
                        }
                        EngineInputs::MoveRight => {
                            movement_vec.x += move_speed.value;
                        }
                        _ => {
                            // Ignore anything other than inputs
                        }
                    }
                }
                _ => {}
            }
        }

        // TODO: if camera is present, orient the movement vector towards it

        let mut velocity = velocity.clone().unwrap();
        velocity.value = movement_vec;

        world.velocities[e] = Some(velocity);
    }
}

/// This system cleans up any input, leaving a blank slate for the next run.
pub fn input_cleanup_system(world: &mut World) {
    for e in world.entities() {
        let engine_inputs = world.engine_inputs[e].clone();

        if engine_inputs.is_none() {
            continue;
        }

        let mut engine_inputs = engine_inputs.unwrap();

        engine_inputs.inputs = vec![];

        world.engine_inputs[e] = Some(engine_inputs);
    }
}

/// Apply velocities to the positions
pub fn position_update_system(world: &mut World) {
    for e in world.entities() {
        let velocity = world.velocities[e].clone();
        let transform = world.transforms[e].clone();

        if velocity.is_none() || transform.is_none() {
            continue;
        }

        let velocity = velocity.as_ref().unwrap();
        let mut transform = transform.clone().unwrap();

        transform.position += velocity.value;
        world.transforms[e] = Some(transform);
    }
}
