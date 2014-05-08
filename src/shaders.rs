extern crate gl;
extern crate glfw;

use gl::types::*;
use std::ptr;
use std::str;

//TODO: write file load logic using http://static.rust-lang.org/doc/master/std/io/fs/struct.File.html
pub fn load_shader(src: &str, ty: GLenum) -> GLuint {
	let shader = gl::CreateShader(ty);
	unsafe {
		// Attempt to compile the shader
		src.with_c_str(|ptr| gl::ShaderSource(shader, 1, &ptr, ptr::null()));
		gl::CompileShader(shader);

		// Get the compile status
		let mut status = gl::FALSE as GLint;
		gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

		// Fail on error
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

pub fn create_program(vs: GLuint, fs: GLuint) -> GLuint {
	let program = gl::CreateProgram();
	gl::AttachShader(program, vs);
	gl::AttachShader(program, fs);
	gl::LinkProgram(program);
	unsafe {
		// Get the link status
		let mut status = gl::FALSE as GLint;
		gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

		// Fail on error
		if status != (gl::TRUE as GLint) {
			let mut len: GLint = 0;
			gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
			let mut buf = Vec::from_elem(len as uint - 1, 0u8);     // subtract 1 to skip the trailing null character
			gl::GetProgramInfoLog(program, len, ptr::mut_null(), buf.as_mut_ptr() as *mut GLchar);
			fail!("{}", str::from_utf8(buf.as_slice()).expect("ProgramInfoLog not valid utf8"));
		}
	}
	program
}
