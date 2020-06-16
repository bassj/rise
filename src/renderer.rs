use std::mem;
use std::ptr;

use gl::types::*;

pub trait Renderer {
    fn new(windowed_context: &glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>) -> Self;
    fn clear(&self);
    fn resize(&self, physical_size: glutin::dpi::PhysicalSize<u32>);
}

pub struct GLRenderer {

}

impl Renderer for GLRenderer {
    fn new(windowed_context: &glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>) -> GLRenderer {

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
            gl::Viewport(0, 0, physical_size.width as i32, physical_size.height as i32);
        }
    }
}

pub struct Mesh {
    vertices: Vec<(f32, f32)>,
    indices: Vec<u32>
}

impl Mesh {
    
    pub fn new() -> Mesh {
        return Mesh {
            vertices: Vec::new(),
            indices: Vec::new()
        }
    }
    
    pub fn add_vert(&mut self, vert: (f32, f32)) {
        self.vertices.push(vert);
    }

    pub fn set_indices(&mut self, indices: Vec<u32>) {
        self.indices = indices;
    }
}

pub struct Drawable {
    vao: u32,
    vbo: u32,
    mesh: Mesh
}

impl Drawable {

    pub fn new() -> Drawable {
        return Drawable {
            vao: 0,
            vbo: 0,
            mesh: Mesh::new()
        };
    }

    pub fn create(&mut self, _renderer: &GLRenderer) {
        unsafe {
            gl::GenVertexArrays(1, &mut self.vao);
            gl::BindVertexArray(self.vao);

            gl::GenBuffers(1, &mut self.vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.mesh.vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                mem::transmute(&self.mesh.vertices[0]),
                gl::STATIC_DRAW
            );

            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE as GLboolean, 0, ptr::null());
        }
    }

    pub fn set_mesh(&mut self, mesh: Mesh) {
        self.mesh = mesh;
    }

    pub fn render(&self, _renderer: &GLRenderer) {
        unsafe {
            //gl::EnableVertexAttribArray(0);

            gl::DrawArrays(gl::TRIANGLES, 0, 3);

            //gl::DisableVertexAttribArray(0);
        }
    }

    pub fn destroy(&self, _renderer: &GLRenderer) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}
