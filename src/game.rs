use crate::graphics::Renderer;
use crate::graphics::RenderObject;
use crate::graphics::Mesh;
use crate::graphics::Material;

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

        let triangle_mesh = Mesh::new();

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