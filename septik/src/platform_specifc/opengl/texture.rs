use std::ffi::{CStr, CString};

extern crate gl;

use crate::platform_specifc::file_io::assets_path;
use crate::platform_specifc::opengl::{helpers::*, shaders::*, vertices::*};

extern crate image;

use image::{GenericImageView, ImageFormat};
use std::path::Path;

pub struct Texture {
    id: gl::types::GLuint,
    width: u32,
    height: u32,
}

impl Texture {
    pub fn new(file_name: String) -> Self {
        // Loading of image
        let i = image::open(assets_path(file_name)).unwrap();

        let texture_w = i.width();
        let texture_h = i.height();

        let mut texId: gl::types::GLuint = 0;
        unsafe {
            // Bind id
            gl::GenTextures(1, &mut texId);
        }

        let mut tex = Self {
            id: texId,
            width: texture_w,
            height: texture_h,
        };

        tex.bind();

        //https://learnopengl.com/Getting-started/Textures
        unsafe {
            let img_bytes: *const gl::types::GLvoid =
                i.to_bytes().as_ptr() as *const gl::types::GLvoid;

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                texture_w as i32,
                texture_h as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img_bytes,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::TextureParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TextureParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        }

        tex
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn bind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}

//TODO: what about dropping?
