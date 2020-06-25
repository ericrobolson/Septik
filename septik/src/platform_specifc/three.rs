use super::{WindowGfx, *};
use crate::lib_core::math::Rotation3d;

use crate::ecs::{components::gfx_components, Entity, World};
use crate::lib_core::{math::Vec3d, EngineInputs, InputType};

extern crate cgmath;
use cgmath::prelude::*;

extern crate mint;
use mint::Quaternion;

extern crate three;
use std::env;
use three::Object;

use std::collections::HashMap;

pub struct ThreeRsBackend {
    window: three::Window,
    inputs: Vec<InputType>,
    cam: three::camera::Camera,
    controls: three::controls::Orbit,
    mesh_data: HashMap<Entity, three::Group>,
    voxel_data: HashMap<Entity, std::vec::Vec<((usize, usize, usize), three::Mesh)>>,
}

impl ThreeRsBackend {
    pub fn new() -> Self {
        let mut win = three::Window::new("Three-rs obj loading example");
        win.scene.background = three::Background::Color(0xFFFFFF);

        let cam = win.factory.perspective_camera(60.0, 1.0..1000.0);
        let mut controls = three::controls::Orbit::builder(&cam)
            .position([0.0, 2.0, -5.0])
            .target([0.0, 0.0, 0.0])
            .build();

        let dir_light = win.factory.directional_light(0xffffff, 0.9);
        dir_light.look_at([15.0, 35.0, 35.0], [0.0, 0.0, 2.0], None);

        win.scene.add(&dir_light);

        return Self {
            window: win,
            inputs: vec![],
            cam: cam,
            controls: controls,
            voxel_data: HashMap::new(),
            mesh_data: HashMap::new(),
        };
    }
}

impl WindowGfx for ThreeRsBackend {
    fn poll_input(&mut self) -> Vec<InputType> {
        // Consume + return inputs
        let inputs = self.inputs.clone();

        self.inputs.clear();

        return inputs;
    }

    fn render(&mut self, world: &World) {
        if self.window.update() {
            // Log inputs, no matter what
            // Then on 'poll_inputs()', simply drain the logged inputs
            {
                let keys_hit = self.window.input.keys_hit();
                let mouse_wheel_movements = self.window.input.mouse_wheel_movements();
                let mouse_movements = self.window.input.mouse_movements_ndc();

                let todo_player_id = 0; // TODO: fix this up to the local player

                for movement in mouse_movements {
                    let x = movement.x;
                    let y = movement.y;

                    self.inputs.push(InputType::CursorNormalized(
                        todo_player_id,
                        Vec3d::new(x.into(), y.into(), 0.into()),
                    ))
                }

                for key in keys_hit {
                    match key {
                        three::controls::Key::W => {
                            // Forward
                            self.inputs
                                .push(InputType::Held(todo_player_id, EngineInputs::MoveForward));
                        }
                        three::controls::Key::A => {
                            // Left
                            self.inputs
                                .push(InputType::Held(todo_player_id, EngineInputs::MoveLeft));
                        }
                        three::controls::Key::S => {
                            // Back
                            self.inputs
                                .push(InputType::Held(todo_player_id, EngineInputs::MoveBack));
                        }
                        three::controls::Key::D => {
                            // Right
                            self.inputs
                                .push(InputType::Held(todo_player_id, EngineInputs::MoveRight));
                        }
                        three::controls::Key::Space => {
                            // Jump
                            self.inputs
                                .push(InputType::Held(todo_player_id, EngineInputs::Jump));
                        }
                        three::controls::Key::LControl => {
                            // Jump
                            self.inputs
                                .push(InputType::Held(todo_player_id, EngineInputs::Crouch));
                        }
                        _ => {
                            // Nothing
                        }
                    }
                }
            }

            for e in world.entities() {
                update_camera(self, world, e);
                draw_chunks(self, world, e);
                draw_meshes(self, world, e);
            }

            self.window.render(&self.cam);
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

fn update_camera(backend: &mut ThreeRsBackend, world: &World, e: Entity) {
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
}

fn draw_meshes(backend: &mut ThreeRsBackend, world: &World, e: Entity) {
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
                gfx_components::Mesh::Monkey => {
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
}

fn draw_chunks(backend: &mut ThreeRsBackend, world: &World, e: Entity) {
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
            let mut three_mesh_data = vec![];

            for (x, x_row) in chunk.voxels.iter().enumerate() {
                for (y, y_row) in x_row.iter().enumerate() {
                    for (z, z_row) in y_row.iter().enumerate() {
                        let scale = 3.0;

                        let mbox = {
                            let geometry =
                                three::Geometry::cuboid(scale * 1.0, scale * 1.0, scale * 1.0);
                            let material = three::material::Wireframe { color: 0x00FF00 };
                            backend.window.factory.mesh(geometry, material)
                        };
                        mbox.set_position([x as f32 * scale, y as f32 * scale, z as f32 * scale]);
                        backend.window.scene.add(&mbox);

                        three_mesh_data.push(((x, y, z), mbox));
                    }
                }
            }

            backend.voxel_data.insert(e, three_mesh_data);
        }
    }
}
