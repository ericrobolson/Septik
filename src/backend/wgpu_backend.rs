extern crate gl;

extern crate nalgebra as na;
use na::Vector2;

use super::*;

use crate::backend;
use backend::SysEvent;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

#[allow(dead_code)]
pub struct GlutinBackend {
    window_width: u32,
    window_height: u32,
}

impl GlutinBackend {
    pub fn new() -> Self {
        let window_width = 1280;
        let window_height = 720;

        let el = EventLoop::new();
        let wb = WindowBuilder::new().with_title("A fantastic window!");

        let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();

        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        println!(
            "Pixel format of the window's GL context: {:?}",
            windowed_context.get_pixel_format()
        );

        let gl = support::load(&windowed_context.context());

        el.run(move |event, _, control_flow| {
            println!("{:?}", event);
            *control_flow = ControlFlow::Wait;

            match event {
                Event::LoopDestroyed => return,
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    gl.draw_frame([1.0, 0.5, 0.7, 1.0]);
                    windowed_context.swap_buffers().unwrap();
                }
                _ => (),
            }
        });

        return Self {
            window_width: window_width,
            window_height: window_height,
        };
    }
}

impl Backend for GlutinBackend {
    fn render(&mut self, node: Node) {
        /*
        unsafe {
            gl::ClearColor(1.0, 1.0, 1.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        */
    }

    fn poll_events(&mut self) -> SysEvent {
        let mut sys_event = SysEvent::new();
        /*
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
        */

        return sys_event;
    }
}
