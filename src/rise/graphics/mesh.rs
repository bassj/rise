use log::{error};

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: cgmath::Vector3<f32>,
    normal: cgmath::Vector3<f32>,
    uv: cgmath::Vector2<f32>,
}

impl Vertex {
    pub fn new(position: cgmath::Vector3<f32>, normal: cgmath::Vector3<f32>, uv: cgmath::Vector2<f32>) -> Vertex {
        Vertex {
            position,
            normal,
            uv
        }
    }
    
    pub fn desc<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float2,
                },
                
            ]
        }
    }
}

unsafe impl bytemuck::Zeroable for Vertex {}
unsafe impl bytemuck::Pod for Vertex {}

#[macro_export]
macro_rules! point {
    ($x:expr, $y:expr, $z:expr) => {
        $crate::graphics::Vertex::new(
            cgmath::Vector3::new($x, $y, $z),
            cgmath::Vector3::new(0., 0., 0.),
            cgmath::Vector2::new(0., 0.)
        );
    }
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>
}

impl Mesh {
    pub fn load_from_file<P: AsRef<std::path::Path> + core::fmt::Debug>(path: P) -> Mesh {
        let(models, _materials) = tobj::load_obj(path, true).unwrap();

        //For the time being we're only going to load the first mesh in the model.
        
        let model = &models[0];

        println!("Loading model {} into mesh", model.name);

        let o_mesh = &model.mesh;

        let indices : Vec<u16> = o_mesh.indices.iter().map(|i| *i as u16).collect();


        let mut vertices : Vec<Vertex> = Vec::new();
        
        let num_vertices = (o_mesh.positions.len() / 3) as usize;


        for ind in 0..num_vertices {

            let ind_x = ind * 3 + 0;
            let ind_y = ind * 3 + 1;
            let ind_z = ind * 3 + 2;

            let mut pos = cgmath::Vector3::new(0., 0., 0.);

            pos.x = o_mesh.positions[ind_x];
            pos.y = o_mesh.positions[ind_y];
            pos.z = o_mesh.positions[ind_z];

            let mut norm = cgmath::Vector3::new(0., 0., 0.);

            if ind_x < o_mesh.normals.len() {
                norm.x = o_mesh.normals[ind_x];
                norm.y = o_mesh.normals[ind_y];
                norm.z = o_mesh.normals[ind_z];


                //println!("Normal: {} {} {}", norm.x, norm.y, norm.z);
            }

            let mut uv = cgmath::Vector2::new(0., 0.);

            uv.x = o_mesh.texcoords[ind * 2 + 0];
            uv.y = o_mesh.texcoords[ind * 2 + 1];

            vertices.push(Vertex::new(
                pos,
                norm,
                uv,
            ));
        }




        Mesh {
            vertices,
            indices,
            vertex_buffer: None,
            index_buffer: None,
        }
    }

    pub fn new() -> Mesh {
        Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
            vertex_buffer: None,
            index_buffer: None
        }
    }

    pub fn set_vertices<V: IntoIterator<Item=Vertex>>(&mut self, vertices: V) {
        self.vertices = vertices.into_iter().collect()
    }

    pub fn get_vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn set_indices<I: IntoIterator<Item=u16>>(&mut self, indices: I) {
        self.indices = indices.into_iter().collect();
    }

    pub fn get_indices(&self) -> &Vec<u16> {
        &self.indices
    }

    pub fn add_vertex(&mut self, vertex: Vertex) {
        self.vertices.push(vertex);
    }

    pub fn add_index(&mut self, index: u16) {
        self.indices.push(index);
    }

    pub fn create(&mut self, render_context: &crate::graphics::RenderContext) {
        if self.index_buffer.is_some() || self.vertex_buffer.is_some() {
            panic!("Attempted to create mesh twice");
        }

        use wgpu::util::DeviceExt;

        let vertex_buffer = render_context.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&self.vertices[..]),
                usage: wgpu::BufferUsage::VERTEX
            }
        );

        self.vertex_buffer = Some(vertex_buffer);

        let index_buffer = render_context.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&self.indices[..]),
                usage: wgpu::BufferUsage::INDEX  
            }
        );

        self.index_buffer = Some(index_buffer);
    }

    pub fn update(&mut self, render_context: &crate::graphics::RenderContext) { 
        
        if self.index_buffer.is_none() || self.vertex_buffer.is_none() {
            error!("Attempted to update mesh before creation.");
            return;
        }

        //TODO: Try to see if we can add data without creating new buffer.

        use wgpu::util::DeviceExt;
        
        let vertex_buffer = render_context.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&self.vertices[..]),
                usage: wgpu::BufferUsage::VERTEX
            }
        );

        self.vertex_buffer = Some(vertex_buffer);

        let index_buffer = render_context.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&self.indices[..]),
                usage: wgpu::BufferUsage::INDEX 
            }
               
        );

        self.index_buffer = Some(index_buffer);
    }

    pub fn get_vertex_buffer(&self) -> Option<&wgpu::Buffer> {
        self.vertex_buffer.as_ref()
    }

    pub fn get_index_buffer(&self) -> Option<&wgpu::Buffer> {
        self.index_buffer.as_ref()
    }
}