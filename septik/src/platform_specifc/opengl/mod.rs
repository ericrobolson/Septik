use crate::ecs::{Entity, World};
use crate::lib_core::{math::Vec3d, EngineInputs, InputType};

use std::ffi::{CStr, CString};

extern crate gl;

mod helpers;

mod program;
mod shaders;
mod sprite;
mod texture;
mod vao;
mod vbo;
mod vertices;

#[derive(Copy, Clone)]
pub struct Resolution {
    pub height: u32,
    pub width: u32,
}

pub struct SpritePass {
    program: program::Program,
    vao: vao::Vao,
    vbo: vbo::Vbo,
    sprite: sprite::Sprite,
    frame_increment: usize,
}

impl SpritePass {
    pub fn new(resolution: Resolution) -> Self {
        // Enables alpha
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Disable(gl::MULTISAMPLE);
        }

        let vert_shader = shaders::Shader::from_vert_source(
            &CString::new(include_str!("shader_implementations\\sprite.vert")).unwrap(),
        )
        .unwrap();

        let frag_shader = shaders::Shader::from_frag_source(
            &CString::new(include_str!("shader_implementations\\sprite.frag")).unwrap(),
        )
        .unwrap();

        let shader_program = program::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

        //NOTE: this is just some dummy data
        //let tex = texture::Texture::new("0001.png".to_string());
        //let sprite = sprite::Sprite::new(&tex, Some(13), None, Some(64), Some(64));

        let tex = texture::Texture::new("baseset_Case01.png".to_string());
        let sprite = sprite::Sprite::new(&tex, None, None, None, None);

        let mut sprite_verts = sprite.clone().into_verts(resolution);

        tex.bind();

        let mut vbo = vbo::Vbo::new();
        let mut vao = vao::Vao::new();

        vao.buffer(&mut vbo, &sprite_verts);

        Self {
            program: shader_program,
            vao: vao,
            vbo: vbo,
            sprite: sprite,
            frame_increment: 0,
        }
    }

    pub fn render(&mut self, resolution: Resolution, world: &World) {
        // TODO: if sprites updated, rebuffer

        self.program.set_used();

        self.frame_increment += 1;

        if self.frame_increment % 50 == 0 {
            // self.sprite.increment_frame();

            self.vao
                .buffer(&mut self.vbo, &self.sprite.into_verts(resolution));
        }

        self.vao.render(&self.vbo);
    }
}

pub struct OpenGlRenderer {
    sprite_pass: SpritePass,
    pub resolution: Resolution,
}

impl OpenGlRenderer {
    /// Create a new OpenGL render backend
    pub fn new(resolution: Resolution) -> Self {
        let sprite_pass = SpritePass::new(resolution);

        Self {
            sprite_pass: sprite_pass,
            resolution: resolution,
        }
    }

    /// Set the viewport
    pub fn set_viewport(&mut self, width: i32, height: i32) {
        self.resolution = Resolution {
            width: width as u32,
            height: height as u32,
        };

        unsafe {
            gl::Viewport(0, 0, width, height);
        }
    }

    /// Execute all render passes
    pub fn render(&mut self, world: &World) {
        // Clear the canvas
        unsafe {
            gl::ClearColor(0.6, 0.0, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Execute the various passes
        self.sprite_pass.render(self.resolution, world);
    }
}
