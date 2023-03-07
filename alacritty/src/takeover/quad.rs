use crate::gl::{self, types::*};
use std::mem::size_of;

#[derive(Debug)]
struct Attribute {
    index: GLuint,
    elem_size: GLuint,
    elem_count: GLuint,
    kind: GLenum,
}

#[derive(Debug)]
pub struct Quad {
    pub gl_vao: GLuint,
    gl_vbo: GLuint,
    gl_ebo: GLuint,
    attributes: Vec<Attribute>,
    index_count: i32,
}

impl Quad {
    pub fn new<T, F>(vertices: &[T], indices: &[u32], add_attrs: F) -> Self
        where
            F: FnOnce(&mut Self) -> (),
    {
        let mut quad = Self {
            gl_vao: 0,
            gl_vbo: 0,
            gl_ebo: 0,
            attributes: Vec::new(),
            index_count: indices.len() as i32,
        };

        unsafe {
            // Create and bind Vertex Arrays.
            gl::GenVertexArrays(1, &mut quad.gl_vao);
            gl::BindVertexArray(quad.gl_vao);

            // Create and bind Vertex Buffer.
            gl::GenBuffers(1, &mut quad.gl_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, quad.gl_vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * size_of::<T>()) as isize,
                vertices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            add_attrs(&mut quad);
            quad.enable_attrs();

            // Create and bind Element Buffer.
            gl::GenBuffers(1, &mut quad.gl_ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, quad.gl_ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * size_of::<u32>()) as isize,
                indices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            // Cleanup.
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }

        quad
    }

    pub fn attr<T>(&mut self, index: GLuint, elem_count: GLuint, kind: GLenum) -> &mut Self {
        self.attributes.push(Attribute {
            index,
            kind,
            elem_count,
            elem_size: size_of::<T>() as u32,
        });

        self
    }

    /// Issue calls to OpenGL enabling vertex attributes.
    /// Make sure that ARRAY_BUFFER is bound.
    fn enable_attrs(&self) -> &Self {
        let mut offset = 0;
        let stride = self.get_stride();

        for attr in self.attributes.iter() {
            unsafe {
                gl::VertexAttribPointer(
                    attr.index,
                    attr.elem_size as i32,
                    attr.kind,
                    gl::FALSE,
                    stride as i32,
                    offset as *const _,
                );
                gl::EnableVertexAttribArray(attr.index);
            }

            offset += attr.elem_size * attr.elem_count;
        }

        self
    }

    fn bind(&self) -> &Self {
        unsafe {
            gl::BindVertexArray(self.gl_vao);
        }

        self
    }

    fn unbind(&self) -> &Self {
        unsafe {
            gl::BindVertexArray(0);
        }

        self
    }

    pub fn draw(&self) -> &Self {
        self.bind();
        unsafe {
            gl::DrawElements(gl::TRIANGLES, self.index_count, gl::UNSIGNED_INT, std::ptr::null());
        }
        self.unbind();

        self
    }

    fn get_stride(&self) -> u32 {
        self.attributes.iter().fold(0, |acc, attr| {
            acc + attr.elem_size * attr.elem_count
        })
    }
}

impl Drop for Quad {
    fn drop(&mut self) -> () {
        unsafe {
            gl::DeleteBuffers(1, &self.gl_vbo);
            gl::DeleteBuffers(1, &self.gl_ebo);
            gl::DeleteVertexArrays(1, &self.gl_vao);
        }
    }
}
