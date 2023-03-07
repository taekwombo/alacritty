use crate::gl::{self, types::*};
use image::{Rgba, ImageBuffer};

#[derive(Debug)]
pub struct Texture2D {
    gl_id: GLuint,
    slot: GLenum,
}

impl Texture2D {
    pub fn new(image: &ImageBuffer<Rgba<f32>, Vec<f32>>, slot: GLenum) -> Self {
        let mut gl_id: GLuint = 0;
        let (width, height) = image.dimensions();

        // Never use the glyphs slot.
        assert!(slot != gl::TEXTURE0);

        unsafe {
            gl::ActiveTexture(slot);
            gl::GenTextures(1, &mut gl_id);
            gl::BindTexture(gl::TEXTURE_2D, gl_id);

            // Set some parameters for the texture.
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_BORDER as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_BORDER as i32);

            // Upload texture.
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA32F as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::FLOAT,
                image.as_raw().as_ptr() as *const _,
            );

            // Cleanup.
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::ActiveTexture(gl::TEXTURE0);
        }

        Self {
            gl_id,
            slot,
        }
    }

    pub fn bind(&self) -> &Self {
        unsafe {
            gl::ActiveTexture(self.slot);
            gl::BindTexture(gl::TEXTURE_2D, self.gl_id);
        }

        self
    }

    pub fn unbind(&self) -> &Self {
        unsafe {
            // First unbind current texture.
            // Then change active texture slot to default.
            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl::ActiveTexture(gl::TEXTURE0);
        }

        self
    }
}

impl Drop for Texture2D {
    fn drop(&mut self) -> () {
        unsafe {
            gl::DeleteTextures(1, &self.gl_id);
        }
    }
}
