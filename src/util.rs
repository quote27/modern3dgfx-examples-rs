extern crate gl;
extern crate cgmath;

use gl::types::*;
use cgmath::matrix::Matrix4;
use cgmath::vector::{Vector3, Vector4};

pub fn print_mat(m: &Matrix4<f32>) {
    println!("{: >8.2}{: >8.2}{: >8.2}{: >8.2}", m.x.x, m.y.x, m.z.x, m.w.x);
    println!("{: >8.2}{: >8.2}{: >8.2}{: >8.2}", m.x.y, m.y.y, m.z.y, m.w.y);
    println!("{: >8.2}{: >8.2}{: >8.2}{: >8.2}", m.x.z, m.y.z, m.z.z, m.w.z);
    println!("{: >8.2}{: >8.2}{: >8.2}{: >8.2}", m.x.w, m.y.w, m.z.w, m.w.w);
    println!("{}", m)
}

fn gen_rotx(ang: f32) -> Matrix4<f32> {
    let rad = ang.to_radians();
    let (cos, sin) = (rad.cos(), rad.sin());

    let mut m: Matrix4<f32> = Matrix4::identity();
    m.y.y = cos; m.z.y =-sin;
    m.y.z = sin; m.z.z = cos;
    m
}

fn gen_roty(ang: f32) -> Matrix4<f32> {
    let rad = ang.to_radians();
    let (cos, sin) = (rad.cos(), rad.sin());

    let mut m: Matrix4<f32> = Matrix4::identity();
    m.x.x = cos; m.z.x = sin;
    m.x.z =-sin; m.z.z = cos;
    m
}

fn gen_rotz(ang: f32) -> Matrix4<f32> {
    let rad = ang.to_radians();
    let (cos, sin) = (rad.cos(), rad.sin());

    let mut m: Matrix4<f32> = Matrix4::identity();
    m.x.x = cos; m.y.x =-sin;
    m.x.y = sin; m.y.y = cos;
    m
}

#[inline(always)]
pub fn fmodf(a: f32, n: f32) -> f32 {
    a - (n * ((a/n) as i32) as f32)
}

#[inline(always)]
pub fn clamp(v: f32, min: f32, max: f32) -> f32 {
    if v < min {
        min
    } else if v > max {
        max
    } else {
        v
    }
}

#[inline(always)]
pub fn vec3to4<T>(v3: Vector3<T>, w: T) -> Vector4<T> {
    Vector4::new(v3.x, v3.y, v3.z, w)
}

#[inline(always)]
pub fn vlength(v: &Vector3<f32>) -> f32 {
    (v.x*v.x + v.y*v.y + v.z*v.z).sqrt()
}

pub fn calc_frustum_scale(fov_deg: f32) -> f32 {
    let deg_rad = 3.14159 * 2.0 / 360.0;
    let fov_rad = fov_deg * deg_rad;
    1.0 / (fov_rad / 2.0).tan()
}


pub struct MatrixStack {
    c: Matrix4<f32>,
    s: Vec<Matrix4<f32>>,
}

impl MatrixStack {
    pub fn new() -> MatrixStack {
        MatrixStack {
            c: Matrix4::identity(),
            s: vec!(Matrix4::identity()),
        }
    }

    pub fn from_mat(m: &Matrix4<f32>) -> MatrixStack {
        MatrixStack {
            c: *m,
            s: vec!(*m),
        }
    }

    pub fn perspective(fov: f32, aspect_ratio: f32, znear: f32, zfar: f32) -> MatrixStack {
        let mut m: Matrix4<f32> = Matrix4::zero();
        let fs = calc_frustum_scale(fov);
        m.x.x = fs / aspect_ratio;
        m.y.y = fs;
        m.z.z = (zfar + znear) / (znear - zfar);
        m.z.w = -1.0;
        m.w.z = (2.0 * zfar * znear) / (znear - zfar);

        MatrixStack {
            c: m,
            s: vec!(m),
        }
    }

    #[inline(always)]
    pub fn top(&self) -> Matrix4<f32> {
        self.c
    }

    #[inline(always)]
    pub fn rotx(&mut self, deg: f32) {
        self.c = self.c * gen_rotx(deg);
    }

    #[inline(always)]
    pub fn roty(&mut self, deg: f32) {
        self.c = self.c * gen_roty(deg);
    }

    #[inline(always)]
    pub fn rotz(&mut self, deg: f32) {
        self.c = self.c * gen_rotz(deg);
    }

    pub fn scale(&mut self, sv: Vector3<f32>) {
        let mut m: Matrix4<f32> = Matrix4::identity();
        m.x.x = sv.x;
        m.y.y = sv.y;
        m.z.z = sv.z;
        self.c = self.c * m;
    }

    pub fn trans(&mut self, sv: Vector3<f32>) {
        let mut m: Matrix4<f32> = Matrix4::identity();
        m.w = vec3to4(sv, 1.0);
        self.c = self.c * m;
    }

    #[inline(always)]
    pub fn mul(&mut self, m: Matrix4<f32>) {
        self.c = self.c * m;
    }

    #[inline(always)]
    pub fn push(&mut self) {
        self.s.push(self.c);
    }

    #[inline(always)]
    pub fn pop(&mut self) {
        self.c = match self.s.pop() {
            Some(x) => x,
            None => Matrix4::identity(),
        }
    }
}

pub fn glenum_name(t: GLenum) -> &'static str {
    match t {
        gl::TRIANGLES      => "triangles",
        gl::TRIANGLE_STRIP => "tri-strip",
        gl::TRIANGLE_FAN   => "tri-fan",
        gl::LINES          => "lines",
        gl::LINE_STRIP     => "line-strip",
        gl::LINE_LOOP      => "line-loop",
        gl::POINTS         => "points",

        gl::FLOAT          => "float",
        gl::HALF_FLOAT     => "half",
        gl::INT            => "int",
        gl::UNSIGNED_INT   => "uint",
        gl::SHORT          => "short",
        gl::UNSIGNED_SHORT => "ushort",
        gl::BYTE           => "byte",

        gl::ARRAY_BUFFER   => "array_buffer",
        gl::ELEMENT_ARRAY_BUFFER => "element_array_buffer",

        gl::VERTEX_SHADER => "vertex_shader",
        gl::FRAGMENT_SHADER => "fragment_shader",
        _                  => "unknown",
    }
}

