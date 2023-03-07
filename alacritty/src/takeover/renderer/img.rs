use image::io::Reader;
use image::Rgba32FImage;
use crate::display::Display;
use crate::gl::{self, types::GLint};
use winit::dpi::PhysicalSize;

use super::super::{
    TakeoverRenderer,
    shader::Program,
    quad::Quad,
    texture::Texture2D,
};

// Base Scale: x: f32, y: f32
// Zoom: f32
// Translation: x: f32, y: f32

const VERTEX_POSITIONS: [f32; 16] = [
    // Vertex position
    //           Texture position
    1.0,  1.0,   1.0,  1.0,
    1.0, -1.0,   1.0,  0.0,
   -1.0, -1.0,   0.0,  0.0,
   -1.0,  1.0,   0.0,  1.0,
];

const VERTEX_INDICES: [u32; 6] = [
    0, 2, 1,
    0, 3, 2,
];

const VERT_SOURCE: &'static str = include_str!("../../../res/takeover/image.vert");
const FRAG_SOURCE: &'static str = include_str!("../../../res/takeover/image.frag");

/// Active texture for this takeover renderer.
/// Note: it should not interfere with default texture used by Alacritty text renderer.
const TEXTURE_SLOT: u32 = gl::TEXTURE6;

/// Stores uniform locations required by Takeover Image Renderer.
#[derive(Debug)]
struct Uniforms {
    /// Location of the texture sampler uniform.
    pub u_texture: GLint,
    /// Location of the image scale uniform.
    pub u_img_scale: GLint,
}

/// Takeover renderer reponsible for rendering fullscreen images.
#[derive(Debug)]
pub struct ImageRenderer {
    program: Program,
    uniforms: Uniforms,
    quad: Quad,
    texture: Texture2D,
    img_buf: Rgba32FImage,
}

impl TakeoverRenderer for ImageRenderer {
    fn render(&self) {
        self.texture.bind();
        self.program.bind();

        self.quad.draw();

        self.program.unbind();
        self.texture.unbind();
    }

    fn resize(&mut self, size: &PhysicalSize<u32>) {
        let img_size = self.img_buf.dimensions();
        let img_scale = Self::calculate_img_scale(
            img_size.0 as i32 as f32,
            img_size.1 as i32 as f32,
            size.width as i32 as f32,
            size.height as i32 as f32,
        );
        unsafe {
            gl::UseProgram(self.program.gl_id);
            gl::Uniform2f(self.uniforms.u_img_scale, img_scale[0], img_scale[1]);
            gl::UseProgram(0);
        }
    }
}

impl ImageRenderer {
    pub fn create(path: std::path::PathBuf, display: &Display) -> Result<Self, String> {
        // Load image first - there is no point doing any other work beforehand if this fails.
        let image = Reader::open(&path)
            .map_err(|_| format!("Cannot open file at {:?}.", &path))?
            .decode()
            .map_err(|_| format!("Cannot decode file at {:?}.", &path))?
            .flipv()
            .into_rgba32f();

        // Create program and it's uniform locations.
        let program = Program::create(VERT_SOURCE, FRAG_SOURCE)?;
        let uniforms = Uniforms {
            u_texture: program.get_uniform_location("u_texture\0")?,
            u_img_scale: program.get_uniform_location("u_img_scale\0")?,
        };

        // Create Quad and enable vertex attributes.
        let quad = Quad::new(
            &VERTEX_POSITIONS,
            &VERTEX_INDICES,
            |quad| {
                quad
                    .attr::<f32>(0, 2, gl::FLOAT)
                    .attr::<f32>(1, 2, gl::FLOAT);
            }
        );

        // Calculate image and window aspect ratios.
        let img_size = image.dimensions();
        let win_size = &display.size_info;
        let img_scale = Self::calculate_img_scale(
            img_size.0 as i32 as f32,
            img_size.1 as i32 as f32,
            win_size.width(),
            win_size.height(),
        );

        unsafe {
            // Update values of the uniforms.
            gl::UseProgram(program.gl_id);
            gl::Uniform1i(uniforms.u_texture, (TEXTURE_SLOT - gl::TEXTURE0) as i32);
            gl::Uniform2f(uniforms.u_img_scale, img_scale[0], img_scale[1]);
            gl::UseProgram(0);
        }

        // Create texture.
        let texture = Texture2D::new(&image, TEXTURE_SLOT);

        return Ok(Self {
            program,
            uniforms,
            quad,
            texture,
            img_buf: image,
        });
    }

    fn calculate_img_scale(
        image_width: f32,
        image_height: f32,
        window_width: f32,
        window_height: f32,
    ) -> [f32; 2] {
        let img_aspect = image_width / image_height;
        let win_aspect = window_width / window_height;

        if img_aspect > win_aspect {
            [1.0, win_aspect / img_aspect]
        } else {
            [img_aspect / win_aspect, 1.0]
        }
    }
}
