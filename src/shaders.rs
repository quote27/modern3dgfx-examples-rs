extern crate gl;
extern crate glfw;

use gl::types::*;
use std::ptr;
use std::str;
use std::mem;
use std::fmt::{Show, Formatter, FormatError};
use cgmath::matrix::Matrix4;
use cgmath::vector::Vector4;

use util;

fn load_file(file_src: &str) -> String {
    use std::io::File;

    let path = Path::new(file_src);
    let mut f = File::open(&path);
    let dat = f.read_to_string().ok().expect(format!("unable to open {} file", file_src).as_slice());

    println!("reading {0} file:\n{1}\n---", file_src, dat);
    dat
}


pub struct Shader {
    file_name: String,
    shader_str: String,
    ty: GLenum,
    s: GLuint
}

impl Shader {
    pub fn from_file(ty: GLenum, file_src: &str) -> Shader {
        let sh = load_file(file_src);
        let s = Shader::load_shader(ty, sh.clone());
        Shader {
            file_name: String::from_str(file_src),
            shader_str: sh,
            ty: ty,
            s: s,
        }
    }

    //TODO: write file load logic using http://static.rust-lang.org/doc/master/std/io/fs/struct.File.html
    fn load_shader(ty: GLenum, src: String) -> GLuint {
        let shader = gl::CreateShader(ty);
        unsafe {
            // attempt to compile the shader
            src.with_c_str(|ptr| gl::ShaderSource(shader, 1, &ptr, ptr::null()));
            gl::CompileShader(shader);

            // get the compile status
            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

            // fail on error
            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::from_elem(len as uint - 1, 0u8);     // subtract 1 to skip the trailing null character
                gl::GetShaderInfoLog(shader, len, ptr::mut_null(), buf.as_mut_ptr() as *mut GLchar);
                fail!("{}", str::from_utf8(buf.as_slice()).expect("ShaderInfoLog not valid utf8"));
            }
        }
        shader
    }
}

impl Show for Shader {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        f.write(format!("file: {}, type: {}, s: {}", self.file_name, util::glenum_name(self.ty), self.s).as_bytes())
    }
}


pub struct Program { p: GLuint }

impl Program {
    pub fn new(shader_list: &Vec<Shader>) -> Program {
        let prog = gl::CreateProgram();
        for s in shader_list.iter() {
            gl::AttachShader(prog, s.s);
        }
        gl::LinkProgram(prog);
        unsafe {
            // get the link status
            let mut status = gl::FALSE as GLint;
            gl::GetProgramiv(prog, gl::LINK_STATUS, &mut status);

            // fail on error
            if status != (gl::TRUE as GLint) {
                let mut len: GLint = 0;
                gl::GetProgramiv(prog, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::from_elem(len as uint - 1, 0u8);     // subtract 1 to skip the trailing null character
                gl::GetProgramInfoLog(prog, len, ptr::mut_null(), buf.as_mut_ptr() as *mut GLchar);
                fail!("{}", str::from_utf8(buf.as_slice()).expect("ProgramInfoLog not valid utf8"));
            }
        }
        Program { p: prog }
    }

    #[inline(always)]
    pub fn get_unif(&self, name: &str) -> Uniform {
        let u = unsafe { gl::GetUniformLocation(self.p, name.with_c_str(|ptr| ptr)) };
        Uniform { u: u }
    }

    #[inline(always)]
    pub fn get_attrib(&self, name: &str) -> GLuint {
        unsafe { gl::GetAttribLocation(self.p, name.with_c_str(|ptr| ptr)) as GLuint }
    }

    #[inline(always)]
    pub fn use_prog(&self) {
        gl::UseProgram(self.p);
    }

    #[inline(always)]
    pub fn delete(&mut self) {
        gl::DeleteProgram(self.p);
    }
}

impl Show for Program {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        f.write(format!("{}", self.p).as_bytes())
    }
}


pub struct Uniform { u: GLint }

impl Uniform {
    pub fn upload_m4f(&self, m: &Matrix4<f32>) {
        unsafe { gl::UniformMatrix4fv(self.u, 1, gl::FALSE, mem::transmute(m)); }
    }

    pub fn upload_v4f(&self, v: &Vector4<f32>) {
        unsafe { gl::Uniform4fv(self.u, 4, mem::transmute(v)); }
    }
}

impl Show for Uniform {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        f.write(format!("{}", self.u).as_bytes())
    }
}

