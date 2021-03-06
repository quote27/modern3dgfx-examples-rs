#![feature(globs)]
#![feature(macro_rules)]

extern crate native;
extern crate gl;
extern crate glfw;
extern crate cgmath;

use gl::types::*;
use glfw::Context;
use std::mem;
use std::ptr;
use cgmath::matrix::Matrix4;
use cgmath::vector::Vector3;
use util::MatrixStack;
use shaders::{Shader, Program, Uniform};
use mesh::Mesh;

mod shaders;
mod util;
mod mesh;

static GREEN_COLOR:   [GLfloat, ..4]  = [0.0, 1.0, 0.0, 1.0];
static BLUE_COLOR:    [GLfloat, ..4]  = [0.0, 0.0, 1.0, 1.0];
static RED_COLOR:     [GLfloat, ..4]  = [1.0, 0.0, 0.0, 1.0];
static YELLOW_COLOR:  [GLfloat, ..4]  = [1.0, 1.0, 0.0, 1.0];
static CYAN_COLOR:    [GLfloat, ..4]  = [0.0, 1.0, 1.0, 1.0];
static MAGENTA_COLOR: [GLfloat, ..4]  = [1.0, 0.0, 1.0, 1.0];

//*
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

/* from translations demo
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
    program: Program,
    pos_attr: GLuint,
    color_attr: GLuint,

    mod_cam_unif: Uniform,
    cam_clip_unif: Uniform,

    frustum_scale: f32,
    cam_clip_m: Matrix4<f32>,

    vbo: GLuint,
    ibo: GLuint,
    vao: GLuint,
}
impl GLState {
    pub fn print(&self) {
        println!("prog: {}, pos_attr: {}, color_attr: {}", self.program, self.pos_attr, self.color_attr);
        println!("mod_cam_unif: {}, cam_clip_unif: {}", self.mod_cam_unif, self.cam_clip_unif);
        println!("vbo: {}, ibo: {}, vao: {}", self.vbo, self.ibo, self.vao);
        println!("frustum_scale: {}", self.frustum_scale);
        util::print_mat(&self.cam_clip_m);
    }

    pub fn delete(&mut self) {
    }
}

fn init_prog() -> GLState {
    let prog =  Program::new(&vec!(
            Shader::from_file(gl::VERTEX_SHADER, "shaders/PosColorLocalTransform.vert"),
            Shader::from_file(gl::FRAGMENT_SHADER, "shaders/ColorPassthrough.frag"),
        ));

    let cam_clip_unif = prog.get_unif("cameraToClipMatrix");

    let fs = util::calc_frustum_scale(45.0);
    let (znear, zfar) = (1.0, 100.0);

    let mut cam_clip_m = Matrix4::zero();
    cam_clip_m.x.x = fs;
    cam_clip_m.y.y = fs;
    cam_clip_m.z.z = (zfar + znear) / (znear - zfar);
    cam_clip_m.z.w = -1.0;
    cam_clip_m.w.z = (2.0 * zfar * znear) / (znear - zfar);

    prog.use_prog();
    cam_clip_unif.upload_m4f(&cam_clip_m);
    gl::UseProgram(0);

    GLState {
        program: prog,
        pos_attr: 0, //prog.get_attrib("position"); // TODO: returning a non-zero value [should be 0 in this case] - I think it's an int->long issue
        color_attr: prog.get_attrib("color"),

        mod_cam_unif: prog.get_unif("modelToCameraMatrix"),
        cam_clip_unif: cam_clip_unif,

        frustum_scale: fs,
        cam_clip_m: cam_clip_m,

        vbo: 0,
        ibo: 0,
        vao: 0,
    }
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



struct Hierarchy {
    pos_base: Vector3<f32>,
    ang_base: f32,

    pos_base_left: Vector3<f32>,
    pos_base_right: Vector3<f32>,
    scale_base_z: f32,

    ang_upperarm: f32,
    size_upperarm: f32,

    pos_lowerarm: Vector3<f32>,
    ang_lowerarm: f32,
    len_lowerarm: f32,
    width_lowerarm: f32,

    pos_wrist: Vector3<f32>,
    ang_wrist_roll: f32,
    ang_wrist_pitch: f32,
    len_wrist: f32,
    width_wrist: f32,

    pos_left_finger: Vector3<f32>,
    pos_right_finger: Vector3<f32>,
    ang_finger_open: f32,
    len_finger: f32,
    width_finger: f32,
    ang_lower_finger: f32,

    ang_inc_standard: f32,
    ang_inc_small: f32,
}

impl Hierarchy {

    pub fn new() -> Hierarchy {
        Hierarchy {
            pos_base:		Vector3::new(3.0, -5.0, -40.0),
            ang_base:		-45.0,
            pos_base_left:	Vector3::new(2.0, 0.0, 0.0),
            pos_base_right:	Vector3::new(-2.0, 0.0, 0.0),
            scale_base_z:	3.0,
            ang_upperarm:	-33.75,
            size_upperarm:	9.0,
            pos_lowerarm:	Vector3::new(0.0, 0.0, 8.0),
            ang_lowerarm:	146.25,
            len_lowerarm:	5.0,
            width_lowerarm:	1.5,
            pos_wrist:		Vector3::new(0.0, 0.0, 5.0),
            ang_wrist_roll:	0.0,
            ang_wrist_pitch:67.5,
            len_wrist:		2.0,
            width_wrist:	2.0,
            pos_left_finger:Vector3::new(1.0, 0.0, 1.0),
            pos_right_finger:Vector3::new(-1.0, 0.0, 1.0),
            ang_finger_open:180.0,
            len_finger:		2.0,
            width_finger:	0.5,
            ang_lower_finger:45.0,
            ang_inc_standard:11.25,
            ang_inc_small: 	9.0,
        }
    }

    pub fn draw(&self, state: &GLState) {
        //model to camera matrix stack
        let mut mcs = MatrixStack::new();

        state.program.use_prog();
        gl::BindVertexArray(state.vao);

        mcs.trans(self.pos_base);
        mcs.roty(self.ang_base);

        { // left base
            mcs.push();
            mcs.trans(self.pos_base_left);
            mcs.scale(Vector3::new(1.0, 1.0, self.scale_base_z));
            unsafe{
                state.mod_cam_unif.upload_m4f(&mcs.top());
                //gl::UniformMatrix4fv(state.mod_cam_unif, 1, gl::FALSE, mem::transmute(&mcs.top()));
                gl::DrawElements(gl::TRIANGLES, index_data.len() as i32, gl::UNSIGNED_SHORT, ptr::null());
            }
            mcs.pop();
        }

        { // right base
            mcs.push();
            mcs.trans(self.pos_base_right);
            mcs.scale(Vector3::new(1.0, 1.0, self.scale_base_z));
            unsafe{
                state.mod_cam_unif.upload_m4f(&mcs.top());
                //gl::UniformMatrix4fv(state.mod_cam_unif, 1, gl::FALSE, mem::transmute(&mcs.top()));
                gl::DrawElements(gl::TRIANGLES, index_data.len() as i32, gl::UNSIGNED_SHORT, ptr::null());
            }
            mcs.pop();
        }

        self.draw_upperarm(&mut mcs, state);

        gl::BindVertexArray(0);
        gl::UseProgram(0);
    }

    fn draw_fingers(&self, mcs: &mut MatrixStack, state: &GLState) {
        // draw left finger
        mcs.push();
        mcs.trans(self.pos_left_finger);
        mcs.roty(self.ang_finger_open);

        mcs.push();
        mcs.trans(Vector3::new(0.0, 0.0, self.len_finger / 2.0));
        mcs.scale(Vector3::new(self.width_finger / 2.0, self.width_finger / 2.0, self.len_finger / 2.0));
        unsafe {
            state.mod_cam_unif.upload_m4f(&mcs.top());
            //gl::UniformMatrix4fv(state.mod_cam_unif, 1, gl::FALSE, mem::transmute(&mcs.top()));
            gl::DrawElements(gl::TRIANGLES, index_data.len() as i32, gl::UNSIGNED_SHORT, ptr::null());
        }
        mcs.pop();

        { // draw left lower finger
            mcs.push();
            mcs.trans(Vector3::new(0.0, 0.0, self.len_finger));
            mcs.roty(-self.ang_lower_finger);

            mcs.push();
            mcs.trans(Vector3::new(0.0, 0.0, self.len_finger / 2.0));
            mcs.scale(Vector3::new(self.width_finger / 2.0, self.width_finger / 2.0, self.len_finger / 2.0));
            unsafe {
                state.mod_cam_unif.upload_m4f(&mcs.top());
                //gl::UniformMatrix4fv(state.mod_cam_unif, 1, gl::FALSE, mem::transmute(&mcs.top()));
                gl::DrawElements(gl::TRIANGLES, index_data.len() as i32, gl::UNSIGNED_SHORT, ptr::null());
            }
            mcs.pop();

            mcs.pop();
        }

        mcs.pop();

        // draw right finger
        mcs.push();
        mcs.trans(self.pos_right_finger);
        mcs.roty(-self.ang_finger_open);

        mcs.push();
        mcs.trans(Vector3::new(0.0, 0.0, self.len_finger / 2.0));
        mcs.scale(Vector3::new(self.width_finger / 2.0, self.width_finger / 2.0, self.len_finger / 2.0));
        unsafe {
            state.mod_cam_unif.upload_m4f(&mcs.top());
            //gl::UniformMatrix4fv(state.mod_cam_unif, 1, gl::FALSE, mem::transmute(&mcs.top()));
            gl::DrawElements(gl::TRIANGLES, index_data.len() as i32, gl::UNSIGNED_SHORT, ptr::null());
        }
        mcs.pop();

        { // draw right lower finger
            mcs.push();
            mcs.trans(Vector3::new(0.0, 0.0, self.len_finger));
            mcs.roty(self.ang_lower_finger);

            mcs.push();
            mcs.trans(Vector3::new(0.0, 0.0, self.len_finger / 2.0));
            mcs.scale(Vector3::new(self.width_finger / 2.0, self.width_finger / 2.0, self.len_finger / 2.0));
            unsafe {
                state.mod_cam_unif.upload_m4f(&mcs.top());
                //gl::UniformMatrix4fv(state.mod_cam_unif, 1, gl::FALSE, mem::transmute(&mcs.top()));
                gl::DrawElements(gl::TRIANGLES, index_data.len() as i32, gl::UNSIGNED_SHORT, ptr::null());
            }
            mcs.pop();

            mcs.pop();
        }

        mcs.pop();
    }

    fn draw_wrist(&self, mcs: &mut MatrixStack, state: &GLState) {
        mcs.push();
        mcs.trans(self.pos_wrist);
        mcs.rotz(self.ang_wrist_roll);
        mcs.rotx(self.ang_wrist_pitch);

        mcs.push();
        mcs.scale(Vector3::new(self.width_wrist / 2.0, self.width_wrist / 2.0, self.len_wrist / 2.0));
        unsafe {
            state.mod_cam_unif.upload_m4f(&mcs.top());
            //gl::UniformMatrix4fv(state.mod_cam_unif, 1, gl::FALSE, mem::transmute(&mcs.top()));
            gl::DrawElements(gl::TRIANGLES, index_data.len() as i32, gl::UNSIGNED_SHORT, ptr::null());
        }
        mcs.pop();

        self.draw_fingers(mcs, state);
        mcs.pop();
    }

    fn draw_lowerarm(&self, mcs: &mut MatrixStack, state: &GLState) {
        mcs.push();
        mcs.trans(self.pos_lowerarm);
        mcs.rotx(self.ang_lowerarm);

        mcs.push();
        mcs.trans(Vector3::new(0.0, 0.0, self.len_lowerarm / 2.0));
        mcs.scale(Vector3::new(self.width_lowerarm / 2.0, self.width_lowerarm / 2.0, self.len_lowerarm / 2.0));
        unsafe {
            state.mod_cam_unif.upload_m4f(&mcs.top());
            //gl::UniformMatrix4fv(state.mod_cam_unif, 1, gl::FALSE, mem::transmute(&mcs.top()));
            gl::DrawElements(gl::TRIANGLES, index_data.len() as i32, gl::UNSIGNED_SHORT, ptr::null());
        }
        mcs.pop();

        self.draw_wrist(mcs, state);
        mcs.pop();
    }

    fn draw_upperarm(&self, mcs: &mut MatrixStack, state: &GLState) {
        mcs.push();
        mcs.rotx(self.ang_upperarm);

        {
            mcs.push();
            mcs.trans(Vector3::new(0.0, 0.0, self.size_upperarm / 2.0 - 1.0));
            mcs.scale(Vector3::new(1.0, 1.0, self.size_upperarm / 2.0));
            unsafe {
                state.mod_cam_unif.upload_m4f(&mcs.top());
                //gl::UniformMatrix4fv(state.mod_cam_unif, 1, gl::FALSE, mem::transmute(&mcs.top()));
                gl::DrawElements(gl::TRIANGLES, index_data.len() as i32, gl::UNSIGNED_SHORT, ptr::null());
            }
            mcs.pop();
        }

        self.draw_lowerarm(mcs, state);
        mcs.pop();
    }

    pub fn adj_base(&mut self, inc: bool) {
        self.ang_base += if inc { self.ang_inc_standard } else { -self.ang_inc_standard };
        self.ang_base = util::fmodf(self.ang_base, 360.0);
    }
    pub fn adj_upperarm(&mut self, inc: bool) {
        self.ang_upperarm += if inc { self.ang_inc_standard } else { -self.ang_inc_standard };
        self.ang_upperarm = util::clamp(self.ang_upperarm, -50.0, 0.0);
    }
    pub fn adj_lowerarm(&mut self, inc: bool) {
        self.ang_lowerarm += if inc { self.ang_inc_standard } else { -self.ang_inc_standard };
        self.ang_lowerarm = util::clamp(self.ang_lowerarm, 0.0, 146.25);
    }
    pub fn adj_wrist_pitch(&mut self, inc: bool) {
        self.ang_wrist_pitch += if inc { self.ang_inc_standard } else { -self.ang_inc_standard };
        self.ang_wrist_pitch = util::clamp(self.ang_wrist_pitch, 0.0, 90.0);
    }
    pub fn adj_wrist_roll(&mut self, inc: bool) {
        self.ang_wrist_roll += if inc { self.ang_inc_standard } else { -self.ang_inc_standard };
        self.ang_wrist_roll = util::fmodf(self.ang_wrist_roll, 360.0);
    }
    pub fn adj_finger_open(&mut self, inc: bool) {
        self.ang_finger_open += if inc { self.ang_inc_standard } else { -self.ang_inc_standard };
        self.ang_finger_open = util::clamp(self.ang_finger_open, 9.0, 180.0);
    }

    pub fn write_pose(&self) {
        println!("ang_base:\t{}", self.ang_base);
        println!("ang_upperarm:\t{}", self.ang_upperarm);
        println!("ang_lowerarm:\t{}", self.ang_lowerarm);
        println!("ang_wrist_pitch:\t{}", self.ang_wrist_pitch);
        println!("ang_wrist_roll:\t{}", self.ang_wrist_roll);
        println!("ang_finger_open:\t{}", self.ang_finger_open);
        println!("");
    }
}


fn init() -> GLState {
    let mut state = init_prog();
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

fn display(state: &GLState, win: &glfw::Window, robot: &Hierarchy, m: &Mesh) {
    gl::ClearColor(0.0, 0.0, 0.0, 0.0);
    gl::ClearDepth(1.0);
    gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

    //	let mut ms = MatrixStack::new();
    //	ms.trans(Vector3::new(0.0, 0.0, -20.0));
    //	gl::UseProgram(state.program);
    //	// draw something
    //	gl::BindVertexArray(state.vao);
    //
    //	let m = ms.top();
    //
    //	unsafe { gl::UniformMatrix4fv(state.mod_cam_unif, 1, gl::FALSE, mem::transmute(&m)); }
    //	unsafe { gl::DrawElements(gl::TRIANGLES, index_data.len() as i32, gl::UNSIGNED_SHORT, ptr::null()); }
    //
    //	gl::BindVertexArray(0);
    //	gl::UseProgram(0);

//    robot.draw(state);

    m.render();
    win.swap_buffers();
}

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

fn resize(w: i32, h: i32, state: &mut GLState) {
    println!("resize event: {} x {}", w, h);
    state.cam_clip_m.x.x = state.frustum_scale * (h as f32 / w as f32);
    state.cam_clip_m.y.y = state.frustum_scale;

    state.program.use_prog();
    state.cam_clip_unif.upload_m4f(&state.cam_clip_m);
    gl::UseProgram(0);

    gl::Viewport(0, 0, w as GLsizei, h as GLsizei);
}

#[start]
fn start(argc: int, argv: *const *const u8) -> int {
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
    state.print();

    let mut robot = Hierarchy::new();
    let mut depth_clamp = false;

    //let m = mesh::Mesh::new("s/mesh/UnitCubeTint.xml");
    let unit_plane = mesh::unit_plane();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => { window.set_should_close(true); }
                //glfw::KeyEvent(glfw::KeyQ, _, glfw::Press, _)      => { window.set_should_close(true); }
                glfw::KeyEvent(glfw::KeyL, _, glfw::Press, _)      => {
                    if depth_clamp {
                        gl::Disable(gl::DEPTH_CLAMP);
                    } else {
                        gl::Enable(gl::DEPTH_CLAMP);
                    }
                    depth_clamp = !depth_clamp;
                }
                glfw::SizeEvent(w, h) => { resize(w, h, &mut state); }
                _ => { robot_kb(&mut robot, event); }
            }
        }

        display(&state, &window, &robot, &unit_plane);
    }

    state.delete();
}
