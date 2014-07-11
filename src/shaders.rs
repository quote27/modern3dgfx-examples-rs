extern crate gl;
extern crate glfw;

use gl::types::*;
use std::ptr;
use std::str;


pub fn load_file(file_src: &str) -> String {
	use std::io::File;

	let path = Path::new(file_src);
	let mut f = File::open(&path);

	let dat = f.read_to_string().ok().expect(format!("unable to open {} file", file_src).as_slice());

	println!("reading {0} file:\n{1}\n---", file_src, dat);

	dat
}

pub fn load_shader_file(ty: GLenum, file_src: &str) -> GLuint {
	load_shader(ty, load_file(file_src))
}

//TODO: write file load logic using http://static.rust-lang.org/doc/master/std/io/fs/struct.File.html
pub fn load_shader(ty: GLenum, src: String) -> GLuint {
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

pub fn create_program(shader_list: &Vec<GLuint>) -> GLuint {
	let prog = gl::CreateProgram();
	for s in shader_list.iter() {
		gl::AttachShader(prog, *s);
	}
	gl::LinkProgram(prog);
	unsafe {
		// Get the link status
		let mut status = gl::FALSE as GLint;
		gl::GetProgramiv(prog, gl::LINK_STATUS, &mut status);

		// Fail on error
		if status != (gl::TRUE as GLint) {
			let mut len: GLint = 0;
			gl::GetProgramiv(prog, gl::INFO_LOG_LENGTH, &mut len);
			let mut buf = Vec::from_elem(len as uint - 1, 0u8);     // subtract 1 to skip the trailing null character
			gl::GetProgramInfoLog(prog, len, ptr::mut_null(), buf.as_mut_ptr() as *mut GLchar);
			fail!("{}", str::from_utf8(buf.as_slice()).expect("ProgramInfoLog not valid utf8"));
		}
	}
	prog
}
