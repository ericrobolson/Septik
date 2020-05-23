pub mod ecs;
pub mod lib_core;
pub mod platform_specifc;

extern crate three;

use std::env;
use three::Object;
fn main() {
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

    // Add some simple voxels to play with
    {
        for x in 0..8 {
            for y in 0..8 {
                for z in 0..8 {
                    let mbox = {
                        let geometry = three::Geometry::cuboid(1.0, 1.0, 1.0);
                        let material = three::material::Wireframe { color: 0x00FF00 };
                        win.factory.mesh(geometry, material)
                    };
                    mbox.set_position([x as f32, y as f32, z as f32]);
                    win.scene.add(&mbox);
                }
            }
        }
    }

    // Render
    while win.update() && !win.input.hit(three::KEY_ESCAPE) {
        controls.update(&win.input);
        win.render(&cam);
    }
}
