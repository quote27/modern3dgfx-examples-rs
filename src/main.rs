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

/* // frag color tut
static vertex_data: [GLfloat, ..12] = [
    0.75,  0.75, 0.0, 1.0,
    0.75, -0.75, 0.0, 1.0,
   -0.75, -0.75, 0.0, 1.0,
];

static VS_SRC: &'static str =
"#version 330\n\
 layout(location = 0) in vec4 position;\n\
 void main() {\n\
   gl_Position = position;\n\
 }";

static FS_SRC: &'static str =
"#version 330\n\
 out vec4 out_color;\n\
 void main() {\n\
   float lerpValue = gl_FragCoord.y / 500.0f;\n\
   \n\
   out_color = mix(vec4(1.0f, 1.0f, 1.0f, 1.0f), vec4(0.2f, 0.2f, 0.2f, 1.0f), lerpValue);\n\
 }";

// */

//* // vertex colors tut
static vertex_data: [GLfloat, ..12] = [
	 0.25,  0.25, 0.0, 1.0,
	 0.25, -0.25, 0.0, 1.0,
	-0.25, -0.25, 0.0, 1.0,
];

static color_data: [GLfloat, ..12] = [
	 1.0,    0.0, 0.0, 1.0,
	 0.0,    1.0, 0.0, 1.0,
	 0.0,    0.0, 1.0, 1.0,
];

static VS_SRC: &'static str =
"#version 330\n\
layout (location = 0) in vec4 position;\n\
uniform float loop_duration;\n\
uniform float time;
\n\
void main() {\n\
	float tscale = 3.14159f * 2.0f / loop_duration;
	float curr_time = time; //mod(time, loop_duration);
	vec4 offvec = vec4(
		cos(curr_time * tscale) * 0.5f,
		sin(curr_time * tscale) * 0.5f,
		0.0f, 0.0f);
   
	//vec4 offvec = vec4(offset.x, offset.y, 0.0, 0.0);\n\
	//vec4 offvec = vec4(offset, 0.0, 0.0);\n\
	gl_Position = position + offvec;\n\
}";

static FS_SRC: &'static str =
"#version 330\n\
 //smooth in vec4 theColor;\n\
 out vec4 out_color;\n\
 \n\
 void main() {\n\
   //out_color = theColor * (gl_FragCoord.y / 500.0f);\n\
   out_color = vec4(1.0f, 1.0f, 1.0f, 1.0f);\n\
 }";

// */

fn init_program() -> GLuint {
    let vs = shaders::load_shader(VS_SRC, gl::VERTEX_SHADER);
    let fs = shaders::load_shader(FS_SRC, gl::FRAGMENT_SHADER);
    let p = shaders::create_program(vs, fs);

	gl::DeleteShader(vs);
	gl::DeleteShader(fs);
	p
}

fn init_vertex_buffer(vbo: &mut GLuint) {
	unsafe {
		gl::GenBuffers(1, vbo);

		gl::BindBuffer(gl::ARRAY_BUFFER, *vbo);
		gl::BufferData(gl::ARRAY_BUFFER, (vertex_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, cast::transmute(&vertex_data[0]), gl::STATIC_DRAW);
		gl::BindBuffer(gl::ARRAY_BUFFER, 0);
	}
}

fn init_color_buffer(cbo: &mut GLuint) {
	unsafe {
		gl::GenBuffers(1, cbo);

		gl::BindBuffer(gl::ARRAY_BUFFER, *cbo);
		gl::BufferData(gl::ARRAY_BUFFER, (vertex_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, cast::transmute(&color_data[0]), gl::STATIC_DRAW);
		gl::BindBuffer(gl::ARRAY_BUFFER, 0);
	}
}

fn compute_offsets(felap_time: f32) -> (f32, f32) {
	let floop_duration: f32 = 5.0;
	let fscale: f32 = 3.14159 * 2.0 / floop_duration;

	//let (_, fcurr_time_in_loop) = num::div_rem(felap_time, floop_duration);
	//let fcurr_time_in_loop = felap_time % floop_duration;
	//wiki: modulo definition: a % n = a - (n * floor(a/n))
	let fcurr_time_in_loop = felap_time - (floop_duration * (felap_time / floop_duration).floor());

	((fcurr_time_in_loop * fscale).cos() * 0.5, (fcurr_time_in_loop * fscale).sin() * 0.5)
}

fn adjust_vert_data(vbo: GLuint, xoff: f32, yoff: f32) {
	//let mut adjvert_data: [GLfloat, ..12] = [0.0, ..12];
	let mut adjvert_data_vec = Vec::from_slice(vertex_data);
	let mut adjvert_data = adjvert_data_vec.as_mut_slice();


	let mut i = 0;
	while i < vertex_data.len() {
		adjvert_data[i] += xoff;
		adjvert_data[i+1] += yoff;
		i+=4;
	}

	unsafe {
	gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
	gl::BufferSubData(gl::ARRAY_BUFFER, 0, (adjvert_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr, cast::transmute(&adjvert_data[0]));
	gl::BindBuffer(gl::ARRAY_BUFFER, 0);
	}

}

fn main() {
	// init glfw
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Choose a GL profile that is compatible with OS X 10.7+
    glfw.window_hint(glfw::ContextVersion(3, 2));
    glfw.window_hint(glfw::OpenglForwardCompat(true));
    glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));

    let (window, events) = glfw.create_window(800, 600, "OpenGL", glfw::Windowed)
        .expect("Failed to create GLFW window.");

    // It is essential to make the context current before calling `gl::load_with`.
    window.make_current();

	// if true, set the polling function internally [in glfw c, you have to provide the keyboard function
    window.set_key_polling(true);

    // Load the OpenGL function pointers
    gl::load_with(|s| glfw.get_proc_address(s));
	println!("loaded opengl functions");

    // Create GLSL shader program
	let program = init_program();
	println!("compiled shaders");

	let mut loop_duration_loc;
	let mut time_loc;
	unsafe {
		loop_duration_loc = "loop_duration".with_c_str(|ptr| gl::GetUniformLocation(program, ptr));
		time_loc = "time".with_c_str(|ptr| gl::GetUniformLocation(program, ptr));
	}

	gl::UseProgram(program);
	gl::Uniform1f(loop_duration_loc, 5.0);
	gl::UseProgram(0);


    let mut vao = 0;
    let mut vbo = 0;
	let mut cbo = 0;
	init_vertex_buffer(&mut vbo);
	init_color_buffer(&mut cbo);
	println!("init vertex buffer");

	unsafe{
		gl::GenVertexArrays(1, &mut vao);
	}
	gl::BindVertexArray(vao);

	/*
    unsafe {
        // Create Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertex_data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       cast::transmute(&vertex_data[0]),
                       gl::STATIC_DRAW);

        // Use shader program
        gl::UseProgram(program);
        "out_color".with_c_str(|ptr| gl::BindFragDataLocation(program, 0, ptr));

        // Specify the layout of the vertex data
        let pos_attr = "position".with_c_str(|ptr| gl::GetAttribLocation(program, ptr));
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(pos_attr as GLuint, 2, gl::FLOAT,
                                gl::FALSE as GLboolean, 0, ptr::null());
    }
	*/


    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&window, event);
        }

		//let (xoff, yoff) = compute_offsets(glfw.get_time() as f32);
		// adjust_vert_data(vbo, xoff, yoff);


        // Clear the screen to black
        gl::ClearColor(0.0, 0.0, 0.0, 0.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

		gl::UseProgram(program);

		println!("{}", glfw.get_time());
		gl::Uniform1f(time_loc, glfw.get_time() as f32);

		gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
		gl::EnableVertexAttribArray(0);
		unsafe{
			gl::VertexAttribPointer(0, 4, gl::FLOAT, gl::FALSE, 0, ptr::null());
		}

		gl::DrawArrays(gl::TRIANGLES, 0, 3);

		gl::DisableVertexAttribArray(0);
		gl::DisableVertexAttribArray(1);
		gl::UseProgram(0);

        // Swap buffers
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


