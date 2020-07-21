use cgmath::Vector3;
use cgmath::Vector2;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: Vector3<f32>, 
    pub normal: Vector3<f32>,
    pub uv: Vector2<f32>
}


unsafe impl bytemuck::Zeroable for Vertex {}
unsafe impl bytemuck::Pod for Vertex {}

pub struct Mesh {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            vertices: Vec::new(),
            indices: Vec::new()
        }
    }

    pub fn add_verts<'a, I>(&mut self, verts: I) 
    where I: IntoIterator<Item=&'a Vertex> {
        let mut verts : Vec<Vertex> = verts.into_iter().map(|v| {*v}).collect();
        
        self.vertices.append(&mut verts);
    }

    pub fn set_indices<'a, I>(&mut self, indices: I)
    where I: IntoIterator<Item=&'a u16> {
        self.indices = indices.into_iter().map(|x| {*x}).collect();
    }
}