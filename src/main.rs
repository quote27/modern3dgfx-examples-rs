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
extern crate nalgebra;

use gl::types::*;
use glfw::Context;
use std::mem;
use std::ptr;
use nalgebra::na::{Vec4, Mat4};
use nalgebra::na;


mod shaders;

static GREEN_COLOR:[GLfloat, ..4]  = [0.0, 1.0, 0.0, 1.0];
static BLUE_COLOR: [GLfloat, ..4]  = [0.0, 0.0, 1.0, 1.0];
static RED_COLOR:  [GLfloat, ..4]  = [1.0, 0.0, 0.0, 1.0];
//static GREY_COLOR: [GLfloat, ..4]  = [0.8, 0.8, 0.8, 1.0];
static BROWN_COLOR:[GLfloat, ..4]  = [0.5, 0.5, 0.0, 1.0];

static vertex_num: uint = 8;
static vertex_data:[GLfloat, ..56] = [
	//Object 1 positions
	 1.0,  1.0,  1.0,
	-1.0, -1.0,  1.0,
	-1.0,  1.0, -1.0,
	 1.0, -1.0, -1.0,

	-1.0, -1.0, -1.0,
	 1.0,  1.0, -1.0,
	 1.0, -1.0,  1.0,
	-1.0,  1.0,  1.0,

	GREEN_COLOR[0], GREEN_COLOR[1], GREEN_COLOR[2], GREEN_COLOR[3],
	BLUE_COLOR[0], BLUE_COLOR[1], BLUE_COLOR[2], BLUE_COLOR[3],
	RED_COLOR[0], RED_COLOR[1], RED_COLOR[2], RED_COLOR[3],
	BROWN_COLOR[0], BROWN_COLOR[1], BROWN_COLOR[2], BROWN_COLOR[3],

	GREEN_COLOR[0], GREEN_COLOR[1], GREEN_COLOR[2], GREEN_COLOR[3],
	BLUE_COLOR[0], BLUE_COLOR[1], BLUE_COLOR[2], BLUE_COLOR[3],
	RED_COLOR[0], RED_COLOR[1], RED_COLOR[2], RED_COLOR[3],
	BROWN_COLOR[0], BROWN_COLOR[1], BROWN_COLOR[2], BROWN_COLOR[3],
];

static index_data: [GLshort, ..24] = [
	0, 1, 2,
	1, 0, 3,
	2, 3, 0,
	3, 2, 1,

	5, 4, 6,
	4, 5, 7,
	7, 6, 4,
	6, 7, 5,
];

fn calc_frustum_scale(fov_deg: f32) -> f32 {
	// one liner: 1.0 / (fov_deg.to_radians() / 2.0).tan()
	let deg_rad = 3.14159 * 2.0 / 360.0;
	let fov_rad = fov_deg * deg_rad;
	1.0 / (fov_rad / 2.0).tan()
}

fn get_uniform(program: GLuint, name: &str) -> GLint {
	unsafe { gl::GetUniformLocation(program, name.with_c_str(|ptr| ptr)) }
	//alternative: unsafe { name.with_c_str(|ptr| gl::GetUniformLocation(program, ptr)); }
}


fn init_program() -> GLuint {
	println!("== init program ==");
	let mut shader_list = Vec::new();
	shader_list.push(shaders::load_shader_file(gl::VERTEX_SHADER, "s/PosColorLocalTransform.vert"));
    shader_list.push(shaders::load_shader_file(gl::FRAGMENT_SHADER, "s/ColorPassthrough.frag"));
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
		gl::BufferData(gl::ARRAY_BUFFER, (vertex_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, mem::transmute(&vertex_data[0]), gl::STATIC_DRAW);
		gl::BindBuffer(gl::ARRAY_BUFFER, 0);
	}
	unsafe {
		gl::GenBuffers(1, &mut ibo);
		gl::BindBuffer(gl::ARRAY_BUFFER, ibo);
		gl::BufferData(gl::ARRAY_BUFFER, (index_data.len() * mem::size_of::<GLshort>()) as GLsizeiptr, mem::transmute(&index_data[0]), gl::STATIC_DRAW);
		gl::BindBuffer(gl::ARRAY_BUFFER, 0);
	}
	(vbo, ibo)
}

fn init_vertex_array_obj(vbo: GLuint, ibo: GLuint) -> GLuint {
	let mut vao1 = 0;
	let color_data_offset = mem::size_of::<GLfloat>() * 3 * vertex_num;

	//bind vertex data
	unsafe{
		gl::GenVertexArrays(1, &mut vao1);
	}
	gl::BindVertexArray(vao1);


	gl::BindBuffer(gl::ARRAY_BUFFER, vbo); 

	gl::EnableVertexAttribArray(0);
	gl::EnableVertexAttribArray(1);
	unsafe {
		gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
		gl::VertexAttribPointer(1, 4, gl::FLOAT, gl::FALSE, 0, mem::transmute(color_data_offset));
	}
	gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);

	gl::BindVertexArray(0); //unbind

	vao1
}

fn init() -> (GLuint, GLuint, GLuint, GLuint, Mat4<f32>, f32) {
	let program = init_program();
	let (vbo, ibo) = init_vertex_buffer();
	let vao1 = init_vertex_array_obj(vbo, ibo);

	gl::Enable(gl::CULL_FACE);
	gl::CullFace(gl::BACK);
	gl::FrontFace(gl::CW);

	gl::Enable(gl::DEPTH_TEST);
	gl::DepthMask(gl::TRUE);
	gl::DepthFunc(gl::LEQUAL);
	gl::DepthRange(0.0, 1.0);

	//TODO: camera to clip matrix
	let frustum_scale = calc_frustum_scale(45.0);
	let (znear, zfar) = (1.0, 45.0);
	let mut cam_clip_m: Mat4<f32> = na::zero();

	cam_clip_m.m11 = frustum_scale;
	cam_clip_m.m22 = frustum_scale;
	cam_clip_m.m33 = (zfar + znear) / (znear - zfar);
	cam_clip_m.m34 = -1.0;
	cam_clip_m.m43 = (2.0 * zfar * znear) / (znear - zfar);

	let cam_clip_unif = get_uniform(program, "cameraToClipMatrix");
	gl::UseProgram(program);
	unsafe { gl::UniformMatrix4fv(cam_clip_unif, 1, gl::TRUE, mem::transmute(&cam_clip_m.m11)); }
	gl::UseProgram(0);

	(program, vbo, ibo, vao1, cam_clip_m, frustum_scale)
}

struct Instance {
	calc_offset: fn(elap_time: f32) -> Vec4<f32>,
}
impl Instance {
	pub fn construct_matrix(&self, elap_time: f32) -> Mat4<f32> {
		let mut m: Mat4<f32> = na::one();
		let fco = self.calc_offset;
		let vec = fco(elap_time);

		m.m41 = vec.x;
		m.m42 = vec.y;
		m.m43 = vec.z;
		m.m44 = vec.w;
		m
	}
}

fn stationary_offset(elap_time: f32) -> Vec4<f32> {
	Vec4::new(0.0, 0.0, -20.0, 1.0)
}

fn oval_offset(elap_time: f32) -> Vec4<f32> {
	let fscale = 3.14159 * 2.0 / 5.0;
	let elap_time_scale = elap_time * fscale;
	Vec4::new(
		elap_time_scale.cos() * 4.0,
		elap_time_scale.sin() * 6.0,
		-20.0,
		1.0)
}

fn bottom_circle_offset(elap_time: f32) -> Vec4<f32> {
	let fscale = 3.14159 * 2.0 / 5.0;
	let elap_time_scale = elap_time * fscale;
	Vec4::new(
		elap_time_scale.cos() * 5.0,
		-3.5,
		elap_time_scale.sin() * 5.0 - 20.0,
		1.0)
}

fn gen_instance_list() -> [Instance, ..3] {
	[
		Instance {calc_offset: stationary_offset},
		Instance {calc_offset: oval_offset},
		Instance {calc_offset: bottom_circle_offset},
	]
}

fn display(program: GLuint, vao1: GLuint, elap_time: f32, inst_list: &[Instance]) {
	gl::ClearColor(0.0, 0.0, 0.0, 0.0);
	gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

	gl::UseProgram(program);
	let mod_cam_unif = get_uniform(program, "modelToCameraMatrix");

	gl::BindVertexArray(vao1);

	for i in inst_list.iter() {
		let m = i.construct_matrix(elap_time);

		unsafe { gl::UniformMatrix4fv(mod_cam_unif, 1, gl::TRUE, mem::transmute(&m)); }
		unsafe { gl::DrawElements(gl::TRIANGLES, index_data.len() as i32, gl::UNSIGNED_SHORT, ptr::null()); }
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

	let (program, vbo, ibo, vao1, mut cam_clip_m, frustum_scale) = init();

	let mut depth_clamp = false;

	let instance_list = gen_instance_list();


    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
			match event {
				glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => { window.set_should_close(true) }
				glfw::KeyEvent(glfw::KeyQ, _, glfw::Press, _)      => { window.set_should_close(true) }
				glfw::KeyEvent(glfw::KeyD, _, glfw::Press, _)      => {
					if depth_clamp {
						gl::Disable(gl::DEPTH_CLAMP);
					} else {
						gl::Enable(gl::DEPTH_CLAMP);
					}
					depth_clamp = !depth_clamp;
				}
				glfw::SizeEvent(w, h) => { resize(w, h, program, &mut cam_clip_m, frustum_scale); }
				_ => {}
			}
        }

		display(program, vao1, glfw.get_time() as f32, instance_list);
		window.swap_buffers();
    }

    // Cleanup
    gl::DeleteProgram(program);
    unsafe {
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteBuffers(1, &ibo);
        gl::DeleteVertexArrays(1, &vao1);
    }
}

#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
}



fn resize(w: i32, h: i32, program: GLuint, cam_clip_m: &mut Mat4<f32>, frustum_scale: f32) {
	println!("resize event: {} x {}", w, h);
	cam_clip_m.m11 = frustum_scale * (h as f32 / w as f32);
	cam_clip_m.m22 = frustum_scale;

	let cam_clip_unif = get_uniform(program, "cameraToClipMatrix");
	gl::UseProgram(program);
	unsafe { gl::UniformMatrix4fv(cam_clip_unif, 1, gl::TRUE, mem::transmute(&cam_clip_m.m11)); }
	gl::UseProgram(0);

	gl::Viewport(0, 0, w as GLsizei, h as GLsizei);
}


