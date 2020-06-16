use std::mem;
use std::ptr;
use std::str;
use std::ffi::CString;

use gl::types::*;

pub trait Renderer {
    fn new(
        windowed_context: &glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
    ) -> Self;
    fn clear(&self);
    fn resize(&self, physical_size: glutin::dpi::PhysicalSize<u32>);
}

pub struct GLRenderer {}

impl Renderer for GLRenderer {
    fn new(
        windowed_context: &glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
    ) -> GLRenderer {
        gl::load_with(|symbol| windowed_context.get_proc_address(symbol) as *const _);

        //gl::ClearColor();

        return GLRenderer {};
    }

    fn clear(&self) {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    fn resize(&self, physical_size: glutin::dpi::PhysicalSize<u32>) {
        unsafe {
            gl::Viewport(
                0,
                0,
                physical_size.width as i32,
                physical_size.height as i32,
            );
        }
    }
}

pub struct Mesh {
    vertices: Vec<(f32, f32)>,
    indices: Vec<u32>,
}

impl Mesh {
    pub fn new() -> Mesh {
        return Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
        };
    }
    pub fn add_vert(&mut self, vert: (f32, f32)) {
        self.vertices.push(vert);
    }

    pub fn set_indices(&mut self, indices: Vec<u32>) {
        self.indices = indices;
    }
}

pub struct GLShader {
    program_handle: u32,
    shaders: Vec<u32>
}

pub trait Shader {
    fn create(&mut self, renderer: &GLRenderer);

    fn bind(&self, renderer: &GLRenderer);

    fn destroy(&self, renderer: &GLRenderer);
}

impl Shader for GLShader {
    fn create(&mut self, _renderer: &GLRenderer) {
        unsafe {
            self.program_handle = gl::CreateProgram();

            for shader in self.shaders.iter() {
                gl::AttachShader(self.program_handle, *shader);

            }

            gl::LinkProgram(self.program_handle);

            // Get the link status
        let mut status = gl::FALSE as GLint;
        gl::GetProgramiv(self.program_handle, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as GLint) {
            let mut len: GLint = 0;
            gl::GetProgramiv(self.program_handle, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(
                self.program_handle,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf)
                    .ok()
                    .expect("ProgramInfoLog not valid utf8")
            );
        }


        }
    }

    fn bind(&self, _renderer: &GLRenderer) {
        unsafe {
            gl::UseProgram(self.program_handle);
        }
    }

    fn destroy(&self, _renderer: &GLRenderer) {
        unsafe {
            gl::DeleteProgram(self.program_handle);

            for shader in self.shaders.iter() {
                gl::DeleteShader(*shader);
            }
        }
    }
}

impl GLShader {
    pub fn new() -> GLShader {
        return GLShader {
            program_handle: 0,
            shaders: Vec::new()
        };
    }

    pub fn attach_src(&mut self, path: &str, ty: GLenum, _renderer: &GLRenderer) {
        
        let src : String = std::fs::read_to_string(path).unwrap();
        
        let shader;
        unsafe {
            shader = gl::CreateShader(ty);

            let c_src = CString::new(src.as_bytes()).unwrap();

            gl::ShaderSource(shader, 1, &c_src.as_ptr(), ptr::null());
            gl::CompileShader(shader);

            let mut status = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

            // Fail on error
            if status != (gl::TRUE as GLint) {
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
                let mut buf = Vec::with_capacity(len as usize);
                buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
                gl::GetShaderInfoLog(
                    shader,
                    len,
                    ptr::null_mut(),
                    buf.as_mut_ptr() as *mut GLchar,
                );
                panic!(
                    "{}",
                    str::from_utf8(&buf)
                        .ok()
                        .expect("ShaderInfoLog not valid utf8")
                );
            }
        }

        self.shaders.push(shader);
    }
}

pub struct Drawable {
    vao: u32,
    vbo: u32,
    mesh: Mesh,
    shader: GLShader
}

impl Drawable {
    pub fn new() -> Drawable {
        return Drawable {
            vao: 0,
            vbo: 0,
            mesh: Mesh::new(),
            shader: GLShader::new()
        };
    }

    pub fn create(&mut self, _renderer: &GLRenderer) {
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);

            let verts : Vec<f32> = self.mesh.vertices.iter().fold(Vec::new(), |mut array, vert| {
                array.push(vert.0);
                array.push(vert.1);
                //array.push(vert.2);
                array
            });

            gl::GenBuffers(1, &mut self.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (verts.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                mem::transmute(&verts[0]),
                gl::STATIC_DRAW,
            );

            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null());
        }
    }

    pub fn set_mesh(&mut self, mesh: Mesh) {
        self.mesh = mesh;
    }

    pub fn set_shader(&mut self, shader: GLShader) {
        self.shader = shader;
    }

    pub fn render(&self, renderer: &GLRenderer) {
        self.shader.bind(renderer);
        
        unsafe {
            
            
            //gl::EnableVertexAttribArray(0);

            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            //gl::DisableVertexAttribArray(0);
        }
    }

    pub fn destroy(&self, renderer: &GLRenderer) {

        self.shader.destroy(renderer);

        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}