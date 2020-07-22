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
    standard_material: Material,
    triangle: RenderObject
}

impl Game {
    pub fn new(_env: GameEnvironment, renderer: &mut Renderer) -> Game {

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

        let standard_material = renderer.create_material("./res/shader/standard.mat");

        let material_instance = standard_material.create_instance();

        let triangle = renderer.create_object(&triangle_mesh, material_instance);

        Game {  
            standard_material,
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