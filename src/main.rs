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
use nalgebra::na::{Vec3, Mat4};
use nalgebra::na;


mod shaders;

static GREEN_COLOR:   [GLfloat, ..4]  = [0.0, 1.0, 0.0, 1.0];
static BLUE_COLOR:    [GLfloat, ..4]  = [0.0, 0.0, 1.0, 1.0];
static RED_COLOR:     [GLfloat, ..4]  = [1.0, 0.0, 0.0, 1.0];
static YELLOW_COLOR:  [GLfloat, ..4]  = [1.0, 1.0, 0.0, 1.0];
static CYAN_COLOR:    [GLfloat, ..4]  = [0.0, 1.0, 1.0, 1.0];
static MAGENTA_COLOR: [GLfloat, ..4]  = [1.0, 0.0, 1.0, 1.0];

/*
static vertex_num: uint = 24;
static vertex_data:[GLfloat, ..168] = [
	//Front
	 1.0,  1.0,  1.0,
	 1.0, -1.0,  1.0,
	-1.0, -1.0,  1.0,
	-1.0,  1.0,  1.0,

	//Top
	 1.0,  1.0,  1.0,
	-1.0,  1.0,  1.0,
	-1.0,  1.0, -1.0,
	 1.0,  1.0, -1.0,

	//Let
	 1.0,  1.0,  1.0,
	 1.0,  1.0, -1.0,
	 1.0, -1.0, -1.0,
	 1.0, -1.0,  1.0,

	//Back
	 1.0,  1.0, -1.0,
	-1.0,  1.0, -1.0,
	-1.0, -1.0, -1.0,
	 1.0, -1.0, -1.0,

	//Bottom
	 1.0, -1.0,  1.0,
	 1.0, -1.0, -1.0,
	-1.0, -1.0, -1.0,
	-1.0, -1.0,  1.0,

	//Right
	-1.0,  1.0,  1.0,
	-1.0, -1.0,  1.0,
	-1.0, -1.0, -1.0,
	-1.0,  1.0, -1.0,

	//colors
	GREEN_COLOR[0], GREEN_COLOR[1], GREEN_COLOR[2], GREEN_COLOR[3],
	GREEN_COLOR[0], GREEN_COLOR[1], GREEN_COLOR[2], GREEN_COLOR[3],
	GREEN_COLOR[0], GREEN_COLOR[1], GREEN_COLOR[2], GREEN_COLOR[3],
	GREEN_COLOR[0], GREEN_COLOR[1], GREEN_COLOR[2], GREEN_COLOR[3],

	BLUE_COLOR[0], BLUE_COLOR[1], BLUE_COLOR[2], BLUE_COLOR[3],
	BLUE_COLOR[0], BLUE_COLOR[1], BLUE_COLOR[2], BLUE_COLOR[3],
	BLUE_COLOR[0], BLUE_COLOR[1], BLUE_COLOR[2], BLUE_COLOR[3],
	BLUE_COLOR[0], BLUE_COLOR[1], BLUE_COLOR[2], BLUE_COLOR[3],

	RED_COLOR[0], RED_COLOR[1], RED_COLOR[2], RED_COLOR[3],
	RED_COLOR[0], RED_COLOR[1], RED_COLOR[2], RED_COLOR[3],
	RED_COLOR[0], RED_COLOR[1], RED_COLOR[2], RED_COLOR[3],
	RED_COLOR[0], RED_COLOR[1], RED_COLOR[2], RED_COLOR[3],

	YELLOW_COLOR[0], YELLOW_COLOR[1], YELLOW_COLOR[2], YELLOW_COLOR[3],
	YELLOW_COLOR[0], YELLOW_COLOR[1], YELLOW_COLOR[2], YELLOW_COLOR[3],
	YELLOW_COLOR[0], YELLOW_COLOR[1], YELLOW_COLOR[2], YELLOW_COLOR[3],
	YELLOW_COLOR[0], YELLOW_COLOR[1], YELLOW_COLOR[2], YELLOW_COLOR[3],

	CYAN_COLOR[0], CYAN_COLOR[1], CYAN_COLOR[2], CYAN_COLOR[3],
	CYAN_COLOR[0], CYAN_COLOR[1], CYAN_COLOR[2], CYAN_COLOR[3],
	CYAN_COLOR[0], CYAN_COLOR[1], CYAN_COLOR[2], CYAN_COLOR[3],
	CYAN_COLOR[0], CYAN_COLOR[1], CYAN_COLOR[2], CYAN_COLOR[3],

	MAGENTA_COLOR[0], MAGENTA_COLOR[1], MAGENTA_COLOR[2], MAGENTA_COLOR[3],
	MAGENTA_COLOR[0], MAGENTA_COLOR[1], MAGENTA_COLOR[2], MAGENTA_COLOR[3],
	MAGENTA_COLOR[0], MAGENTA_COLOR[1], MAGENTA_COLOR[2], MAGENTA_COLOR[3],
	MAGENTA_COLOR[0], MAGENTA_COLOR[1], MAGENTA_COLOR[2], MAGENTA_COLOR[3],
];

static index_data: [GLshort, ..36] = [
	0, 1, 2,
	2, 3, 0,

	4, 5, 6,
	6, 7, 4,

	8, 9, 10,
	10, 11, 8,

	12, 13, 14,
	14, 15, 12,

	16, 17, 18,
	18, 19, 16,

	20, 21, 22,
	22, 23, 20,
];
// */

//* from translations demo
static vertex_num: uint = 8;
static vertex_data:[GLfloat, ..56] = [
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
	YELLOW_COLOR[0], YELLOW_COLOR[1], YELLOW_COLOR[2], YELLOW_COLOR[3],

	GREEN_COLOR[0], GREEN_COLOR[1], GREEN_COLOR[2], GREEN_COLOR[3],
	BLUE_COLOR[0], BLUE_COLOR[1], BLUE_COLOR[2], BLUE_COLOR[3],
	RED_COLOR[0], RED_COLOR[1], RED_COLOR[2], RED_COLOR[3],
	YELLOW_COLOR[0], YELLOW_COLOR[1], YELLOW_COLOR[2], YELLOW_COLOR[3],
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
// */

struct GLState {
	program: GLuint,
	pos_attr: GLuint,
	color_attr: GLuint,

	mod_cam_unif: GLint,
	cam_clip_unif: GLint,

	frustum_scale: f32,
	cam_clip_m: Mat4<f32>,

	vbo: GLuint,
	ibo: GLuint,
	vao: GLuint,
}
impl GLState {
	pub fn new() -> GLState {
		GLState {
			program: 0,
			pos_attr: 0,
			color_attr: 0,

			mod_cam_unif: 0,
			cam_clip_unif: 0,

			frustum_scale: 1.0,
			cam_clip_m: na::zero(),

			vbo: 0,
			ibo: 0,
			vao: 0,
		}
	}
}

fn calc_frustum_scale(fov_deg: f32) -> f32 {
	let deg_rad = 3.14159 * 2.0 / 360.0;
	let fov_rad = fov_deg * deg_rad;
	1.0 / (fov_rad / 2.0).tan()
}

//TODO: inline
fn get_uniform(program: GLuint, name: &str) -> GLint {
	unsafe { gl::GetUniformLocation(program, name.with_c_str(|ptr| ptr)) }
}
//TODO: inline
fn get_attrib(program: GLuint, name: &str) -> GLuint {
	unsafe { gl::GetAttribLocation(program, name.with_c_str(|ptr| ptr)) as GLuint }
}


// matrix format - row major memory order
// initial memory order -- using this for now, passing true to opengl
// m11 m21 m31 m41
// m12 m22 m32 m42
// m13 m23 m33 m43
// m14 m24 m34 m44
//
//transposed logically
// m11 m12 m13 m14
// m22 m22 m23 m24
// m33 m32 m33 m34
// m44 m42 m43 m44

fn init_prog(state: &mut GLState) {
	let mut shader_list = Vec::new();
	shader_list.push(shaders::load_shader_file(gl::VERTEX_SHADER, "s/PosColorLocalTransform.vert"));
	shader_list.push(shaders::load_shader_file(gl::FRAGMENT_SHADER, "s/ColorPassthrough.frag"));
	state.program = shaders::create_program(&shader_list);

	state.pos_attr = get_attrib(state.program, "position");
	state.color_attr = get_attrib(state.program, "color");

	state.mod_cam_unif = get_uniform(state.program, "modelToCameraMatrix");
	state.cam_clip_unif = get_uniform(state.program, "cameraToClipMatrix");

	state.frustum_scale = calc_frustum_scale(45.0);
	let (znear, zfar) = (1.0, 100.0);

	state.cam_clip_m = na::zero();
	state.cam_clip_m.m11 = state.frustum_scale;
	state.cam_clip_m.m22 = state.frustum_scale;
	state.cam_clip_m.m33 = (zfar + znear) / (znear - zfar);
	state.cam_clip_m.m43 = -1.0;
	state.cam_clip_m.m34 = (2.0 * zfar * znear) / (znear - zfar);

	gl::UseProgram(state.program);
	unsafe { gl::UniformMatrix4fv(state.cam_clip_unif, 1, gl::TRUE, mem::transmute(&state.cam_clip_m)); }
	gl::UseProgram(0);
}

fn init_vao(state: &mut GLState) {
	unsafe {
	gl::GenBuffers(1, &mut state.vbo);
	gl::BindBuffer(gl::ARRAY_BUFFER, state.vbo);
	gl::BufferData(gl::ARRAY_BUFFER, (vertex_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, mem::transmute(&vertex_data[0]), gl::STATIC_DRAW);
	gl::BindBuffer(gl::ARRAY_BUFFER, 0);
	}

	unsafe {
	gl::GenBuffers(1, &mut state.ibo);
	gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, state.ibo);
	gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (index_data.len() * mem::size_of::<GLshort>()) as GLsizeiptr, mem::transmute(&index_data[0]), gl::STATIC_DRAW);
	gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
	}

	unsafe { gl::GenVertexArrays(1, &mut state.vao); }
	gl::BindVertexArray(state.vao);

	let color_data_offset = mem::size_of::<GLfloat>() * 3 * vertex_num;
	gl::BindBuffer(gl::ARRAY_BUFFER, state.vbo);
	gl::EnableVertexAttribArray(state.pos_attr);
	gl::EnableVertexAttribArray(state.color_attr);
	unsafe {
	gl::VertexAttribPointer(state.pos_attr, 3, gl::FLOAT, gl::FALSE, 0, ptr::null());
	gl::VertexAttribPointer(state.color_attr, 4, gl::FLOAT, gl::FALSE, 0, mem::transmute(color_data_offset));
	}
	gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, state.ibo);

	gl::BindVertexArray(0);
}

//TODO: inline
fn deg_rad(ang_deg: f32) -> f32 {
	let deg_rad = 3.14159 * 2.0 / 360.0;
	ang_deg * deg_rad
}

//TODO: inline
fn clamp(v: f32, min: f32, max: f32) -> f32 {
	if v < min {
		min
	} else if v > max {
		max
	} else {
		v
	}
}

fn rotx(ang: f32) -> Mat4<f32> {
	let rad = deg_rad(ang);
	let (cos, sin) = (rad.cos(), rad.sin());

	let mut m: Mat4<f32> = na::one();
	m.m22 = cos; m.m23 =-sin;
	m.m32 = sin; m.m33 = cos;
	m
}

fn roty(ang: f32) -> Mat4<f32> {
	let rad = deg_rad(ang);
	let (cos, sin) = (rad.cos(), rad.sin());

	let mut m: Mat4<f32> = na::one();
	m.m11 = cos; m.m31 = sin;
	m.m13 =-sin; m.m33 = cos;
	m
}

fn rotz(ang: f32) -> Mat4<f32> {
	let rad = deg_rad(ang);
	let (cos, sin) = (rad.cos(), rad.sin());

	let mut m: Mat4<f32> = na::one();
	m.m11 = cos; m.m21 =-sin;
	m.m12 = sin; m.m22 = cos;
	m
}

struct MatrixStack {
	c: Mat4<f32>,
	s: Vec<Mat4<f32>>,
}
impl MatrixStack {
	pub fn new() -> MatrixStack {
		MatrixStack {
			c: na::one(),
			s: Vec::new(),
		}
	}

	pub fn top(&self) -> Mat4<f32> {
		self.c
	}

	pub fn rotx(&mut self, deg: f32) {
		self.c = self.c * ::rotx(deg);
	}

	pub fn roty(&mut self, deg: f32) {
		self.c = self.c * ::roty(deg);
	}

	pub fn rotz(&mut self, deg: f32) {
		self.c = self.c * ::rotz(deg);
	}

	pub fn scale(&mut self, sv: Vec3<f32>) {
		let mut m: Mat4<f32> = na::one();
		m.m11 = sv.x;
		m.m22 = sv.y;
		m.m33 = sv.z;
		self.c = self.c * m;
	}

	pub fn trans(&mut self, sv: Vec3<f32>) {
		let mut m: Mat4<f32> = na::one();
		m.m41 = sv.x;
		m.m42 = sv.y;
		m.m43 = sv.z;
		self.c = self.c * m;
	}

	pub fn push(&mut self) {
		self.s.push(self.c);
	}

	pub fn pop(&mut self) {
		self.c = match self.s.pop() {
			Some(x) => x,
			None => na::one(),
		}
	}
}

/*
struct Hierarchy {
	a: int;
}
impl Hierarchy {
	pub fn new() -> Hierarchy {
		Hierarchy { a: 0 }
	}
}
*/

fn init() -> GLState {
	let mut state = GLState::new();

	init_prog(&mut state);
	init_vao(&mut state);

	gl::Enable(gl::CULL_FACE);
	gl::CullFace(gl::BACK);
	gl::FrontFace(gl::CW);

	gl::Enable(gl::DEPTH_TEST);
	gl::DepthMask(gl::TRUE);
	gl::DepthFunc(gl::LEQUAL);
	gl::DepthRange(0.0, 1.0);
	state
}

fn display(state: &GLState, win: &glfw::Window) {
	gl::ClearColor(0.0, 0.0, 0.0, 0.0);
	gl::ClearDepth(1.0);
	gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

	let mut ms = MatrixStack::new();
	ms.trans(Vec3::new(0.0, 0.0, -40.0));
	// draw something
	gl::BindVertexArray(state.vao);

	let m = ms.top();

	unsafe { gl::UniformMatrix4fv(state.mod_cam_unif, 1, gl::TRUE, mem::transmute(&m)); }
	unsafe { gl::DrawElements(gl::TRIANGLES, index_data.len() as i32, gl::UNSIGNED_SHORT, ptr::null()); }

	gl::BindVertexArray(0);
	gl::UseProgram(0);
	
	win.swap_buffers();
}



/*
fn robot_kb(robot: &mut Hierarchy, event: glfw::WindowEvent) {
	match event {
	glfw::KeyEvent(glfw::KeyA, _, glfw::Press, _) => { robot.adj_base(true)		    }
	glfw::KeyEvent(glfw::KeyD, _, glfw::Press, _) => { robot.adj_base(false)        }
	glfw::KeyEvent(glfw::KeyW, _, glfw::Press, _) => { robot.adj_upperarm(false)    }
	glfw::KeyEvent(glfw::KeyS, _, glfw::Press, _) => { robot.adj_upperarm(true)     }
	glfw::KeyEvent(glfw::KeyR, _, glfw::Press, _) => { robot.adj_lowerarm(false)    }
	glfw::KeyEvent(glfw::KeyF, _, glfw::Press, _) => { robot.adj_lowerarm(true)     }
	glfw::KeyEvent(glfw::KeyT, _, glfw::Press, _) => { robot.adj_wrist_pitch(false) }
	glfw::KeyEvent(glfw::KeyG, _, glfw::Press, _) => { robot.adj_wrist_pitch(true)  }
	glfw::KeyEvent(glfw::KeyZ, _, glfw::Press, _) => { robot.adj_wrist_roll(true)   }
	glfw::KeyEvent(glfw::KeyC, _, glfw::Press, _) => { robot.adj_wrist_roll(false)  }
	glfw::KeyEvent(glfw::KeyQ, _, glfw::Press, _) => { robot.adj_finger_open(true)  }
	glfw::KeyEvent(glfw::KeyE, _, glfw::Press, _) => { robot.adj_finger_open(false) }
	glfw::KeyEvent(glfw::KeyP, _, glfw::Press, _) => { robot.write_pose()           }
	_ => { }
	}
}
*/

fn resize(w: i32, h: i32, state: &mut GLState) {
	println!("resize event: {} x {}", w, h);
	state.cam_clip_m.m11 = state.frustum_scale * (h as f32 / w as f32);
	state.cam_clip_m.m22 = state.frustum_scale;

	gl::UseProgram(state.program);
	unsafe { gl::UniformMatrix4fv(state.cam_clip_unif, 1, gl::TRUE, mem::transmute(&state.cam_clip_m.m11)); }
	gl::UseProgram(0);

	gl::Viewport(0, 0, w as GLsizei, h as GLsizei);
}


#[start]
fn start(argc: int, argv: **u8) -> int {
    native::start(argc, argv, main)
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

	let mut state = init();

	let mut depth_clamp = false;


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
				glfw::SizeEvent(w, h) => { resize(w, h, &mut state); }
				_ => {}
			}
        }

		display(&state, &window);
    }

    // Cleanup
    gl::DeleteProgram(state.program);
    unsafe {
        gl::DeleteBuffers(1, &state.vbo);
        gl::DeleteBuffers(1, &state.ibo);
        gl::DeleteVertexArrays(1, &state.vao);
    }
}
