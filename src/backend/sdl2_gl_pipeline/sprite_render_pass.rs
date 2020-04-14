extern crate gl;
extern crate nalgebra as na;
use na::{Isometry3, Matrix4, Orthographic3, Vector2, Vector3};

use image::{DynamicImage, Rgba};
use rusttype::{point, Font, Scale};

use std::collections::HashMap;

use super::programs::*;
use super::textures::*;
use super::*;

// Based off of https://learnopengl.com/In-Practice/2D-Game/Rendering-Sprites

pub struct SpriteRenderPipeline {
    vbo: u32,
    vao: u32,
    todo_remove_texture: Texture,
}

impl SpriteRenderPipeline {
    pub fn new(program: &programs::Program) -> Self {
        let (vbo, vao) = create_vbos_vaos_quads();

        let texture = Texture::new();

        return Self {
            vbo: vbo,
            vao: vao,
            todo_remove_texture: texture,
        };
    }

    pub fn execute(&self, pipeline_args: &Sdl2GlRenderPipelineArgs, program: &Program) {
        // Early short out if nothing to render

        let execution_pass = || {
            // Pre render setup
            {
                unsafe {
                    gl::Enable(gl::TEXTURE_2D);
                    // gl::Disable(gl::LIGHTING);
                    gl::Disable(gl::BLEND);
                }

                // Setup projections
                {
                    let projection = Orthographic3::new(
                        0.0,
                        pipeline_args.window_width as f32,
                        0.0,
                        pipeline_args.window_height as f32,
                        -1.0,
                        1.0,
                    )
                    .to_homogeneous();

                    program.set_uniform(ProgramUniforms::Projection(projection));
                }
            }

            // Now for each sprite, render it
            {
                let todo_sprite_color = Vector3::new(0.5, 0.5, 0.5);
                let todo_position = Vector2::new(0.0, 0.0);

                let todo_size = Vector2::new(1000.0, 1000.0);

                // Calculate model
                let model: Matrix4<f32>;
                {
                    let m = Vector3::new(todo_position.x, todo_position.y, 0.0);
                    let m = Isometry3::new(m, na::zero());

                    let m = m.to_homogeneous();
                    /*
                    //TODO: rotation, scaling
                                        let m = m.append_translation(&Vector3::new(
                                            0.5 * todo_size.x,
                                            0.5 * todo_size.y,
                                            0.0,
                                        ));

                                        // TODO: rotate

                                        let m = m.append_translation(&Vector3::new(
                                            -0.5 * todo_size.x,
                                            -0.5 * todo_size.y,
                                            0.0,
                                        ));

                    */
                    let m =
                        m.append_nonuniform_scaling(&Vector3::new(todo_size.x, todo_size.y, 1.0));

                    model = m;
                }
                /*
                //model = glm::translate(model, glm::vec3(position, 0.0f));

                //model = glm::translate(model, glm::vec3(0.5f * size.x, 0.5f * size.y, 0.0f));
                model = glm::rotate(model, rotate, glm::vec3(0.0f, 0.0f, 1.0f));
                //model = glm::translate(model, glm::vec3(-0.5f * size.x, -0.5f * size.y, 0.0f));

                //model = glm::scale(model, glm::vec3(size, 1.0f));
                */

                //TODO: Set uniforms
                {
                    println!("model: {}", model);
                    program.set_uniform(ProgramUniforms::Model(model));
                    program.set_uniform(ProgramUniforms::SpriteColor(todo_sprite_color));
                }

                //Bind texture
                {
                    unsafe {
                        gl::ActiveTexture(gl::TEXTURE0);
                    }

                    self.todo_remove_texture.bind();
                }

                // Bind vertex array + draw
                unsafe {
                    gl::BindVertexArray(self.vao);
                    gl::DrawArrays(gl::TRIANGLES, 0, 6);
                    gl::BindVertexArray(0);
                }
            }
        };

        program.execute(&mut || execution_pass());
    }
}

impl Drop for SpriteRenderPipeline {
    fn drop(&mut self) {
        // Drop textures?

        // Dispose of vbo, vao
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}

fn create_vbos_vaos_quads() -> (u32, u32) {
    let mut vao = 0;
    let mut vbo = 0;

    // 1 vec2 for position, 1 vec2 for tex
    let vertices = vec![
        0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 0.0, 1.0, 0.0,
    ];

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
            vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        gl::BindVertexArray(vao);
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0,
            4,
            gl::FLOAT,
            gl::FALSE,
            (4 * std::mem::size_of::<f32>()) as i32,
            std::ptr::null(),
        );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    return (vbo, vao);
}
