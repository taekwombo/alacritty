use crate::gl::{self, types::*};

#[derive(Debug)]
pub struct Program {
    pub gl_id: GLuint,
}

impl Program {
    pub fn create(vert: &'static str, frag: &'static str) -> Result<Self, String> {
        let vertex_shader = Shader::create(gl::VERTEX_SHADER, vert)?;
        let fragment_shader = Shader::create(gl::FRAGMENT_SHADER, frag)?;
        let program = Self {
            gl_id: unsafe { gl::CreateProgram() },
        };

        let mut success: GLint = 0;
        unsafe {
            gl::AttachShader(program.gl_id, vertex_shader.gl_id);
            gl::AttachShader(program.gl_id, fragment_shader.gl_id);
            gl::LinkProgram(program.gl_id);
            gl::GetProgramiv(program.gl_id, gl::LINK_STATUS, &mut success);
        }

        if success == gl::TRUE as i32 {
            Ok(program)
        } else {
            Err(Self::get_info_log(program.gl_id))
        }
    }

    pub fn get_uniform_location(&self, name: &str) -> Result<GLint, String> {
        debug_assert!(&name[name.len() - 1..] == "\0");

        let location = unsafe { gl::GetUniformLocation(self.gl_id, name.as_ptr().cast() ) };

        if location == -1 {
            Err(format!("Could not find uniform location of '{name}'."))
        } else {
            Ok(location)
        }
    }

    pub fn bind(&self) -> &Self {
        unsafe {
            gl::UseProgram(self.gl_id);
        }

        self
    }

    pub fn unbind(&self) -> &Self {
        unsafe {
            gl::UseProgram(0);
        }

        self
    }

    fn get_info_log(program_id: GLuint) -> String {
        let mut info_max_length: GLint = 0;
        unsafe {
            gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut info_max_length);
        }

        let mut written: GLint = 0;
        let mut buf: Vec<u8> = Vec::with_capacity(info_max_length as usize);
        unsafe {
            gl::GetProgramInfoLog(program_id, info_max_length, &mut written, buf.as_mut_ptr().cast());
        }

        unsafe {
            buf.set_len(written as usize);
        }

        String::from_utf8_lossy(&buf).to_string()
    }
}

impl Drop for Program {
    fn drop(&mut self) -> () {
        unsafe {
            gl::DeleteProgram(self.gl_id);
        }
    }
}

#[derive(Debug)]
struct Shader {
    gl_id: GLuint,
}

impl Shader {
    pub fn create(kind: GLenum, source: &'static str) -> Result<Self, String> {
        let shader = Self {
            gl_id: unsafe { gl::CreateShader(kind) },
        };

        let src_ptr = source.as_ptr().cast();
        let mut success: GLint = 0;

        unsafe {
            gl::ShaderSource(
                shader.gl_id,
                1,
                &src_ptr,
                &(source.len() as i32),
            );
            gl::CompileShader(shader.gl_id);
            gl::GetShaderiv(shader.gl_id, gl::COMPILE_STATUS, &mut success);
        }

        if success == gl::TRUE as i32 {
            Ok(shader)
        } else {
            Err(Self::get_info_log(shader.gl_id))
        }
    }

    fn get_info_log(shader_id: GLuint) -> String {
        let mut info_max_length: GLint = 0;
        unsafe {
            gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut info_max_length);
        }

        let mut written: GLint = 0;
        let mut buf: Vec<u8> = Vec::with_capacity(info_max_length as usize);
        unsafe {
            gl::GetShaderInfoLog(shader_id, info_max_length, &mut written, buf.as_mut_ptr().cast());
        }

        unsafe {
            buf.set_len(written as usize);
        }

        String::from_utf8_lossy(&buf).to_string()
    }
}

impl Drop for Shader {
    fn drop(&mut self) -> () {
        unsafe {
            gl::DeleteShader(self.gl_id);
        }
    }
}

