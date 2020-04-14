extern crate gl;
extern crate nalgebra as na;
use na::{Matrix4, Orthographic3, Vector2};

use image::{DynamicImage, Rgba};
use rusttype::{point, Font, Scale};

use std::collections::HashMap;

use super::programs::*;
use super::*;

pub struct TextShader {}

impl TextShader {
    pub fn new(program: &programs::Program) -> Self {
        return Self {};
    }
}

pub struct TextRenderPipeline {
    char_map: HashMap<char, Character>,
    shader: TextShader,
    vbo: u32,
    vao: u32,
}

impl TextRenderPipeline {
    pub fn new(program: &programs::Program) -> Self {
        let char_map = init_chars_texture_from_font_gl();

        let (vbo, vao) = create_vbos_vaos_quads();

        let shader = TextShader::new(&program);

        return Self {
            char_map: char_map,
            vbo: vbo,
            vao: vao,
            shader,
        };
    }

    pub fn execute(&self, pipeline_args: &Sdl2GlRenderPipelineArgs, program: &Program) {
        // Early short out if nothing to render
        if pipeline_args.strings.is_empty() {
            return;
        }

        let execution_pass = || {
            // Pre render setup
            {
                // Enable blending
                {
                    unsafe {
                        gl::Enable(gl::BLEND);
                        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
                    }
                }

                // Setup projections
                {
                    let projection = Orthographic3::new(
                        0.0,
                        pipeline_args.window_width as f32,
                        0.0,
                        pipeline_args.window_height as f32,
                        0.001,
                        10000.0,
                    )
                    .to_homogeneous();

                    program.set_uniform(ProgramUniforms::TextProjectionUniform(projection));
                }
            }
            // Render each string
            for (string, color) in pipeline_args.strings.iter() {
                // let string_pos be a normalized range, from 0..1
                let string_pos = na::Vector2::new(0.0, 0.0); //TODO: pass in!!!
                let string_scale = 100.0;
                let mut x = string_pos.x;
                let y = string_pos.y;

                // Activate corresponding render state
                {
                    // Set the color
                    program.set_uniform(ProgramUniforms::TextColor((color.x, color.y, color.z)));

                    // Activate tex + bind vao
                    unsafe {
                        gl::ActiveTexture(gl::TEXTURE0);
                        gl::BindVertexArray(self.vao);
                    }
                }

                // Iterate through all the characters
                for character in string.chars() {
                    let ch = &self.char_map[&character];

                    println!("char: {:?}", ch);

                    if ch.texture_id.is_some() {
                        let xpos = x + ch.bearing.x * string_scale;
                        let ypos = y - (ch.size.y - ch.bearing.y) * string_scale;

                        let w = ch.size.x * string_scale;
                        let h = ch.size.y * string_scale;

                        // Update VBO for each character
                        let mut vertices = vec![];
                        vertices.append(&mut vec![xpos, ypos + h, 0.0, 0.0]);
                        vertices.append(&mut vec![xpos, ypos, 0.0, 1.0]);
                        vertices.append(&mut vec![xpos + w, ypos, 1.0, 1.0]);
                        vertices.append(&mut vec![xpos, ypos + h, 0.0, 0.0]);
                        vertices.append(&mut vec![xpos + w, ypos, 1.0, 1.0]);
                        vertices.append(&mut vec![xpos + w, ypos + h, 1.0, 0.0]);

                        //TODO: this is a place holder to debug textures
                        {
                            vertices = vec![];
                            vertices.append(&mut vec![0.0, 0.0 + 200.0, 0.0, 0.0]);
                            vertices.append(&mut vec![0.0, 0.0, 0.0, 1.0]);
                            vertices.append(&mut vec![0.0 + 200.0, 0.0, 1.0, 1.0]);
                            vertices.append(&mut vec![0.0, 0.0 + 200.0, 0.0, 0.0]);
                            vertices.append(&mut vec![0.0 + 200.0, 0.0, 1.0, 1.0]);
                            vertices.append(&mut vec![0.0 + 200.0, 0.0 + 200.0, 1.0, 0.0]);
                        }
                        let texture_id = ch.texture_id.unwrap();

                        unsafe {
                            // Render glyph texture over quad
                            gl::BindTexture(gl::TEXTURE_2D, texture_id);
                            // Update content of VBO memory
                            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
                            gl::BufferSubData(
                                gl::ARRAY_BUFFER,
                                0,
                                (vertices.len() * std::mem::size_of::<f32>())
                                    as gl::types::GLsizeiptr,
                                vertices.as_ptr() as *const gl::types::GLvoid,
                            );
                            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
                            // Render quad
                            gl::DrawArrays(gl::TRIANGLES, 0, 6);
                        }
                    }

                    // Now advance cursors for next glyph (note that advance is number of 1/64 pixels)
                    // Advance to the next position; need to convert to pixels
                    let advance_in_pixels = ch.advance * 64.0;
                    x += advance_in_pixels * string_scale;
                }

                unsafe {
                    // Reset; may be done
                    gl::BindVertexArray(0);
                    gl::BindTexture(gl::TEXTURE_2D, 0);
                }
            }

            println!("executed!");
        };

        program.execute(&mut || execution_pass());
    }
}

impl Drop for TextRenderPipeline {
    fn drop(&mut self) {
        for (key, ch) in &self.char_map {
            if ch.texture_id.is_some() {
                unsafe {
                    gl::DeleteTextures(1, &ch.texture_id.unwrap());
                }
            }
        }

        // Dispose of vbo, vao
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}

#[derive(Debug)]
struct Character {
    pub bearing: na::Vector2<f32>,
    pub size: na::Vector2<f32>,
    pub advance: f32,
    pub texture_id: Option<u32>,
}

fn create_vbos_vaos_quads() -> (u32, u32) {
    let mut vao = 0;
    let mut vbo = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (6 * 4 * std::mem::size_of::<f32>()) as isize,
            std::ptr::null(),
            gl::DYNAMIC_DRAW,
        );
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

fn is_whitespace(c: char) -> bool {
    char::is_whitespace(c)
}

fn init_chars_texture_from_font_gl() -> HashMap<char, Character> {
    let font_data = include_bytes!("../../assets/arial.ttf");
    // This only succeeds if collection consists of one font
    let font = Font::from_bytes(font_data as &[u8]).expect("Error constructing Font");

    // The font size to use
    let scale = Scale::uniform(48.0);

    // Render info
    let colour_black = (0, 0, 0);
    let v_metrics = font.v_metrics(scale);

    // Init character map
    let mut char_map = HashMap::<char, Character>::new();
    for i in 32..128 {
        //TODO: change from 32 to 0
        //TODO: unicode optimizations?
        //TODO: perform unicode normalization crate: [unicode-normalization = "0.1.12"]
        let c: char = char::from(i);
        let is_whitespace = is_whitespace(c);

        let mut character = Character {
            advance: 10.0,                            // Set defaults if whitespace
            bearing: Vector2::<f32>::new(10.0, 10.0), // set defaults if whitespace
            size: Vector2::<f32>::new(10.0, 10.0),    // set defaults if whitespace
            texture_id: None,
        };

        if is_whitespace == false {
            // The text to render
            let mut text = String::new();

            text.push(c);

            // layout the glyphs in a line with 20 pixels padding
            let glyphs: Vec<_> = font
                .layout(text.as_str(), scale, point(20.0, 20.0 + v_metrics.ascent))
                .collect();

            // work out the layout size
            let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
            let glyphs_width = {
                let min_x = glyphs
                    .first()
                    .map(|g| g.pixel_bounding_box().unwrap().min.x)
                    .unwrap();
                let max_x = glyphs
                    .last()
                    .map(|g| g.pixel_bounding_box().unwrap().max.x)
                    .unwrap();
                (max_x - min_x) as u32
            };

            // Create a new rgba image with some padding
            let mut image =
                DynamicImage::new_rgba8(glyphs_width + 40, glyphs_height + 40).to_rgba();

            // Loop through the glyphs in the text, positing each one on a line
            character.advance = 0.0;

            for glyph in glyphs {
                if let Some(bounding_box) = glyph.pixel_bounding_box() {
                    // Calculate metrics
                    {
                        character.size.x = bounding_box.width() as f32;
                        character.size.y = bounding_box.height() as f32;

                        let position = glyph.position();
                        character.bearing.x = position.x;
                        character.bearing.y = position.y;

                        character.advance += glyph.unpositioned().h_metrics().advance_width;
                    }

                    // Draw the glyph into the image per-pixel by using the draw closure
                    glyph.draw(|x, y, v| {
                        image.put_pixel(
                            // Offset the position by the glyph bounding box
                            x + bounding_box.min.x as u32,
                            y + bounding_box.min.y as u32,
                            // Turn the coverage into an alpha value
                            Rgba([
                                colour_black.0,
                                colour_black.1,
                                colour_black.2,
                                (v * 255.0) as u8,
                            ]),
                        )
                    });
                }
            }

            // OpenGL texture rendering
            let img_width = image.width();
            let img_height = image.height();

            // save copy of img for debugging
            {
                let copy = image.clone();
                let s = i.to_string();
                let s = format!("test_imgs/{}.png", s);
                copy.save(s).unwrap();
            }

            let raw_img = image.into_raw();
            {
                // Based on https://learnopengl.com/In-Practice/Text-Rendering

                unsafe {
                    gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
                }

                let mut texture = 0;
                unsafe {
                    // Generate texture
                    {
                        gl::GenTextures(1, &mut texture);
                        gl::BindTexture(gl::TEXTURE_2D, texture);

                        gl::TexImage2D(
                            gl::TEXTURE_2D,
                            0,
                            gl::RED as i32,
                            img_width as i32,
                            img_height as i32,
                            0,
                            gl::RED,
                            gl::UNSIGNED_BYTE,
                            raw_img.as_ptr() as *const gl::types::GLvoid,
                        );
                    }
                    // Set texture options
                    {
                        gl::TexParameteri(
                            gl::TEXTURE_2D,
                            gl::TEXTURE_WRAP_S,
                            gl::CLAMP_TO_EDGE as i32,
                        );
                        gl::TexParameteri(
                            gl::TEXTURE_2D,
                            gl::TEXTURE_WRAP_T,
                            gl::CLAMP_TO_EDGE as i32,
                        );
                        gl::TexParameteri(
                            gl::TEXTURE_2D,
                            gl::TEXTURE_MIN_FILTER,
                            gl::LINEAR as i32,
                        );
                        gl::TexParameteri(
                            gl::TEXTURE_2D,
                            gl::TEXTURE_MAG_FILTER,
                            gl::LINEAR as i32,
                        );
                    }
                }
                character.texture_id = Some(texture);
            }

            char_map.insert(c, character);
        }
    }

    return char_map;
}
