extern crate gl;
extern crate glfw;
//extern crate sax;

use gl::types::*;
use std::ptr;
use std::mem;
use std::collections::TreeMap;
use std::fmt::{Show, Formatter, FormatError};
use util::*;


struct RenderCmd {
    idx_cmd: bool,
    prim_type: GLenum,
    start: GLuint,
    elem_count: GLint,
    idx_data_type: GLenum, // only if idx_cmd is true
}

impl RenderCmd {
    pub fn render(&self) {
        if self.idx_cmd {
            println!("rendering indexed command: {}", self);
            unsafe { gl::DrawElements(self.prim_type, self.elem_count, self.idx_data_type, mem::transmute(self.start as GLsizeiptr)) }
        } else {
            println!("rendering array: {}", self);
            unsafe { gl::DrawArrays(self.prim_type, mem::transmute(self.start), self.elem_count) }
        }
    }
}

impl Show for RenderCmd {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        let primtype = glenum_name(self.prim_type);
        let idxtype = glenum_name(self.idx_data_type);
        f.write(format!("idx_cmd? {}, prim_type: {}, start: {}, elem_count: {}, idx_data_type: {}", self.idx_cmd, primtype, self.start, self.elem_count, idxtype).as_bytes())
    }
}


pub enum AttribData {
    FValue(GLfloat),
    UIValue(GLuint),
    IValue(GLint),
    USValue(GLushort),
    SValue(GLshort),
    UBValue(GLubyte),
    BValue(GLbyte),
}

impl AttribData {
    pub fn floatv(v: Vec<GLfloat>) -> Vec<AttribData> {
        let mut fv: Vec<AttribData> = Vec::with_capacity(v.len());
        for e in v.iter() { fv.push(FValue(*e)); }
        fv
    }

    pub fn ushortv(v: Vec<GLushort>) -> Vec<AttribData> {
        let mut uv: Vec<AttribData> = Vec::with_capacity(v.len());
        for e in v.iter() { uv.push(USValue(*e)); }
        uv
    }
}

impl Show for AttribData {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        match *self {
            FValue(x) => f.write(format!("{}f", x).as_bytes()),
            USValue(x) => f.write(format!("{}us", x).as_bytes()),
            _ => f.write(String::from_str("").as_bytes()),
        }
    }
}


// initial function definition: fn $fn_name(buff: GLenum, data: &Vec<AttribData>, size: int, offset: uint) {
// however, 'size' is not used in the fn, removing it for now
macro_rules! write_array_fn_def(
    ($fn_name:ident, $gl_type:ty, $attr_enum:ident) => {
        fn $fn_name(buff: GLenum, data: &Vec<AttribData>, offset: uint) {
            let mut tmp_buff: Vec<$gl_type> = Vec::with_capacity(data.len());
            for i in data.iter() {
                tmp_buff.push(match *i {
                    $attr_enum(x) => x,
                    _ => fail!("parse attrib data - wrong type"),
                });
            }
            println!("[attribute] buffer_sub_data: buff type: {}, attribdata: {}, offset: {}, len: {}={}, memsize: {}, tmp_buff: {}", glenum_name(buff), data, offset, data.len(), tmp_buff.len(), tmp_buff.len() * mem::size_of::<$gl_type>(), tmp_buff);
            unsafe { gl::BufferSubData(buff, offset as GLintptr, (tmp_buff.len() * mem::size_of::<$gl_type>()) as GLsizeiptr, mem::transmute(&tmp_buff.as_slice()[0])); }
        }
    }
)

//TODO: write these parse functions - the write functions are to push to gl::BufferSubData
//fn parse_floats(output_data: &mut Vec<AttribData>, input_data: &[String]) { }
write_array_fn_def!(write_floats, GLfloat, FValue)
//fn parse_ints(output_data: &mut Vec<AttribData>, input_data: &[String]) { }
//write_array_fn_def!(write_ints, GLint, IValue)
//fn parse_uints(output_data: &mut Vec<AttribData>, input_data: &[String]) { }
//write_array_fn_def!(write_uints, GLuint, UIValue)
//fn parse_shorts(output_data: &mut Vec<AttribData>, input_data: &[String]) { }
//write_array_fn_def!(write_shorts, GLshort, SValue)
//fn parse_ushorts(output_data: &mut Vec<AttribData>, input_data: &[String]) { }
write_array_fn_def!(write_ushorts, GLushort, USValue)
//fn parse_bytes(output_data: &mut Vec<AttribData>, input_data: &[String]) { }
//write_array_fn_def!(write_bytes, GLbyte, BValue)
//fn parse_ubytes(output_data: &mut Vec<AttribData>, input_data: &[String]) { }
//write_array_fn_def!(write_ubytes, GLubyte, UBValue)

struct AttribType {
    name: &'static str,
    normalized: bool,
    gl_type: GLenum,
    num_bytes: int,

    //parse: fn(&mut Vec<AttribData>, &[String]),
    write_buf: fn(GLenum, &Vec<AttribData>, uint),
}

impl AttribType {
    // TODO: figure out the best way to do a 'global' list of attrib types - can't do static list due to mem::size_of unfortunately
//    pub fn gen_all() -> [AttribType, ..14] {
//        [
//            AttribType { name: "float",       normalized: false, gl_type: gl::FLOAT,          num_bytes: mem::size_of::<GLfloat>()   as int, parse: parse_floats,  write_buf: write_floats},
//            AttribType { name: "half",        normalized: false, gl_type: gl::HALF_FLOAT,     num_bytes: mem::size_of::<GLhalfARB>() as int, parse: parse_floats,  write_buf: write_floats},
//            AttribType { name: "int",         normalized: false, gl_type: gl::INT,            num_bytes: mem::size_of::<GLint>()     as int, parse: parse_ints,    write_buf: write_ints},
//            AttribType { name: "uint",        normalized: false, gl_type: gl::UNSIGNED_INT,   num_bytes: mem::size_of::<GLuint>()    as int, parse: parse_uints,   write_buf: write_uints},
//            AttribType { name: "norm-int",    normalized: true,  gl_type: gl::INT,            num_bytes: mem::size_of::<GLint>()     as int, parse: parse_ints,    write_buf: write_ints},
//            AttribType { name: "norm-uint",   normalized: true,  gl_type: gl::UNSIGNED_INT,   num_bytes: mem::size_of::<GLuint>()    as int, parse: parse_uints,   write_buf: write_uints},
//            AttribType { name: "short",       normalized: false, gl_type: gl::SHORT,          num_bytes: mem::size_of::<GLshort>()   as int, parse: parse_shorts,  write_buf: write_shorts},
//            AttribType { name: "ushort",      normalized: false, gl_type: gl::UNSIGNED_SHORT, num_bytes: mem::size_of::<GLushort>()  as int, parse: parse_ushorts, write_buf: write_ushorts},
//            AttribType { name: "norm-short",  normalized: true,  gl_type: gl::SHORT,          num_bytes: mem::size_of::<GLshort>()   as int, parse: parse_shorts,  write_buf: write_shorts},
//            AttribType { name: "norm-ushort", normalized: true,  gl_type: gl::UNSIGNED_SHORT, num_bytes: mem::size_of::<GLushort>()  as int, parse: parse_ushorts, write_buf: write_ushorts},
//            AttribType { name: "byte",        normalized: false, gl_type: gl::BYTE,           num_bytes: mem::size_of::<GLbyte>()    as int, parse: parse_bytes,   write_buf: write_bytes},
//            AttribType { name: "ubyte",       normalized: false, gl_type: gl::UNSIGNED_BYTE,  num_bytes: mem::size_of::<GLubyte>()   as int, parse: parse_ubytes,  write_buf: write_ubytes},
//            AttribType { name: "norm-byte",   normalized: true,  gl_type: gl::BYTE,           num_bytes: mem::size_of::<GLbyte>()    as int, parse: parse_bytes,   write_buf: write_bytes},
//            AttribType { name: "norm-ubyte",  normalized: true,  gl_type: gl::UNSIGNED_BYTE,  num_bytes: mem::size_of::<GLubyte>()   as int, parse: parse_ubytes,  write_buf: write_ubytes}
//        ]
//    }

    pub fn float() -> AttribType {
        AttribType { name: "float",       normalized: false, gl_type: gl::FLOAT,          num_bytes: mem::size_of::<GLfloat>()   as int, /*parse: parse_floats,*/  write_buf: write_floats}
    }

    pub fn ushort() -> AttribType {
        AttribType { name: "ushort",      normalized: false, gl_type: gl::UNSIGNED_SHORT, num_bytes: mem::size_of::<GLushort>()  as int, /*parse: parse_ushorts,*/ write_buf: write_ushorts}
    }
}

impl Show for AttribType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        f.write(format!("{}", self.name).as_bytes())
    }
}


struct Attribute {
    idx: GLuint,
    ty: Option<AttribType>,
    size: int,
    is_integral: bool,
    data_array: Vec<AttribData>, //TODO: copy behavior?
}

impl Attribute {
    pub fn num_elem(&self) -> uint {
        self.data_array.len() / self.size as uint
    }

    pub fn calc_byte_size(&self) -> uint {
        self.data_array.len() * self.ty.unwrap().num_bytes as uint
    }

    pub fn fill_bound_bo(&self, offset: uint) {
        println!("[attribute] fill_bound_bo: offset: {}, data_array.len(): {}", offset, self.data_array.len());
        let f = self.ty.unwrap().write_buf;
        f(gl::ARRAY_BUFFER, &self.data_array, offset);
    }

    pub fn setup_attrib_array(&self, offset: uint) {
        println!("[attribute] setup_attrib_array: attrib array idx: {}, is_integral: {}, offset: {}", self.idx, self.is_integral, offset);
        gl::EnableVertexAttribArray(self.idx);
        if self.is_integral {
            unsafe { gl::VertexAttribIPointer(self.idx, self.size as GLint, self.ty.unwrap().gl_type, 0, mem::transmute(offset)); }
        } else {
            unsafe { gl::VertexAttribPointer(self.idx, self.size as GLint, self.ty.unwrap().gl_type, if self.ty.unwrap().normalized { gl::TRUE } else { gl::FALSE }, 0, mem::transmute(offset)); }
        }
    }
}

impl Show for Attribute {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        f.write(format!("idx: {}, type: {}, size: {}, integral? {}, data: {}", self.idx, self.ty.unwrap(), self.size, self.is_integral, self.data_array).as_bytes())
    }
}


struct IndexData {
    ty: Option<AttribType>,
    data_array: Vec<AttribData>,
}

impl IndexData {
    pub fn calc_byte_size(&self) -> uint {
        self.data_array.len() * self.ty.unwrap().num_bytes as uint
    }

    pub fn fill_bound_bo(&self, offset: uint) {
        println!("[indexdata] fill_bound_bo: offset: {}, data_array.len: {}", offset, self.data_array.len());
        let f = self.ty.unwrap().write_buf;
        f(gl::ARRAY_BUFFER, &self.data_array, offset);
    }
}

impl Show for IndexData {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        f.write(format!("type: {}, data: {}", self.ty.unwrap(), self.data_array).as_bytes())
    }
}


struct MeshData {
    attrib_array_buff: GLuint,
    idx_buff: GLuint,
    vao: GLuint,

    named_vaos: TreeMap<String, GLuint>,
    primatives: Vec<RenderCmd>,
}

impl MeshData {
    pub fn new() -> MeshData {
        MeshData {
            attrib_array_buff: 0,
            idx_buff: 0,
            vao: 0,
            named_vaos: TreeMap::new(),
            primatives: Vec::new()
        }
    }
}

impl Show for MeshData {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        f.write(format!("attrib_array_buff: {}, idx_buff: {}, vao: {}, named_vaos: {}, primatives: {}", self.attrib_array_buff, self.idx_buff, self.vao, self.named_vaos, self.primatives).as_bytes())
    }
}


pub struct Mesh {
    data: MeshData,
}

impl Mesh {
    pub fn new(attribs: Vec<Attribute>, idx_data: Vec<IndexData>, prims: Vec<RenderCmd>) -> Mesh {
        println!("\n== creating the mesh ==");
        println!("[mesh] attribs: {}", attribs);
        println!("[mesh] idx_data: {}", idx_data);
        println!("[mesh] prims: {}", prims);
        // assuming hard coded, skipping file parse logic
        // let mut attribs: Vec<Attribute> = Vec::with_capacity(16);
        // let mut idx_data: Vec<IndexData> = Vec::new();
        // let mut named_vao_list: Vec<(String, Vec<GLuint>)> = Vec::new(); -- TODO: dont need this for ch7

        let mut attrib_buff_size = 0;
        let mut attrib_start_locs: Vec<uint> = Vec::with_capacity(attribs.len());
        let mut num_elem = 0;

        // figure out buffer size for attribute data needed
        println!("[mesh] counting attrib buffer start locations for {} attrib(s)", attribs.len());
        for i in attribs.iter() {
            attrib_buff_size = if attrib_buff_size % 16 == 0 {
                attrib_buff_size
            } else {
                attrib_buff_size + (16 - attrib_buff_size % 16)
            };
            attrib_start_locs.push(attrib_buff_size);
            let attr = &(*i);
            attrib_buff_size += attr.calc_byte_size();
            if num_elem != 0 {
                if num_elem != attr.num_elem() {
                    fail!("some attribute arrays have different lengths");
                } else {
                    num_elem = attr.num_elem();
                }
            }
        }
        println!("[mesh] attrib_start_locs: {}", attrib_start_locs);
        println!("[mesh] atrib_buff_size: {}", attrib_buff_size);

        let mut md = MeshData::new();
        md.primatives = prims;

        // create 'everything' vao
        unsafe { gl::GenVertexArrays(1, &mut md.vao); }
        gl::BindVertexArray(md.vao);
        println!("[mesh] vao: {}", md.vao);

        // create the buffer object
        unsafe {
            gl::GenBuffers(1, &mut md.attrib_array_buff);
            gl::BindBuffer(gl::ARRAY_BUFFER, md.attrib_array_buff);
            gl::BufferData(gl::ARRAY_BUFFER, attrib_buff_size as GLsizeiptr, ptr::null(), gl::STATIC_DRAW);
        }
        println!("[mesh] md.attrib_array_buff: {}", md.attrib_array_buff);

        // fill in data and set up the attribute arrays
        println!("[mesh] fill bound buffer objects for {} attrib(s)", attribs.len());
        for i in range(0, attribs.len()) {
            let attr = &attribs.get(i);
            println!("[mesh] fill_buffer_obj: attribute: {}", attr);
            attr.fill_bound_bo(*attrib_start_locs.get(i));
            attr.setup_attrib_array(*attrib_start_locs.get(i));
        }

        // fill the named vaos
        //println!("filling the named vaos");
        //for named_vao in named_vao_list.iter() {
        //    let (ref name, ref attrib_list) = *named_vao;

        //    let mut vao = -1;
        //    unsafe { gl::GenVertexArrays(1, &mut vao); }
        //    gl::BindVertexArray(vao); //bind the vao

        //    // setup the attributes for this vao
        //    for attr_id in attrib_list.iter() {
        //        //let mut attr_off = -1;
        //        for i in range(0, attribs.len()) {
        //            let a = attribs.get(i);
        //            if a.idx == *attr_id {
        //                a.setup_attrib_array(*attrib_start_locs.get(i));
        //                //attr_off = i;
        //                break;
        //            }
        //        }
        //        //attribs.get(attr_off).setup_attrib_array(*attrib_start_locs.get(attr_off));
        //    }
        //    md.named_vaos.insert(name.clone(), vao); //TODO: not sure if this is right, just copying a string, should be fine(?)
        //}
        gl::BindVertexArray(0);
        println!("[mesh] unbinding vertex array");
        println!("[mesh] finished going through attributes");

        // get the size of the index buffer data
        let mut idx_buff_size = 0;
        let mut idx_start_locs: Vec<uint> = Vec::with_capacity(idx_data.len());

        println!("[mesh] counting index buffer start locations for {} indexdata(s)", idx_data.len());
        for i in idx_data.iter() {
            idx_buff_size = if idx_buff_size % 16 == 0 {
                idx_buff_size
            } else {
                idx_buff_size + (16 - idx_buff_size % 16)
            };
            idx_start_locs.push(idx_buff_size);
            idx_buff_size += i.calc_byte_size();
        }
        println!("[mesh] idx_start_locs: {}", idx_start_locs);
        println!("[mesh] idx_buff_size: {}", idx_buff_size);

        // create index buffer object
        if idx_buff_size > 0 {
            println!("[mesh] idx_buffobj: creating index buffer object");
            println!("[mesh] idx_buffobj: binding vao: {}", md.vao);
            gl::BindVertexArray(md.vao);

            unsafe {
                gl::GenBuffers(1, &mut md.idx_buff);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, md.idx_buff);
                gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, idx_buff_size as GLsizeiptr, ptr::null(), gl::STATIC_DRAW);
            }
            println!("[mesh] idx_buffobj: md.idx_buff: {}", md.idx_buff);

            // fill with data
            println!("[mesh] idx_buffobj: fill bound buffer objects for {} index_data(s)", idx_data.len());
            for i in range(0, idx_data.len()) {
                let idx = idx_data.get(i);
                println!("[mesh] fill_buffer_obj: index_data: {}",idx);
                idx.fill_bound_bo(*idx_start_locs.get(i));
            }

            // fill in indexed rendering commands
            let mut curr_idx = 0;
            println!("[mesh] idx_buffobj: fill in indexed rendering commands for {} primatives", md.primatives.len());
            for prim in md.primatives.mut_iter() {
                if prim.idx_cmd {
                    println!("[mesh] idx_buffobj: rendercmd: updating primative index command. curr_idx: {}", curr_idx);
                    println!("[mesh] idx_buffobj: rendercmd: current index: {}", idx_data.get(curr_idx));
                    println!("[mesh] idx_buffobj: rendercmd: before: {}", prim);
                    prim.start = *idx_start_locs.get(curr_idx) as GLuint;
                    prim.elem_count = idx_data.get(curr_idx).data_array.len() as GLint;
                    prim.idx_data_type = idx_data.get(curr_idx).ty.unwrap().gl_type;
                    println!("[mesh] idx_buffobj: rendercmd: after : {}", prim);
                    curr_idx+=1;
                }
            }

            for named_vao in md.named_vaos.iter() {
                let (_, vao) = named_vao;
                gl::BindVertexArray(*vao);
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, md.idx_buff);
            }
            gl::BindVertexArray(0);
        } else {
            println!("[mesh] no index buffer(s)");
        }

        println!("[mesh] finished generating: {}", md);

        Mesh {
            data: md
        }
    }

    pub fn render(&self) {
        if self.data.vao == 0 {
            return;
        }

        println!("[mesh] render: {} primatives using vao {}", self.data.primatives.len(), self.data.vao);
        gl::BindVertexArray(self.data.vao);
        for p in self.data.primatives.iter() {
            p.render();
        }
        gl::BindVertexArray(0);
    }

//    pub fn render_name(&self, mesh_name: &String) {
//        for nv in self.data.named_vaos.iter() {
//            let (name, vao) = nv;
//            if name == mesh_name {
//                gl::BindVertexArray(*vao);
//                self.data.primatives.iter().map(|p| p.render()); // TODO: not sure if this works, removed it in the render fn above
//                break;
//            }
//        }
//    }

    pub fn delete_objects(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.data.attrib_array_buff);
            self.data.attrib_array_buff = 0;
            gl::DeleteBuffers(1, &self.data.idx_buff);
            self.data.idx_buff = 0;
            gl::DeleteVertexArrays(1, &self.data.vao);
            self.data.vao = 0;
            self.data.named_vaos.iter().map(|(_,vao)| gl::DeleteVertexArrays(1, vao));
        }
    }
}

impl Show for Mesh {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        f.write(format!("data: {}", self.data).as_bytes())
    }
}

pub fn unit_plane() -> Mesh {
    println!("[unit_plane] generating the unit_plane mesh");

    let attribs: Vec<Attribute> = vec!(
        Attribute {
            idx: 0,
            ty: Some(AttribType::float()),
            size: 3,
            is_integral: false,
            data_array: AttribData::floatv(vec!(0.5, 0.0, -0.5,
                                                0.5, 0.0,  0.5,
                                                -0.5, 0.0,  0.5,
                                                -0.5, 0.0, -0.5)),
        });
    println!("[unit_plane] attribs list: {}", attribs);

    let idx_data: Vec<IndexData> = vec!(
        IndexData {
            ty: Some(AttribType::ushort()),
            data_array: AttribData::ushortv(vec!(0, 1, 2,
                                                 0, 2, 1,
                                                 2, 3, 0,
                                                 2, 0, 3)),
        });
    println!("[unit_plane] index data list: {}", idx_data);

    let prims: Vec<RenderCmd> = vec!(
        RenderCmd { //start, elem_count, idx_data_type will be overwritten later
            idx_cmd: true,
            prim_type: gl::TRIANGLES,
            start: 0,
            elem_count: 0,
            idx_data_type: gl::UNSIGNED_SHORT,
        });
    println!("[unit_plane] render commands: {}", prims);
    Mesh::new(attribs, idx_data, prims)
}

pub fn unit_cube_color() -> Mesh {
    println!("[unit_cube_color] generating the unit_plane mesh");

    let attribs: Vec<Attribute> = vec!(
        Attribute {
            idx: 0,
            ty: Some(AttribType::float()),
            size: 3,
            is_integral: false,
            data_array: AttribData::floatv(vec!(
                    0.5, 0.5, 0.5,
                    0.5,-0.5, 0.5,
                   -0.5,-0.5, 0.5,
                   -0.5, 0.5, 0.5,
                    0.5, 0.5, 0.5,
                   -0.5, 0.5, 0.5,
                   -0.5, 0.5,-0.5,
                    0.5, 0.5,-0.5,
                    0.5, 0.5, 0.5,
                    0.5, 0.5,-0.5,
                    0.5,-0.5,-0.5,
                    0.5,-0.5, 0.5,
                    0.5, 0.5,-0.5,
                   -0.5, 0.5,-0.5,
                   -0.5,-0.5,-0.5,
                    0.5,-0.5,-0.5,
                    0.5,-0.5, 0.5,
                    0.5,-0.5,-0.5,
                   -0.5,-0.5,-0.5,
                   -0.5,-0.5, 0.5,
                   -0.5, 0.5, 0.5,
                   -0.5,-0.5, 0.5,
                   -0.5,-0.5,-0.5,
                   -0.5, 0.5,-0.5)),
        },
        Attribute {
            idx: 1,
            ty: Some(AttribType::float()),
            size: 4,
            is_integral: false,
            data_array: AttribData::floatv(vec!(
                    0.0, 1.0, 0.0, 1.0,
                    0.0, 1.0, 0.0, 1.0,
                    0.0, 1.0, 0.0, 1.0,
                    0.0, 1.0, 0.0, 1.0,
                    0.0, 0.0, 1.0, 1.0,
                    0.0, 0.0, 1.0, 1.0,
                    0.0, 0.0, 1.0, 1.0,
                    0.0, 0.0, 1.0, 1.0,
                    1.0, 0.0, 0.0, 1.0,
                    1.0, 0.0, 0.0, 1.0,
                    1.0, 0.0, 0.0, 1.0,
                    1.0, 0.0, 0.0, 1.0,
                    1.0, 1.0, 0.0, 1.0,
                    1.0, 1.0, 0.0, 1.0,
                    1.0, 1.0, 0.0, 1.0,
                    1.0, 1.0, 0.0, 1.0,
                    0.0, 1.0, 1.0, 1.0,
                    0.0, 1.0, 1.0, 1.0,
                    0.0, 1.0, 1.0, 1.0,
                    0.0, 1.0, 1.0, 1.0,
                    1.0, 0.0, 1.0, 1.0,
                    1.0, 0.0, 1.0, 1.0,
                    1.0, 0.0, 1.0, 1.0,
                    1.0, 0.0, 1.0, 1.0)),
        });
    println!("[unit_cube_color] attribs list: {}", attribs);

    let idx_data: Vec<IndexData> = vec!(
        IndexData {
            ty: Some(AttribType::ushort()),
            data_array: AttribData::ushortv(vec!(
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
                    22, 23, 20)),
        });
    println!("[unit_cube_color] index data list: {}", idx_data);

    let prims: Vec<RenderCmd> = vec!(
        RenderCmd { //start, elem_count, idx_data_type will be overwritten later
            idx_cmd: true,
            prim_type: gl::TRIANGLES,
            start: 0,
            elem_count: 0,
            idx_data_type: gl::UNSIGNED_SHORT,
        });
    println!("[unit_cube_color] render commands: {}", prims);
    Mesh::new(attribs, idx_data, prims)
}

//Mesh parser logic
//impl Mesh {
//    pub fn new_from_file(file_src: &str) -> Mesh {
//        let mut attribs: Vec<Attribute> = Vec::with_capacity(16);
//        let mut attrib_idx_map: TreeMap<GLuint, uint> = TreeMap::new(); //TODO: hm, this isn't even used in the original code...
//        let mut idx_data: Vec<IndexData> = Vec::new();
//        let mut named_vao_list: Vec<(String, Vec<GLuint>)> = Vec::new();
//
//        {
//            use std::io::File;
//            let path = Path::new(file_src);
//            let parser = sax::parse_file(&path).ok().expect("issue loading mesh");;
//
//            for result in parser.iter() {
//                match result {
//                    Ok(sax::StartDocument) => ( println!("doc start") ),
//                    Ok(sax::EndDocument) => ( println!("doc end") ),
//                    Ok(event) => {
//                        match event {
//                            sax::StartElement(e, attr) => ( println!("start: {} {}", e, attr) ),
//                            sax::EndElement(e) => ( println!("end: {}", e) ),
//                            sax::Characters(c) => ( println!("char: {}", c) ),
//                            _ => (),
//                        }
//                    },
//                    Err(err) => println!("{}", err),
//                }
//            }
//        }
//
//        let m = MeshData {
//            attrib_array_buff: 0,
//            idx_buff: 0,
//            vao: 0,
//            named_vaos: TreeMap::new(),
//            primatives: Vec::new(),
//        };
//
//        Mesh {
//            data: box m,
//        }
//    }
//}

