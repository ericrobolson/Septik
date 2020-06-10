use super::{WindowGfx, *};
use crate::lib_core::math::Rotation3d;

use crate::ecs::{components::mesh_component, Entity, World};
use crate::lib_core::{math::Vec3d, EngineInputs, InputType};

extern crate gl;
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::video::GLProfile;

pub struct Sdl2Backend {
    sdl_context: sdl2::Sdl,
    event_pump: sdl2::EventPump,
    context: sdl2::video::GLContext,
    window: sdl2::video::Window,
    inputs: Vec<InputType>,
}

impl Sdl2Backend {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let window = video_subsystem
            .window("Window", 800, 600)
            .opengl()
            .build()
            .unwrap();

        // Unlike the other example above, nobody created a context for your window, so you need to create one.
        let ctx = window.gl_create_context().unwrap();
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

        debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
        debug_assert_eq!(gl_attr.context_version(), (3, 3));

        let mut event_pump = sdl_context.event_pump().unwrap();

        return Self {
            sdl_context: sdl_context,
            event_pump: event_pump,
            window: window,
            context: ctx,
            inputs: vec![],
        };
    }

    fn read_input(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => println!("quit!"),
                _ => {}
            }
        }
    }
}

impl WindowGfx for Sdl2Backend {
    fn poll_input(&mut self) -> Vec<InputType> {
        // Consume + return inputs
        let inputs = self.inputs.clone();

        self.inputs.clear();

        return inputs;
    }

    fn render(&mut self, world: &World) {
        self.read_input();
        // Rendering portion:
        {
            unsafe {
                gl::ClearColor(0.6, 0.0, 0.8, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            self.window.gl_swap_window();
        }
    }
}
