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

static RIGHT_EXTENT: GLfloat = 0.8;
static LEFT_EXTENT: GLfloat = -0.8;//-RIGHT_EXTENT;
static TOP_EXTENT: GLfloat = 0.20;
static MIDDLE_EXTENT: GLfloat = 0.0;
static BOTTOM_EXTENT: GLfloat = -0.20;//-TOP_EXTENT;
static FRONT_EXTENT: GLfloat = -1.25;
static REAR_EXTENT: GLfloat = -1.75;

static GREEN_COLOR:[GLfloat, ..4]  = [0.75, 0.75, 1.0, 1.0];
static BLUE_COLOR: [GLfloat, ..4]  = [0.0, 0.5, 0.0, 1.0];
static RED_COLOR:  [GLfloat, ..4]  = [1.0, 0.0, 0.0, 1.0];
static GREY_COLOR: [GLfloat, ..4]  = [0.8, 0.8, 0.8, 1.0];
static BROWN_COLOR:[GLfloat, ..4]  = [0.5, 0.5, 0.0, 1.0];

static vertex_num: uint = 36;
static vertex_data:[GLfloat, ..252] = [
	//Object 1 positions
	LEFT_EXTENT,	TOP_EXTENT,		REAR_EXTENT,
	LEFT_EXTENT,	MIDDLE_EXTENT,	FRONT_EXTENT,
	RIGHT_EXTENT,	MIDDLE_EXTENT,	FRONT_EXTENT,
	RIGHT_EXTENT,	TOP_EXTENT,		REAR_EXTENT, //12

	LEFT_EXTENT,	BOTTOM_EXTENT,	REAR_EXTENT,
	LEFT_EXTENT,	MIDDLE_EXTENT,	FRONT_EXTENT,
	RIGHT_EXTENT,	MIDDLE_EXTENT,	FRONT_EXTENT,
	RIGHT_EXTENT,	BOTTOM_EXTENT,	REAR_EXTENT, //12

	LEFT_EXTENT,	TOP_EXTENT,		REAR_EXTENT,
	LEFT_EXTENT,	MIDDLE_EXTENT,	FRONT_EXTENT,
	LEFT_EXTENT,	BOTTOM_EXTENT,	REAR_EXTENT, //9

	RIGHT_EXTENT,	TOP_EXTENT,		REAR_EXTENT,
	RIGHT_EXTENT,	MIDDLE_EXTENT,	FRONT_EXTENT,
	RIGHT_EXTENT,	BOTTOM_EXTENT,	REAR_EXTENT, //9

	LEFT_EXTENT,	BOTTOM_EXTENT,	REAR_EXTENT,
	LEFT_EXTENT,	TOP_EXTENT,		REAR_EXTENT,
	RIGHT_EXTENT,	TOP_EXTENT,		REAR_EXTENT,
	RIGHT_EXTENT,	BOTTOM_EXTENT,	REAR_EXTENT, //12

	//Object 2 positions
	TOP_EXTENT,		RIGHT_EXTENT,	REAR_EXTENT,
	MIDDLE_EXTENT,	RIGHT_EXTENT,	FRONT_EXTENT,
	MIDDLE_EXTENT,	LEFT_EXTENT,	FRONT_EXTENT,
	TOP_EXTENT,		LEFT_EXTENT,	REAR_EXTENT, //12

	BOTTOM_EXTENT,	RIGHT_EXTENT,	REAR_EXTENT,
	MIDDLE_EXTENT,	RIGHT_EXTENT,	FRONT_EXTENT,
	MIDDLE_EXTENT,	LEFT_EXTENT,	FRONT_EXTENT,
	BOTTOM_EXTENT,	LEFT_EXTENT,	REAR_EXTENT, //12

	TOP_EXTENT,		RIGHT_EXTENT,	REAR_EXTENT,
	MIDDLE_EXTENT,	RIGHT_EXTENT,	FRONT_EXTENT,
	BOTTOM_EXTENT,	RIGHT_EXTENT,	REAR_EXTENT, //9
					
	TOP_EXTENT,		LEFT_EXTENT,	REAR_EXTENT,
	MIDDLE_EXTENT,	LEFT_EXTENT,	FRONT_EXTENT,
	BOTTOM_EXTENT,	LEFT_EXTENT,	REAR_EXTENT, //9
					
	BOTTOM_EXTENT,	RIGHT_EXTENT,	REAR_EXTENT,
	TOP_EXTENT,		RIGHT_EXTENT,	REAR_EXTENT,
	TOP_EXTENT,		LEFT_EXTENT,	REAR_EXTENT,
	BOTTOM_EXTENT,	LEFT_EXTENT,	REAR_EXTENT, //12

	//Object 1 colors
	GREEN_COLOR[0], GREEN_COLOR[1], GREEN_COLOR[2], GREEN_COLOR[3],
	GREEN_COLOR[0], GREEN_COLOR[1], GREEN_COLOR[2], GREEN_COLOR[3],
	GREEN_COLOR[0], GREEN_COLOR[1], GREEN_COLOR[2], GREEN_COLOR[3],
	GREEN_COLOR[0], GREEN_COLOR[1], GREEN_COLOR[2], GREEN_COLOR[3], //12

	BLUE_COLOR[0], BLUE_COLOR[1], BLUE_COLOR[2], BLUE_COLOR[3],
	BLUE_COLOR[0], BLUE_COLOR[1], BLUE_COLOR[2], BLUE_COLOR[3],
	BLUE_COLOR[0], BLUE_COLOR[1], BLUE_COLOR[2], BLUE_COLOR[3],
	BLUE_COLOR[0], BLUE_COLOR[1], BLUE_COLOR[2], BLUE_COLOR[3], //12

	RED_COLOR[0], RED_COLOR[1], RED_COLOR[2], RED_COLOR[3],
	RED_COLOR[0], RED_COLOR[1], RED_COLOR[2], RED_COLOR[3],
	RED_COLOR[0], RED_COLOR[1], RED_COLOR[2], RED_COLOR[3], //9

	GREY_COLOR[0], GREY_COLOR[1], GREY_COLOR[2], GREY_COLOR[3],
	GREY_COLOR[0], GREY_COLOR[1], GREY_COLOR[2], GREY_COLOR[3],
	GREY_COLOR[0], GREY_COLOR[1], GREY_COLOR[2], GREY_COLOR[3], //9

	BROWN_COLOR[0], BROWN_COLOR[1], BROWN_COLOR[2], BROWN_COLOR[3],
	BROWN_COLOR[0], BROWN_COLOR[1], BROWN_COLOR[2], BROWN_COLOR[3],
	BROWN_COLOR[0], BROWN_COLOR[1], BROWN_COLOR[2], BROWN_COLOR[3],
	BROWN_COLOR[0], BROWN_COLOR[1], BROWN_COLOR[2], BROWN_COLOR[3], //12

	//Object 2 colors
	RED_COLOR[0], RED_COLOR[1], RED_COLOR[2], RED_COLOR[3],
	RED_COLOR[0], RED_COLOR[1], RED_COLOR[2], RED_COLOR[3],
	RED_COLOR[0], RED_COLOR[1], RED_COLOR[2], RED_COLOR[3],
	RED_COLOR[0], RED_COLOR[1], RED_COLOR[2], RED_COLOR[3], //12

	BROWN_COLOR[0], BROWN_COLOR[1], BROWN_COLOR[2], BROWN_COLOR[3],
	BROWN_COLOR[0], BROWN_COLOR[1], BROWN_COLOR[2], BROWN_COLOR[3],
	BROWN_COLOR[0], BROWN_COLOR[1], BROWN_COLOR[2], BROWN_COLOR[3],
	BROWN_COLOR[0], BROWN_COLOR[1], BROWN_COLOR[2], BROWN_COLOR[3], //12

	BLUE_COLOR[0], BLUE_COLOR[1], BLUE_COLOR[2], BLUE_COLOR[3],
	BLUE_COLOR[0], BLUE_COLOR[1], BLUE_COLOR[2], BLUE_COLOR[3],
	BLUE_COLOR[0], BLUE_COLOR[1], BLUE_COLOR[2], BLUE_COLOR[3], //9

	GREEN_COLOR[0], GREEN_COLOR[1], GREEN_COLOR[2], GREEN_COLOR[3],
	GREEN_COLOR[0], GREEN_COLOR[1], GREEN_COLOR[2], GREEN_COLOR[3],
	GREEN_COLOR[0], GREEN_COLOR[1], GREEN_COLOR[2], GREEN_COLOR[3], //9

	GREY_COLOR[0], GREY_COLOR[1], GREY_COLOR[2], GREY_COLOR[3],
	GREY_COLOR[0], GREY_COLOR[1], GREY_COLOR[2], GREY_COLOR[3],
	GREY_COLOR[0], GREY_COLOR[1], GREY_COLOR[2], GREY_COLOR[3],
	GREY_COLOR[0], GREY_COLOR[1], GREY_COLOR[2], GREY_COLOR[3], //12
];

static index_data: [GLshort, ..24] = [
	0, 2, 1,
	3, 2, 0,

	4, 5, 6,
	6, 7, 4,

	8, 9, 10,
	11, 13, 12,

	14, 16, 15,
	17, 16, 14,
];

static frustum_scale: GLfloat = 1.0;

fn get_uniform(program: GLuint, name: &str) -> GLint {
	unsafe { gl::GetUniformLocation(program, name.with_c_str(|ptr| ptr)) }
	//alternative: unsafe { name.with_c_str(|ptr| gl::GetUniformLocation(program, ptr)); }
}

fn set_perspective_mat(program: GLuint, s0: GLfloat, s5: GLfloat) {
	let (znear, zfar) = (1.0, 3.0);

	let pm = [ //column major order
		s0, 0.0, 0.0, 0.0,
		0.0, s5, 0.0, 0.0,
		0.0, 0.0, (zfar + znear) / (znear - zfar), -1.0,
		0.0, 0.0, (2.0 * zfar * znear) / (znear - zfar), 0.0,
	];
	let pm_unif = get_uniform(program, "perspectiveMatrix");
	gl::UseProgram(program);
	unsafe {
		gl::UniformMatrix4fv(pm_unif, 1, gl::FALSE, &pm[0]);
	}
	gl::UseProgram(0);
}

fn init_program() -> GLuint {
	println!("== init program ==");
	let mut shader_list = Vec::new();
	shader_list.push(shaders::load_shader_file(gl::VERTEX_SHADER, "s/Standard.vert"));
    shader_list.push(shaders::load_shader_file(gl::FRAGMENT_SHADER, "s/Standard.frag"));
    let program = shaders::create_program(&shader_list);

	for s in shader_list.iter() {
		gl::DeleteShader(*s);
	}
	program
}

fn init_vertex_buffer() -> (GLuint,GLuint) {
	let mut vbo: GLuint = 0;
	let mut ibo: GLuint = 0;
	unsafe {
		gl::GenBuffers(1, &mut vbo);
		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
		gl::BufferData(gl::ARRAY_BUFFER, (vertex_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, cast::transmute(&vertex_data[0]), gl::STATIC_DRAW);
		gl::BindBuffer(gl::ARRAY_BUFFER, 0);
	}
	unsafe {
		gl::GenBuffers(1, &mut ibo);
		gl::BindBuffer(gl::ARRAY_BUFFER, ibo);
		gl::BufferData(gl::ARRAY_BUFFER, (index_data.len() * mem::size_of::<GLshort>()) as GLsizeiptr, cast::transmute(&index_data[0]), gl::STATIC_DRAW);
		gl::BindBuffer(gl::ARRAY_BUFFER, 0);
	}
	(vbo, ibo)
}

fn init_vertex_array_obj(vbo: GLuint, ibo: GLuint) -> (GLuint, GLuint) {
	let mut vao1 = 0;
	let mut vao2 = 0;

	//bind vertex data
	unsafe{
		gl::GenVertexArrays(1, &mut vao1);
	}
	gl::BindVertexArray(vao1);

	let color_data_offset = mem::size_of::<GLfloat>() * 3 * vertex_num;

	gl::BindBuffer(gl::ARRAY_BUFFER, vbo); 

	gl::EnableVertexAttribArray(0);
	gl::EnableVertexAttribArray(1);
	unsafe {
		gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
		gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 0, cast::transmute(color_data_offset));
	}
	gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);

	gl::BindVertexArray(0); //unbind

	//bind index data
	unsafe{
		gl::GenVertexArrays(1, &mut vao2);
	}
	gl::BindVertexArray(vao2);
	
	let pos_data_offset = mem::size_of::<GLfloat>() * 3 * (vertex_num/2);
	let color_data_offset2 = color_data_offset + mem::size_of::<GLfloat>() * 4 * (vertex_num/2);
	
	gl::EnableVertexAttribArray(0);
	gl::EnableVertexAttribArray(1);
	unsafe {
		gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, cast::transmute(pos_data_offset));
		gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 0, cast::transmute(color_data_offset2));
	}
	gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);

	gl::BindVertexArray(0);

	(vao1, vao2)
}

fn init() -> (GLuint, GLuint, GLuint, GLuint, GLuint) {
	let program = init_program();
	let (vbo, ibo) = init_vertex_buffer();
	let (vao1, vao2) = init_vertex_array_obj(vbo, ibo);

	gl::Enable(gl::CULL_FACE);
	gl::CullFace(gl::BACK);
	gl::FrontFace(gl::CW);

	set_perspective_mat(program, frustum_scale, frustum_scale);

	(program, vbo, ibo, vao1, vao2)
}

fn display(program: GLuint, vao1: GLuint, vao2: GLuint, offset_unif: GLint) {
	gl::ClearColor(0.0, 0.0, 0.0, 0.0);
	gl::Clear(gl::COLOR_BUFFER_BIT);

	gl::UseProgram(program);

	gl::BindVertexArray(vao1);
	gl::Uniform3f(offset_unif, 0.0, 0.0, 0.0);
	unsafe {
		gl::DrawElements(gl::TRIANGLES, index_data.len() as i32, gl::UNSIGNED_SHORT, ptr::null());
	}

	gl::BindVertexArray(vao2);
	gl::Uniform3f(offset_unif, 0.0, 0.0, -1.0);
	unsafe {
		gl::DrawElements(gl::TRIANGLES, index_data.len() as i32, gl::UNSIGNED_SHORT, ptr::null());
	}

	gl::BindVertexArray(0);
	gl::UseProgram(0);
}

fn main() {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Choose a GL profile that is compatible with OS X 10.7+ -- from example code
    glfw.window_hint(glfw::ContextVersion(3, 2));
    glfw.window_hint(glfw::OpenglForwardCompat(true));
    glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));
	//glfw.window_hint(glfw::Resizable(false));

    let (window, events) = glfw.create_window(600, 600, "OpenGL", glfw::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current(); //make context current before calling gl::load_with
    window.set_key_polling(true); //enable internal polling function
	window.set_size_polling(true); //enable polling for size changes

    gl::load_with(|s| glfw.get_proc_address(s)); //loading opengl function pointers

	let (program, vbo, ibo, vao1, vao2) = init();
	let offset_unif = get_uniform(program, "offset");


    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
			match event {
				glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => { window.set_should_close(true) }
				glfw::KeyEvent(glfw::KeyQ, _, glfw::Press, _)      => { window.set_should_close(true) }
				glfw::SizeEvent(w, h) => { resize(w, h, program); }
				_ => {}
			}
        }

		display(program, vao1, vao2, offset_unif);
		window.swap_buffers();
    }

    // Cleanup
    gl::DeleteProgram(program);
    unsafe {
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteBuffers(1, &ibo);
        gl::DeleteVertexArrays(1, &vao1);
        gl::DeleteVertexArrays(1, &vao2);
    }
}

#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}



fn resize(w: i32, h: i32, program: GLuint) {
	println!("resize event: {} x {}", w, h);
	set_perspective_mat(program, frustum_scale / ((w as GLfloat) / (h as GLfloat)), frustum_scale);
	gl::Viewport(0, 0, w as GLsizei, h as GLsizei);
}


