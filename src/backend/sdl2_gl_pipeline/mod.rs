mod programs;
mod sprite_render_pass;
mod text_render_pass_gl;
mod textures;
use super::*;
use sprite_render_pass::SpriteRenderPipeline;
use text_render_pass_gl::TextRenderPipeline;

extern crate nalgebra as na;
use na::Vector2;

extern crate gl;

pub struct Sdl2GlRenderPipelineArgs {
    pub strings: Vec<(String, na::Vector3<f32>)>,
    pub window_height: u32,
    pub window_width: u32,
}

pub struct Sdl2GlRenderPipeline {
    text_render_pipeline: TextRenderPipeline,
    sprite_render_pipeline: SpriteRenderPipeline,
    program: programs::Program,
}

impl Sdl2GlRenderPipeline {
    pub fn execute(&mut self, args: &Sdl2GlRenderPipelineArgs) {
        //TODO: fix up after sprites are available. self.text_render_pipeline.execute(&args, &self.program);
        self.sprite_render_pipeline.execute(&args, &self.program);
    }
}

impl Drop for Sdl2GlRenderPipeline {
    fn drop(&mut self) {}
}

pub fn init_sdl2_gl_render_pipeline() -> Sdl2GlRenderPipeline {
    let program = programs::Program::default_program().unwrap();

    let text_render_pipeline = TextRenderPipeline::new(&program);

    let sprite_render_pipeline = SpriteRenderPipeline::new(&program);

    return Sdl2GlRenderPipeline {
        text_render_pipeline: text_render_pipeline,
        sprite_render_pipeline: sprite_render_pipeline,
        program: program,
    };
}
