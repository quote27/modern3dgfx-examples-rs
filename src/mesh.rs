extern crate gl;
extern crate glfw;
extern crate sax;

use gl::types::*;
use std::ptr;
use std::str;
use std::mem;
use std::collections::TreeMap;
use std::vec;
use std::from_str::FromStr;

//typedef std::map<std::string, GLuint> VAOMap;
//typedev VAOmap::value_ptr VAOMapData
struct MeshData {
	attrib_array_buf: GLuint,
	idx_buf: GLuint,
	vao: GLuint,

	named_vaos: TreeMap<String, GLuint>,
	primatives: Vec<RenderCmd>,
}

pub struct Mesh {
	data: Box<MeshData>,
}

struct RenderCmd {
	idx_cmd: bool,
	prim_type: GLenum,
	start: GLuint,
	elem_count: GLint,
	idx_data_type: GLenum, // only if idx_cmd is true
	prim_restart: int, // only if idx_cmd is true
}

impl RenderCmd {
	pub fn nwe() -> RenderCmd {
		RenderCmd { idx_cmd: true, prim_type: gl::TRIANGLES, start: 0, elem_count: 0, idx_data_type: gl::UNSIGNED_SHORT, prim_restart: 0 }
	}
	pub fn render(&self) {
		if self.idx_cmd {
			unsafe { gl::DrawElements(self.prim_type, self.elem_count, self.idx_data_type, mem::transmute(self.start as u64)) }
		} else {
			unsafe { gl::DrawArrays(self.prim_type, mem::transmute(self.start), self.elem_count) }
		}
	}
}

pub enum AttribData {
	FValue(f32),
	UIValue(GLuint),
	IValue(GLint),
	USValue(GLushort),
	SValue(GLshort),
	UBValue(GLubyte),
	BValue(GLbyte),
}

struct PrimitiveType {
	name: &'static str,
	prim_type: GLenum,
}

static all_prim_types: [PrimitiveType, ..1] = [
	PrimitiveType { name: "float", prim_type: gl::FLOAT },
];


fn parse_floats(output_data: &mut Vec<AttribData>, input_data: &[String]) { }
fn write_floats(buff: GLenum, data: &Vec<AttribData>, size: int, offset: uint) { }

fn parse_ints(output_data: &mut Vec<AttribData>, input_data: &[String]) { }
fn write_ints(buff: GLenum, data: &Vec<AttribData>, size: int, offset: uint) { }

fn parse_uints(output_data: &mut Vec<AttribData>, input_data: &[String]) { }
fn write_uints(buff: GLenum, data: &Vec<AttribData>, size: int, offset: uint) { }

fn parse_shorts(output_data: &mut Vec<AttribData>, input_data: &[String]) { }
fn write_shorts(buff: GLenum, data: &Vec<AttribData>, size: int, offset: uint) { }

fn parse_ushorts(output_data: &mut Vec<AttribData>, input_data: &[String]) { }
fn write_ushorts(buff: GLenum, data: &Vec<AttribData>, size: int, offset: uint) { }

fn parse_bytes(output_data: &mut Vec<AttribData>, input_data: &[String]) { }
fn write_bytes(buff: GLenum, data: &Vec<AttribData>, size: int, offset: uint) { }

fn parse_ubytes(output_data: &mut Vec<AttribData>, input_data: &[String]) { }
fn write_ubytes(buff: GLenum, data: &Vec<AttribData>, size: int, offset: uint) { }

struct AttribType {
	name: &'static str,
	normalized: bool,
	gl_type: GLenum,
	num_bytes: int,

	parse: fn(&mut Vec<AttribData>, &[String]),
	write_buf: fn(GLenum, &Vec<AttribData>, int, uint),
}

impl AttribType {
	pub fn gen_all() -> [AttribType, ..14] {
		[
		AttribType { name: "float",       normalized: false, gl_type: gl::FLOAT,          num_bytes: mem::size_of::<GLfloat>()   as int, parse: parse_floats,  write_buf: write_floats},
		AttribType { name: "half",        normalized: false, gl_type: gl::HALF_FLOAT,     num_bytes: mem::size_of::<GLhalfARB>() as int, parse: parse_floats,  write_buf: write_floats},
		AttribType { name: "int",         normalized: false, gl_type: gl::INT,            num_bytes: mem::size_of::<GLint>()     as int, parse: parse_ints,    write_buf: write_ints},
		AttribType { name: "uint",        normalized: false, gl_type: gl::UNSIGNED_INT,   num_bytes: mem::size_of::<GLuint>()    as int, parse: parse_uints,   write_buf: write_uints},
		AttribType { name: "norm-int",    normalized: true,  gl_type: gl::INT,            num_bytes: mem::size_of::<GLint>()     as int, parse: parse_ints,    write_buf: write_ints},
		AttribType { name: "norm-uint",   normalized: true,  gl_type: gl::UNSIGNED_INT,   num_bytes: mem::size_of::<GLuint>()    as int, parse: parse_uints,   write_buf: write_uints},
		AttribType { name: "short",       normalized: false, gl_type: gl::SHORT,          num_bytes: mem::size_of::<GLshort>()   as int, parse: parse_shorts,  write_buf: write_shorts},
		AttribType { name: "ushort",      normalized: false, gl_type: gl::UNSIGNED_SHORT, num_bytes: mem::size_of::<GLushort>()  as int, parse: parse_ushorts, write_buf: write_ushorts},
		AttribType { name: "norm-short",  normalized: true,  gl_type: gl::SHORT,          num_bytes: mem::size_of::<GLshort>()   as int, parse: parse_shorts,  write_buf: write_shorts},
		AttribType { name: "norm-ushort", normalized: true,  gl_type: gl::UNSIGNED_SHORT, num_bytes: mem::size_of::<GLushort>()  as int, parse: parse_ushorts, write_buf: write_ushorts},
		AttribType { name: "byte",        normalized: false, gl_type: gl::BYTE,           num_bytes: mem::size_of::<GLbyte>()    as int, parse: parse_bytes,   write_buf: write_bytes},
		AttribType { name: "ubyte",       normalized: false, gl_type: gl::UNSIGNED_BYTE,  num_bytes: mem::size_of::<GLubyte>()   as int, parse: parse_ubytes,  write_buf: write_ubytes},
		AttribType { name: "norm-byte",   normalized: true,  gl_type: gl::BYTE,           num_bytes: mem::size_of::<GLbyte>()    as int, parse: parse_bytes,   write_buf: write_bytes},
		AttribType { name: "norm-ubyte",  normalized: true,  gl_type: gl::UNSIGNED_BYTE,  num_bytes: mem::size_of::<GLubyte>()   as int, parse: parse_ubytes,  write_buf: write_ubytes},
		]
	}
	
	pub fn new() -> AttribType { //not sure if this is the best way to handle
		AttribType { name: "float",   normalized: false, gl_type: gl::FLOAT,          num_bytes: mem::size_of::<GLfloat>()   as int, parse: parse_floats,  write_buf: write_floats}
	}
}

impl FromStr for AttribType {
	fn from_str(s: &str) -> Option<AttribType> {

		// going to return a Some(AttribType {...})
		None
	}
}


struct Attribute {
	attrib_idx: GLuint,
	attrib_type: Option<AttribType>,
	size: int,
	is_integral: bool,
	data_array: Vec<AttribData>,
}

impl Attribute {
	pub fn new() -> Attribute {
		Attribute {
			attrib_idx: 0xffffffff,
			attrib_type: None,
			size: -1,
			is_integral: false,
			data_array: Vec::new()
		}
	}

	pub fn from_xml(tag_name: &str, attr: sax::Attributes) -> Attribute {
		let attrib_idx: GLuint = match from_str(attr.get("index")) {
			None => { println!("{} missing attribute: index", tag_name); 0 },
			Some(x) => x,
		};

		let size: int = match from_str(attr.get("size")) {
			None => { println!("{} missing attribute: size", tag_name); -1 },
			Some(x) => x,
		};

		let attrib_type: AttribType = match from_str(attr.get("type")) {
			None => { println!("{} missing attribute: type", tag_name); AttribType::new() },
			Some(x) => x,
		};

		Attribute {
			attrib_idx: attrib_idx,
			attrib_type: None,
			size: -1,
			is_integral: false,
			data_array: Vec::new()
		}
	}
}


struct IndexData {
	a: int,
}






impl Mesh {
	pub fn new(file_src: &str) -> Mesh {
		let mut attribs: Vec<Attribute> = Vec::with_capacity(16);
		let mut attrib_idx_map: TreeMap<GLuint, uint> = TreeMap::new();
		let mut idx_data: Vec<IndexData> = Vec::new();
		let mut named_vao_list: Vec<(String, Vec<GLuint>)> = Vec::new();

		{
			use std::io::File;
			let path = Path::new(file_src);
			let parser = sax::parse_file(&path).ok().expect("issue loading mesh");;

			for result in parser.iter() {
				match result {
					Ok(sax::StartDocument) => ( println!("doc start") ),
					Ok(sax::EndDocument) => ( println!("doc end") ),
					Ok(event) => {
						match event {
							sax::StartElement(e, attr) => ( println!("start: {} {}", e, attr) ),
							sax::EndElement(e) => ( println!("end: {}", e) ),
							sax::Characters(c) => ( println!("char: {}", c) ),
							_ => (),
						}
					},
					Err(err) => println!("{}", err),
				}
			}
		}

		let m = MeshData {
			attrib_array_buf: 0,
			idx_buf: 0,
			vao: 0,
			named_vaos: TreeMap::new(),
			primatives: Vec::new(),
		};

		Mesh {
			data: box m,
		}
	}
}



// 
// pub fn load_file(file_src: &str) -> String {
// 	use std::io::File;
// 
// 	let path = Path::new(file_src);
// 	let mut f = File::open(&path);
// 
// 	let dat = f.read_to_str().ok().expect(format!("unable to open {} file", file_src));
// 
// 	println!("reading {0} file:\n{1}\n---", file_src, dat);
// 
// 	dat
// }
// 
// pub fn load_shader_file(ty: GLenum, file_src: &str) -> GLuint {
// 	load_shader(ty, load_file(file_src))
// }
// 
// //TODO: write file load logic using http://static.rust-lang.org/doc/master/std/io/fs/struct.File.html
// pub fn load_shader(ty: GLenum, src: String) -> GLuint {
// 	let shader = gl::CreateShader(ty);
// 	unsafe {
// 		// Attempt to compile the shader
// 		src.with_c_str(|ptr| gl::ShaderSource(shader, 1, &ptr, ptr::null()));
// 		gl::CompileShader(shader);
// 
// 		// Get the compile status
// 		let mut status = gl::FALSE as GLint;
// 		gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
// 
// 		// Fail on error
// 		if status != (gl::TRUE as GLint) {
// 			let mut len = 0;
// 			gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
// 			let mut buf = Vec::from_elem(len as uint - 1, 0u8);     // subtract 1 to skip the trailing null character
// 			gl::GetShaderInfoLog(shader, len, ptr::mut_null(), buf.as_mut_ptr() as *mut GLchar);
// 			fail!("{}", str::from_utf8(buf.as_slice()).expect("ShaderInfoLog not valid utf8"));
// 		}
// 	}
// 	shader
// }
// 
// pub fn create_program(shader_list: &Vec<GLuint>) -> GLuint {
// 	let prog = gl::CreateProgram();
// 	for s in shader_list.iter() {
// 		gl::AttachShader(prog, *s);
// 	}
// 	gl::LinkProgram(prog);
// 	unsafe {
// 		// Get the link status
// 		let mut status = gl::FALSE as GLint;
// 		gl::GetProgramiv(prog, gl::LINK_STATUS, &mut status);
// 
// 		// Fail on error
// 		if status != (gl::TRUE as GLint) {
// 			let mut len: GLint = 0;
// 			gl::GetProgramiv(prog, gl::INFO_LOG_LENGTH, &mut len);
// 			let mut buf = Vec::from_elem(len as uint - 1, 0u8);     // subtract 1 to skip the trailing null character
// 			gl::GetProgramInfoLog(prog, len, ptr::mut_null(), buf.as_mut_ptr() as *mut GLchar);
// 			fail!("{}", str::from_utf8(buf.as_slice()).expect("ProgramInfoLog not valid utf8"));
// 		}
// 	}
// 	prog
// }
