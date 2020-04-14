extern crate gl;
extern crate nalgebra as na;
use na::{Matrix4, Orthographic3, Vector2};
use std::env;

extern crate image;
use image::GenericImageView;
use image::{DynamicImage, Rgba};
use std::fs;
use std::fs::File;

use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;

pub enum Textures {
    TrollFace,
}

pub struct Texture {
    pub texture_id: u32,
}

impl Texture {
    pub fn new() -> Self {
        // Dummy stuff, for now just loading troll face
        //let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        //p.push("assets\\troll_face.png");

        //let f = File::open(p.clone()).unwrap();
        //let path = env::current_dir().unwrap();
        //println!("The current directory is {}", path.display());

        let f = fs::read("src\\assets\\troll_face.png").unwrap();

        let img = image::load_from_memory_with_format(&f, image::ImageFormat::Png).unwrap();
        let (w, h) = img.dimensions();
        {
            let clnd = img.clone();
            clnd.save("a test.png").unwrap();
        }

        let raw_img = img.into_rgba().into_raw();

        let mut texture = 0;
        unsafe {
            // Generate texture
            {
                gl::GenTextures(1, &mut texture);
                gl::BindTexture(gl::TEXTURE_2D, texture);
            }
            // Set texture options
            {
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            }
            // Bind texture
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                w as i32,
                h as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                raw_img.as_ptr() as *const gl::types::GLvoid,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        return Self {
            texture_id: texture,
        };
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.texture_id);
        }
    }
}
