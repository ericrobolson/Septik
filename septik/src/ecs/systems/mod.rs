pub mod game_engine;
pub mod game_logic;
pub mod platform_specific;

use crate::ecs::{components, Entity, World};
use crate::lib_core::math::Rotation3d;

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

        let mut summed_cursor_deltas = Vec3d::default();

        for input in &engine_inputs.inputs {
            match input {
                InputType::CursorNormalized(_, cursor_pos) => {
                    summed_cursor_deltas += *cursor_pos;
                }
                InputType::Held(_, input_type) => {
                    match input_type {
                        EngineInputs::MoveForward => {
                            movement_vec.z += move_speed.value;
                        }
                        EngineInputs::MoveBack => {
                            movement_vec.z -= move_speed.value;
                        }
                        EngineInputs::MoveRight => {
                            movement_vec.x -= move_speed.value;
                        }
                        EngineInputs::MoveLeft => {
                            movement_vec.x += move_speed.value;
                        }
                        EngineInputs::Jump => {
                            movement_vec.y += move_speed.value;
                        }
                        EngineInputs::Crouch => {
                            movement_vec.y -= move_speed.value;
                        }
                        _ => {
                            // Ignore anything other than inputs
                        }
                    }
                }
                _ => {}
            }
        }

        // Update velocity
        let mut velocity = velocity.clone().unwrap();
        velocity.value = movement_vec;

        // Calculate rotation based on cursor
        velocity.rotational_velocity =
            apply_cursor_to_rotation(summed_cursor_deltas, Rotation3d::default());

        world.velocities[e] = Some(velocity);

        //TODO: this section should instead add the rotation to the velocity so it can be picked up by the collision detection
        {
            // Update transform rotation?

            // Update camera
            let camera = world.third_person_cameras[e].as_ref();
            world.third_person_cameras[e] = update_camera(camera, summed_cursor_deltas);
        }
    }
}

fn apply_cursor_to_rotation(summed_cursor_deltas: Vec3d, rotation: Rotation3d) -> Rotation3d {
    // Add the cursor motion to the current rotation angle so that the rotation is added to the previous rotations.
    // Sensitivity controls the speed of the rotation.
    // Apply cursor movements
    // https://sites.google.com/site/csc8820/educational/move-a-camera
    let sensitivity: FixedNumber = 10.into();

    let mut rotation = rotation;

    rotation.yaw_radians = {
        let mut yaw_radians = rotation.yaw_radians;
        yaw_radians -= sensitivity * summed_cursor_deltas.x;

        yaw_radians
    };

    rotation.pitch_radians = {
        let mut pitch_radians: FixedNumber = rotation.pitch_radians;

        let min_pitch: FixedNumber = FixedNumber::PI() / (-2).into();
        let max_pitch: FixedNumber = FixedNumber::PI() / 2.into();

        pitch_radians += sensitivity * summed_cursor_deltas.y;

        // Cap it so that the max/min angles are -90* and 90*
        FixedNumber::max(FixedNumber::min(pitch_radians, max_pitch), min_pitch)
    };

    rotation
}

fn update_camera(
    camera: std::option::Option<&components::ThirdPersonCameraComponent>,
    summed_cursor_deltas: Vec3d,
) -> Option<components::ThirdPersonCameraComponent> {
    if camera.is_none() {
        return None;
    }

    let mut camera = camera.unwrap().clone();

    let mut summed_cursor_deltas = summed_cursor_deltas;
    let mut rotation = apply_cursor_to_rotation(summed_cursor_deltas, camera.rotation);

    camera.rotation = rotation;

    return Some(camera);
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
        let velocity = world.velocities[e].as_ref();
        let transform = world.transforms[e].as_ref();

        if velocity.is_none() || transform.is_none() {
            continue;
        }

        let velocity = velocity.as_ref().unwrap();
        let mut transform = transform.unwrap().clone();

        transform.position += velocity.value;
        transform.rotation += velocity.rotational_velocity;
        world.transforms[e] = Some(transform);
    }
}
