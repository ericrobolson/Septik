// Copyright 2020, Eric Olson, All rights reserved. Contact eric.rob.olson@gmail.com for questions regarding use.

extern crate gl;

use std::ffi::{CStr, CString};
extern crate nalgebra as na;
use na::{Matrix4, Orthographic3, Vector2, Vector3};
use std::collections::HashMap;

pub struct Program {
    id: gl::types::GLuint,
    uniforms: HashMap<String, i32>,
}

pub trait Uniform {
    fn get_uniform_str(&self) -> &str;
}

pub enum ProgramUniforms {
    TextColor((f32, f32, f32)),
    TextProjectionUniform(Matrix4<f32>),
    //
    Model(Matrix4<f32>),
    Projection(Matrix4<f32>),
    //
    SpriteColor(Vector3<f32>),
}

impl Uniform for ProgramUniforms {
    fn get_uniform_str(&self) -> &str {
        match self {
            ProgramUniforms::TextColor(_) => {
                return "textColor";
            }
            ProgramUniforms::TextProjectionUniform(_) => {
                return "textProjection";
            }
            ProgramUniforms::Model(_) => {
                return "model";
            }
            ProgramUniforms::Projection(_) => {
                return "projection";
            }

            ProgramUniforms::SpriteColor(_) => {
                return "spriteColor";
            }
        }
    }
}

impl Program {
    pub fn default_program() -> Result<Program, String> {
        let static_sprite_vert_shader;
        let static_sprite_frag_shader;
        {
            static_sprite_vert_shader = Shader::from_vert_source(
                &CString::new(include_str!("shaders\\gen\\static_sprite.vert")).unwrap(),
            )
            .unwrap();

            static_sprite_frag_shader = Shader::from_frag_source(
                &CString::new(include_str!("shaders\\gen\\static_sprite.frag")).unwrap(),
            )
            .unwrap();
        }

        return program_from_shaders(&[
            //text_vert_shader, //NOTE: there's an issue with multiple shaders for some reason; has to do with the fact that only one main can exist for a group of shaders
            //text_frag_shader, //NOTE: there's an issue with multiple shaders for some reason; has to do with the fact that only one main can exist for a group of shaders
            static_sprite_vert_shader,
            static_sprite_frag_shader,
        ]);
    }

    pub fn execute<F>(&self, func: &mut F)
    where
        F: FnMut() -> (),
    {
        //Note: using a lambda to make sure that the program is actually active
        unsafe {
            gl::UseProgram(self.id);
        }
        func();
    }

    pub fn id(&self) -> gl::types::GLuint {
        return self.id;
    }

    pub fn set_uniform(&self, uniform: ProgramUniforms) {
        //Note: using an enum for this, as each program is limited to a single uniform name.
        match uniform {
            ProgramUniforms::TextColor((r, g, b)) => {
                // Upload the text color
                unsafe {
                    gl::Uniform3f(self.get_uniform_id(uniform), r, g, b);
                }
            }
            ProgramUniforms::SpriteColor(color) => {
                // Upload the text color
                unsafe {
                    gl::Uniform3f(self.get_uniform_id(uniform), color.x, color.y, color.z);
                }
            }
            ProgramUniforms::TextProjectionUniform(projection) => {
                // Set the projection uniform
                unsafe {
                    gl::UniformMatrix4fv(
                        self.get_uniform_id(uniform),
                        1,
                        gl::FALSE,
                        projection.as_ptr(),
                    );
                }
            }
            ProgramUniforms::Model(model) => {
                // Set the projection uniform
                unsafe {
                    gl::UniformMatrix4fv(
                        self.get_uniform_id(uniform),
                        1,
                        gl::FALSE,
                        model.as_ptr(),
                    );
                }
            }
            ProgramUniforms::Projection(projection) => {
                // Set the projection uniform
                unsafe {
                    gl::UniformMatrix4fv(
                        self.get_uniform_id(uniform),
                        1,
                        gl::FALSE,
                        projection.as_ptr(),
                    );
                }
            }
        }
    }

    fn get_uniform_id(&self, uniform: ProgramUniforms) -> i32 {
        return self.uniforms[uniform.get_uniform_str()];
    }

    fn generate_uniform(&self, uniform_str: &str) -> i32 {
        let uniform_str = &CString::new(uniform_str).unwrap();

        unsafe {
            return gl::GetUniformLocation(self.id(), uniform_str.as_ptr());
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

fn program_from_shaders(shaders: &[Shader]) -> Result<Program, String> {
    let program_id = unsafe { gl::CreateProgram() };
    for shader in shaders {
        unsafe {
            gl::AttachShader(program_id, shader.id());
        };
    }

    unsafe {
        gl::LinkProgram(program_id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
    }

    if success == 0 {
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let error = create_whitespace_cstring_with_len(len as usize);
        unsafe {
            gl::GetProgramInfoLog(
                program_id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }

        return Err(error.to_string_lossy().into_owned());
    }

    // Continue error handling

    for shader in shaders {
        unsafe {
            gl::DetachShader(program_id, shader.id());
        }
    }

    let mut program = Program {
        id: program_id,
        uniforms: HashMap::new(),
    };
    bind_uniforms(&mut program);

    return Ok(program);
}

fn bind_uniforms(program: &mut Program) {
    // Set uniforms
    {
        let dummy_mat4 = Orthographic3::new(0.1, 0.2, 0.3, 0.4, 0.5, 0.6).to_homogeneous();
        let dummy_vec3 = Vector3::new(1.0, 1.0, 1.0);

        //TextColor
        {
            let text_color_str = ProgramUniforms::TextColor((0.0, 0.0, 0.0)).get_uniform_str();
            let text_color_uniform_id = program.generate_uniform(text_color_str);

            program
                .uniforms
                .insert(text_color_str.to_string(), text_color_uniform_id);
        }

        //TextProjection
        {
            let value = ProgramUniforms::TextProjectionUniform(dummy_mat4);
            let uniform_str = value.get_uniform_str();

            let id = program.generate_uniform(uniform_str);

            program.uniforms.insert(uniform_str.to_string(), id);
        }

        //Model
        {
            let value = ProgramUniforms::Model(dummy_mat4);
            let uniform_str = value.get_uniform_str();

            let id = program.generate_uniform(uniform_str);

            program.uniforms.insert(uniform_str.to_string(), id);
        }
        //Projection
        {
            let value = ProgramUniforms::Projection(dummy_mat4);
            let uniform_str = value.get_uniform_str();

            let id = program.generate_uniform(uniform_str);

            program.uniforms.insert(uniform_str.to_string(), id);
        }
        //SpriteColor
        {
            let value = ProgramUniforms::SpriteColor(dummy_vec3);
            let uniform_str = value.get_uniform_str();

            let id = program.generate_uniform(uniform_str);

            program.uniforms.insert(uniform_str.to_string(), id);
        }
    }
}

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_source(source: &CStr, kind: gl::types::GLenum) -> Result<Shader, String> {
        let id = shader_from_source(source, kind)?;
        return Ok(Shader { id: id });
    }

    pub fn from_vert_source(source: &CStr) -> Result<Shader, String> {
        return Shader::from_source(source, gl::VERTEX_SHADER);
    }

    pub fn from_frag_source(source: &CStr) -> Result<Shader, String> {
        return Shader::from_source(source, gl::FRAGMENT_SHADER);
    }

    pub fn id(&self) -> gl::types::GLuint {
        return self.id;
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len as usize));
    return unsafe { CString::from_vec_unchecked(buffer) };
}

fn shader_from_source(source: &CStr, kind: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
    let id = unsafe { gl::CreateShader(kind) };

    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }

    const FAILURE_CODE: gl::types::GLint = 0;
    if success == FAILURE_CODE {
        // error handling
        let mut len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }

        let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
        buffer.extend([b' '].iter().cycle().take(len as usize));
        let error = create_whitespace_cstring_with_len(len as usize);
        unsafe {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar,
            );
        }

        return Err(error.to_string_lossy().into_owned());
    }

    return Ok(id);
}
