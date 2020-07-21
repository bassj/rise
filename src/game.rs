use crate::graphics::*;

pub struct Window {
    pub width: f32,
    pub height: f32,
    pub resizable: bool,
    pub title: &'static str,
}

pub struct GameEnvironment {
    pub window: Window,
}

pub struct Game {
    triangle: RenderObject
}

impl Game {
    pub fn new(env: GameEnvironment, renderer: &Renderer) -> Game {

        let mut triangle_mesh = Mesh::new();

        triangle_mesh.add_verts(&[
            Vertex {
                position: cgmath::Vector3::new(0.0, 0.5, 0.0),
                normal: cgmath::Vector3::new(0., 0., 0.),
                uv: cgmath::Vector2::new(0., 0.)
            },
            Vertex {
                position: cgmath::Vector3::new(-0.5, -0.5, 0.0),
                normal: cgmath::Vector3::new(0., 0., 0.),
                uv: cgmath::Vector2::new(0., 0.)
            },
            Vertex {
                position: cgmath::Vector3::new(0.5, -0.5, 0.0),
                normal: cgmath::Vector3::new(0., 0., 0.),
                uv: cgmath::Vector2::new(0., 0.)
            }
        ]);

        triangle_mesh.set_indices(&[0, 1, 2]);

        let standard_material = Material::new();

        let triangle = renderer.create_object(triangle_mesh, standard_material.create_instance());
        
        Game {
            triangle
        }
    }

    pub fn update(&mut self, delta: f32) {
        unimplemented!();
    }

    pub fn render(&mut self, renderer: &mut Renderer) {
        renderer.render(&self.triangle);
    }   
}