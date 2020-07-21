use cgmath::Vector3;
use cgmath::Vector2;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: Vector3<f32>, 
    normal: Vector3<f32>,
    uv: Vector2<f32>
}


unsafe impl bytemuck::Zeroable for Vertex {}
unsafe impl bytemuck::Pod for Vertex {}


mod material;
pub use self::material::Material;

mod mesh;
pub use self::mesh::Mesh;

mod renderer;
pub use self::renderer::Renderer;
pub use self::renderer::RenderObject;