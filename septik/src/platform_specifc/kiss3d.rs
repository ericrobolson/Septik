use super::{WindowGfx, *};
use crate::lib_core::math::Rotation3d;

use crate::ecs::{components::gfx_components, Entity, World};
use crate::lib_core::{math::Vec3d, EngineInputs, InputType};

extern crate cgmath;
use cgmath::prelude::*;

extern crate mint;
use mint::Quaternion;

extern crate kiss3d;
use kiss3d::{event::Key, event::WindowEvent, resource::Mesh, scene::SceneNode, window::Window};
use std::env;

use nalgebra;
use std::collections::HashMap;

pub struct Kiss3dBackend {
    window: Window,
    inputs: Vec<InputType>,
    mesh_data: HashMap<Entity, Mesh>,
    voxel_data: HashMap<Entity, std::vec::Vec<((usize, usize, usize), SceneNode)>>,
}

impl Kiss3dBackend {
    pub fn new() -> Self {
        let mut window = kiss3d::window::Window::new("Kiss3d Backend");

        window.set_background_color(1.0, 1.0, 1.0);

        window.set_light(kiss3d::light::Light::StickToCamera);

        return Self {
            window: window,
            inputs: vec![],
            voxel_data: HashMap::new(),
            mesh_data: HashMap::new(),
        };
    }
}

impl WindowGfx for Kiss3dBackend {
    fn poll_input(&mut self) -> Vec<InputType> {
        // Consume + return inputs
        let inputs = self.inputs.clone();

        self.inputs.clear();

        return inputs;
    }

    fn render(&mut self, world: &World) {
        if self.window.render() {
            // Log inputs, no matter what
            // Then on 'poll_inputs()', simply drain the logged inputs
            {
                let todo_player_id = 0; // TODO: fix this up to the local player

                for event in self.window.events().iter() {
                    match event.value {
                        WindowEvent::CursorPos(x, y, _modif) => {
                            let x = x as f32;
                            let y = y as f32;

                            let window_w = self.window.size()[0] as f32;
                            let window_h = self.window.size()[1] as f32;

                            // Normalize from 0..1
                            let x = x / window_w;
                            let y = y / window_h;

                            // Shift so that it's mapped from -1..1 and matches the NDC for CursorNormalized
                            let x = ((-0.5) + x) * 2.0;
                            let y = ((-0.5) + y) * -2.0;

                            self.inputs.push(InputType::CursorNormalized(
                                todo_player_id,
                                Vec3d::new(x.into(), y.into(), 0.into()),
                            ));
                        }
                        WindowEvent::Key(key, action, modif) => {
                            println!("key event {:?} on {:?} with {:?}", key, action, modif);

                            match key {
                                Key::W => {
                                    // Forward
                                    self.inputs.push(InputType::Held(
                                        todo_player_id,
                                        EngineInputs::MoveForward,
                                    ));
                                }
                                Key::A => {
                                    // Left
                                    self.inputs.push(InputType::Held(
                                        todo_player_id,
                                        EngineInputs::MoveLeft,
                                    ));
                                }
                                Key::S => {
                                    // Back
                                    self.inputs.push(InputType::Held(
                                        todo_player_id,
                                        EngineInputs::MoveBack,
                                    ));
                                }
                                Key::D => {
                                    // Right
                                    self.inputs.push(InputType::Held(
                                        todo_player_id,
                                        EngineInputs::MoveRight,
                                    ));
                                }
                                Key::Space => {
                                    // Jump
                                    self.inputs
                                        .push(InputType::Held(todo_player_id, EngineInputs::Jump));
                                }
                                Key::LControl => {
                                    // Jump
                                    self.inputs.push(InputType::Held(
                                        todo_player_id,
                                        EngineInputs::Crouch,
                                    ));
                                }
                                _ => {
                                    // Nothing
                                }
                            }
                        }

                        _ => {}
                    }
                }
            }

            for e in world.entities() {
                update_camera(self, world, e);
                draw_chunks(self, world, e);
                draw_meshes(self, world, e);
            }
        }
    }
}

fn rotation_into_quaternion(rotation: Rotation3d) -> cgmath::Quaternion<f32> {
    let yaw = cgmath::Rad::<f32>(rotation.yaw_radians.into());
    let pitch = cgmath::Rad::<f32>(rotation.pitch_radians.into());
    let roll = cgmath::Rad::<f32>(rotation.roll_radians.into());

    let yaw = cgmath::Quaternion::<f32>::from_angle_y(yaw);
    let pitch = cgmath::Quaternion::<f32>::from_angle_x(pitch);
    let roll = cgmath::Quaternion::<f32>::from_angle_z(roll);

    // Note: order is important!
    return (yaw * pitch * roll);
}

fn update_camera(backend: &mut Kiss3dBackend, world: &World, e: Entity) {
    return;
    /*
    let camera = world.third_person_cameras[e].as_ref();

    if camera.is_some() {
        let camera = camera.unwrap();

        let transform = world.transforms[e].as_ref();
        if transform.is_some() {
            let transform = transform.unwrap();
            let base_pos = transform.position;

            // Testing to figure out best approach

            let actual_pos = camera.relative_position + base_pos;

            backend
                .cam
                .set_orientation(rotation_into_quaternion(camera.rotation));

            backend.cam.set_position([
                actual_pos.x.into(),
                actual_pos.y.into(),
                actual_pos.z.into(),
            ]);
        }
    }
    */
}

fn draw_meshes(backend: &mut Kiss3dBackend, world: &World, e: Entity) {
    return;
    /*
    let mesh = world.meshes[e].as_ref();

    if mesh.is_some() {
        let mesh = &mesh.unwrap();

        // If the mesh exists, update it
        if backend.mesh_data.contains_key(&e) {
            let mut three_mesh_data = backend.mesh_data.get_mut(&e);
            if three_mesh_data.is_none() {
                return;
            }

            let mut three_mesh_data = three_mesh_data.unwrap();
            let transform = world.transforms[e].as_ref();
            if transform.is_some() {
                let transform = transform.unwrap();
                let pos = transform.position;

                three_mesh_data.set_orientation(rotation_into_quaternion(transform.rotation));

                three_mesh_data.set_position([pos.x.into(), pos.y.into(), pos.z.into()]);
            }
        }
        // The mesh does not exist, so create it
        else {
            let (mut group_map, _meshes) = match mesh.mesh {
                mesh_component::Mesh::Monkey => {
                    let mut args = env::args();
                    let obj_path = concat!(env!("CARGO_MANIFEST_DIR"), "/test_data/untitled.obj");
                    let path = args.nth(1).unwrap_or(obj_path.into());
                    backend.window.factory.load_obj(&path)
                }
            };

            let transform = world.transforms[e].as_ref();
            let group = backend.window.factory.group();

            for g in group_map.values_mut() {
                if transform.is_some() {
                    let transform = transform.unwrap();
                    let pos = transform.position;
                    g.set_position([pos.x.into(), pos.y.into(), pos.z.into()]);
                }
                group.add(g);
            }

            backend.window.scene.add(&group);

            backend.mesh_data.insert(e, group);
        }
    }
    */
}

fn draw_chunks(backend: &mut Kiss3dBackend, world: &World, e: Entity) {
    let chunk = world.voxel_chunks[e].as_ref();

    if chunk.is_some() {
        let chunk = &chunk.unwrap().chunk;

        // If the chunk exists, update it
        if backend.voxel_data.contains_key(&e) {
            let mut three_mesh_data = backend.voxel_data.get_mut(&e);
            if three_mesh_data.is_none() {
                return;
            }

            let mut three_mesh_data = three_mesh_data.unwrap();

            for ((x, y, z), mesh) in three_mesh_data.iter_mut() {
                let voxel = chunk.voxels[*x][*y][*z];
                //TODO: voxel updates update the mesh
            }
        }
        // The chunk doesn't exist, so create it
        else {
            let mut mesh_data = vec![];

            for (x, x_row) in chunk.voxels.iter().enumerate() {
                for (y, y_row) in x_row.iter().enumerate() {
                    for (z, z_row) in y_row.iter().enumerate() {
                        let scale = 3.0;

                        let mbox = {
                            let mut geometry =
                                backend
                                    .window
                                    .add_cube(scale * 1.0, scale * 1.0, scale * 1.0);
                            geometry.append_translation(&nalgebra::Translation3::new(
                                x as f32 * scale,
                                y as f32 * scale,
                                z as f32 * scale,
                            ));

                            geometry.set_color(0.0, 1.0, 0.0);
                            geometry.set_points_size(10.0);
                            geometry.set_lines_width(1.0);
                            geometry.set_surface_rendering_activation(false);

                            geometry
                        };
                        //TODO: position
                        //mbox.set_position([x as f32 * scale, y as f32 * scale, z as f32 * scale]);

                        mesh_data.push(((x, y, z), mbox));
                    }
                }
            }

            backend.voxel_data.insert(e, mesh_data);
        }
    }
}
