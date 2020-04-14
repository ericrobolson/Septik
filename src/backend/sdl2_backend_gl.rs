extern crate gl;
extern crate sdl2;
use sdl2::video::GLProfile;

extern crate nalgebra as na;
use na::Vector2;

use super::*;

use crate::backend;
use backend::SysEvent;

#[allow(dead_code)]
pub struct Sdl2BackendGL {
    sdl_context: sdl2::Sdl,
    window: sdl2::video::Window,
    video_subsystem: sdl2::VideoSubsystem,
    gl_context: sdl2::video::GLContext, // Need this to keep the OpenGL context active
    render_pipeline: sdl2_gl_pipeline::Sdl2GlRenderPipeline,
    window_width: u32,
    window_height: u32,
}

impl Sdl2BackendGL {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();

        // Init OpenGL
        let video_subsystem = sdl_context.video().unwrap();
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 2);

        let window_width = 1280;
        let window_height = 720;

        let window = video_subsystem
            .window("Window", window_width, window_height)
            .opengl()
            .build()
            .unwrap();

        let ctx = window.gl_create_context().unwrap();
        gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
        debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
        // If mobile: debug_assert_eq!(gl_attr.context_profile(), GLProfile::GLES);
        debug_assert_eq!(gl_attr.context_version(), (3, 2));

        return Self {
            sdl_context: sdl_context,
            window: window,
            video_subsystem: video_subsystem,
            gl_context: ctx,
            render_pipeline: sdl2_gl_pipeline::init_sdl2_gl_render_pipeline(),
            window_width: window_width,
            window_height: window_height,
        };
    }
}

impl Backend for Sdl2BackendGL {
    fn render(&mut self, node: Node) {
        let mut text_to_render = vec![];

        let todo_text_color = na::Vector3::new(1.0, 1.0, 1.0);

        match node {
            Node::Text(text) => text_to_render.push((text, todo_text_color)),
            _ => {}
        }

        // Dummy value
        text_to_render.push(("blah".to_string(), todo_text_color));

        let pipeline_args = sdl2_gl_pipeline::Sdl2GlRenderPipelineArgs {
            strings: text_to_render,
            window_width: self.window_width,
            window_height: self.window_height,
        };

        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        self.render_pipeline.execute(&pipeline_args);
        self.window.gl_swap_window();
    }

    fn poll_events(&mut self) -> SysEvent {
        let sys_events: Vec<sdl2::event::Event> =
            self.sdl_context.event_pump().unwrap().poll_iter().collect();

        let mut sys_event = SysEvent::new();

        for event in sys_events {
            match event {
                sdl2::event::Event::MouseMotion {
                    timestamp: _,
                    window_id: _,
                    which: _,
                    x,
                    y,
                    xrel: _,
                    yrel: _,
                    mousestate: _,
                } => {
                    sys_event.add_cursor_move(x as u32, y as u32);
                }
                sdl2::event::Event::MouseButtonDown {
                    timestamp: _,
                    window_id: _,
                    which: _,
                    mouse_btn: _,
                    clicks: _,
                    x,
                    y,
                } => {
                    sys_event.add_cursor_click(x as u32, y as u32);
                }
                sdl2::event::Event::MouseButtonUp {
                    timestamp: _,
                    window_id: _,
                    which: _,
                    mouse_btn: _,
                    clicks: _,
                    x,
                    y,
                } => {
                    sys_event.add_cursor_release(x as u32, y as u32);
                }
                _ => {}
            }
        }

        return sys_event;
    }
}
