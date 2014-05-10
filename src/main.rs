// Copyright 2013 The gl-rs developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![feature(globs)]

extern crate gl;
extern crate glfw;
extern crate native;

use gl::types::*;
use glfw::Context;
use std::cast;
use std::mem;
use std::ptr;

mod shaders;


static vertex_data: [GLfloat, ..288] = [
	 0.25,  0.25, -1.25, 1.0,
	 0.25, -0.25, -1.25, 1.0,
	-0.25,  0.25, -1.25, 1.0,

	 0.25, -0.25, -1.25, 1.0,
	-0.25, -0.25, -1.25, 1.0,
	-0.25,  0.25, -1.25, 1.0,

	 0.25,  0.25, -2.75, 1.0,
	-0.25,  0.25, -2.75, 1.0,
	 0.25, -0.25, -2.75, 1.0,

	 0.25, -0.25, -2.75, 1.0,
	-0.25,  0.25, -2.75, 1.0,
	-0.25, -0.25, -2.75, 1.0,

	-0.25,  0.25, -1.25, 1.0,
	-0.25, -0.25, -1.25, 1.0,
	-0.25, -0.25, -2.75, 1.0,

	-0.25,  0.25, -1.25, 1.0,
	-0.25, -0.25, -2.75, 1.0,
	-0.25,  0.25, -2.75, 1.0,

	 0.25,  0.25, -1.25, 1.0,
	 0.25, -0.25, -2.75, 1.0,
	 0.25, -0.25, -1.25, 1.0,

	 0.25,  0.25, -1.25, 1.0,
	 0.25,  0.25, -2.75, 1.0,
	 0.25, -0.25, -2.75, 1.0,

	 0.25,  0.25, -2.75, 1.0,
	 0.25,  0.25, -1.25, 1.0,
	-0.25,  0.25, -1.25, 1.0,

	 0.25,  0.25, -2.75, 1.0,
	-0.25,  0.25, -1.25, 1.0,
	-0.25,  0.25, -2.75, 1.0,

	 0.25, -0.25, -2.75, 1.0,
	-0.25, -0.25, -1.25, 1.0,
	 0.25, -0.25, -1.25, 1.0,

	 0.25, -0.25, -2.75, 1.0,
	-0.25, -0.25, -2.75, 1.0,
	-0.25, -0.25, -1.25, 1.0,




	0.0, 0.0, 1.0, 1.0,
	0.0, 0.0, 1.0, 1.0,
	0.0, 0.0, 1.0, 1.0,

	0.0, 0.0, 1.0, 1.0,
	0.0, 0.0, 1.0, 1.0,
	0.0, 0.0, 1.0, 1.0,

	0.8, 0.8, 0.8, 1.0,
	0.8, 0.8, 0.8, 1.0,
	0.8, 0.8, 0.8, 1.0,

	0.8, 0.8, 0.8, 1.0,
	0.8, 0.8, 0.8, 1.0,
	0.8, 0.8, 0.8, 1.0,

	0.0, 1.0, 0.0, 1.0,
	0.0, 1.0, 0.0, 1.0,
	0.0, 1.0, 0.0, 1.0,

	0.0, 1.0, 0.0, 1.0,
	0.0, 1.0, 0.0, 1.0,
	0.0, 1.0, 0.0, 1.0,

	0.5, 0.5, 0.0, 1.0,
	0.5, 0.5, 0.0, 1.0,
	0.5, 0.5, 0.0, 1.0,

	0.5, 0.5, 0.0, 1.0,
	0.5, 0.5, 0.0, 1.0,
	0.5, 0.5, 0.0, 1.0,

	1.0, 0.0, 0.0, 1.0,
	1.0, 0.0, 0.0, 1.0,
	1.0, 0.0, 0.0, 1.0,

	1.0, 0.0, 0.0, 1.0,
	1.0, 0.0, 0.0, 1.0,
	1.0, 0.0, 0.0, 1.0,

	0.0, 1.0, 1.0, 1.0,
	0.0, 1.0, 1.0, 1.0,
	0.0, 1.0, 1.0, 1.0,

	0.0, 1.0, 1.0, 1.0,
	0.0, 1.0, 1.0, 1.0,
	0.0, 1.0, 1.0, 1.0,

];

fn init_program() -> GLuint {
	println!("== init program ==");
	let mut shader_list = Vec::new();
	shader_list.push(shaders::load_shader_file(gl::VERTEX_SHADER, "s/ManualPerspective.vert"));
    shader_list.push(shaders::load_shader_file(gl::FRAGMENT_SHADER, "s/StandardColors.frag"));
    let mut program = shaders::create_program(&shader_list);

	for s in shader_list.iter() {
		gl::DeleteShader(*s);
	}
	program
}


fn init_vertex_buffer() -> GLuint {
	let mut vbo: GLuint = 0;
	unsafe {
		gl::GenBuffers(1, &mut vbo);

		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
		gl::BufferData(gl::ARRAY_BUFFER, (vertex_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, cast::transmute(&vertex_data[0]), gl::STATIC_DRAW);
		gl::BindBuffer(gl::ARRAY_BUFFER, 0);
	}
	vbo
}

fn init() -> (GLuint, GLuint, GLuint, GLint) {
	let mut program =init_program();
	let mut vbo = init_vertex_buffer();
	let mut vao = 0;

	unsafe{
		gl::GenVertexArrays(1, &mut vao);
	}
	gl::BindVertexArray(vao);

	gl::Enable(gl::CULL_FACE);
	gl::CullFace(gl::BACK);
	gl::FrontFace(gl::CW);

	let mut offset_unif = 0;
	let mut frustum_scale_unif = 0;
	let mut znear_unif = 0;
	let mut zfar_unif = 0;
	gl::UseProgram(program);
	unsafe {
		offset_unif = gl::GetUniformLocation(program, "offset".with_c_str(|ptr| ptr));
		frustum_scale_unif = gl::GetUniformLocation(program, "frustumScale".with_c_str(|ptr| ptr));
		znear_unif = gl::GetUniformLocation(program, "zNear".with_c_str(|ptr| ptr));
		zfar_unif = gl::GetUniformLocation(program, "zFar".with_c_str(|ptr| ptr));
		//offset_unif = "offset".with_c_str(|ptr| gl::GetUniformLocation(program, ptr));
	}
	gl::Uniform1f(frustum_scale_unif, 1.0);
	gl::Uniform1f(znear_unif, 1.0);
	gl::Uniform1f(zfar_unif, 3.0);
	gl::UseProgram(0);

	(program, vbo, vao, offset_unif)
}

fn display(program: GLuint, vbo: GLuint, offset_unif: GLint) {
	gl::ClearColor(0.0, 0.0, 0.0, 0.0);
	gl::Clear(gl::COLOR_BUFFER_BIT);

	gl::UseProgram(program);
	gl::Uniform2f(offset_unif, 0.5, 0.5);

	let color_data = vertex_data.len() * mem::size_of::<GLfloat>() / 2;
	gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
	gl::EnableVertexAttribArray(0);
	gl::EnableVertexAttribArray(1);
	unsafe{
		gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, 0, ptr::null());
		gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 0, cast::transmute(color_data));
	}

	gl::DrawArrays(gl::TRIANGLES, 0, 36);

	gl::DisableVertexAttribArray(0);
	gl::DisableVertexAttribArray(1);
	gl::UseProgram(0);
}

fn main() {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Choose a GL profile that is compatible with OS X 10.7+ -- from example code
    glfw.window_hint(glfw::ContextVersion(3, 2));
    glfw.window_hint(glfw::OpenglForwardCompat(true));
    glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));
	glfw.window_hint(glfw::Resizable(false));

    let (window, events) = glfw.create_window(600, 600, "OpenGL", glfw::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current(); //make context current before calling gl::load_with
    window.set_key_polling(true); //enable internal polling function

	window.set_size(600, 600);
    gl::load_with(|s| glfw.get_proc_address(s)); //loading opengl function pointers

	let (mut program, mut vbo, mut vao, mut offset_unif) = init();


    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&window, event);
        }

		display(program, vbo, offset_unif);
		window.swap_buffers();
    }

    // Cleanup
    gl::DeleteProgram(program);
    unsafe {
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &vao);
    }
}

#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}

fn handle_window_event(window: &glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => {
            window.set_should_close(true)
        }
        glfw::KeyEvent(glfw::KeyQ, _, glfw::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}

// fn reshape(w: int, h: int) {
// 	gl::Viewport(0, 0, w as GLsizei, h as GLsizei);
// }


