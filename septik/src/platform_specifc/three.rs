use super::{WindowGfx, *};

use crate::ecs::{Entity, World};
use crate::lib_core::InputType;

extern crate three;
use std::env;
use three::Object;

use std::collections::HashMap;

pub struct ThreeRsBackend {
    window: three::Window,
    cam: three::camera::Camera,
    controls: three::controls::Orbit,
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

        // Load a monkey for debugging
        {
            let mut args = env::args();
            let obj_path = concat!(env!("CARGO_MANIFEST_DIR"), "/test_data/untitled.obj");
            let path = args.nth(1).unwrap_or(obj_path.into());
            let root = win.factory.group();
            win.scene.add(&root);
            let (mut group_map, _meshes) = win.factory.load_obj(&path);
            for g in group_map.values_mut() {
                root.add(g);
            }
        }
        return Self {
            window: win,
            cam: cam,
            controls: controls,
            voxel_data: HashMap::new(),
        };
    }
}

impl WindowGfx for ThreeRsBackend {
    fn poll_input(&mut self) -> std::vec::Vec<InputType> {
        //TODO:!
        return vec![];
    }
    fn render(&mut self, world: &World) {
        // TODO:

        // Add some simple voxels to play with
        for e in world.entities() {
            let chunk = world.voxel_chunks[e].as_ref();

            if chunk.is_some() {
                let chunk = &chunk.unwrap().chunk;

                if self.voxel_data.contains_key(&e) {
                    let mut three_mesh_data = self.voxel_data.get_mut(&e);
                    if three_mesh_data.is_none() {
                        continue;
                    }

                    let mut three_mesh_data = three_mesh_data.unwrap();

                    for ((x, y, z), mesh) in three_mesh_data.iter_mut() {
                        let voxel = chunk.voxels[*x][*y][*z];
                        //TODO: voxel updates
                    }
                } else {
                    let mut three_mesh_data = vec![];

                    for (x, x_row) in chunk.voxels.iter().enumerate() {
                        for (y, y_row) in x_row.iter().enumerate() {
                            for (z, z_row) in y_row.iter().enumerate() {
                                let mbox = {
                                    let geometry = three::Geometry::cuboid(1.0, 1.0, 1.0);
                                    let material = three::material::Wireframe { color: 0x00FF00 };
                                    self.window.factory.mesh(geometry, material)
                                };
                                mbox.set_position([x as f32, y as f32, z as f32]);
                                self.window.scene.add(&mbox);

                                three_mesh_data.push(((x, y, z), mbox));
                            }
                        }
                    }

                    self.voxel_data.insert(e, three_mesh_data);
                }
            }
        }

        // Render
        if self.window.update() && !self.window.input.hit(three::KEY_ESCAPE) {
            self.controls.update(&self.window.input);
            self.window.render(&self.cam);
        }
    }
}
