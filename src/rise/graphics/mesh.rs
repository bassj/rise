#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: cgmath::Vector3<f32>
}

impl Vertex {
    pub fn new(position: cgmath::Vector3<f32>) -> Vertex {
        Vertex {
            position
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
                }
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
            cgmath::Vector3::<f32>::new($x, $y, $z)
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

    pub fn set_indices<I: IntoIterator<Item=u16>>(&mut self, indices: I) {
        self.indices = indices.into_iter().collect();
    }

    pub fn add_vertex(&mut self, vertex: Vertex) {
        self.vertices.push(vertex);
    }

    pub fn add_index(&mut self, index: u16) {
        self.indices.push(index);
    }

    pub fn create(&mut self, render_context: &crate::graphics::RenderContext) {
        if self.index_buffer != std::option::Option::None || self.vertex_buffer != std::option::Option::None {
            panic!("Attempted to create mesh twice");
        }

        let vertex_buffer = render_context.device.create_buffer_with_data(
            bytemuck::cast_slice(&self.vertices[..]),
            wgpu::BufferUsage::VERTEX
        );

        self.vertex_buffer = Some(vertex_buffer);

        let index_buffer = render_context.device.create_buffer_with_data(
            bytemuck::cast_slice(&self.indices[..]),
            wgpu::BufferUsage::INDEX    
        );

        self.index_buffer = Some(index_buffer);
    }

    pub fn update(&mut self, render_context: &crate::graphics::RenderContext) { 
        if self.index_buffer == std::option::Option::None || self.vertex_buffer == std::option::Option::None {
            panic!("Attempted to update mesh before creation.");
        }

        //TODO: Try to see if we can add data without creating new buffer.

        let vertex_buffer = render_context.device.create_buffer_with_data(
            bytemuck::cast_slice(&self.vertices[..]),
            wgpu::BufferUsage::VERTEX
        );

        self.vertex_buffer = Some(vertex_buffer);

        let index_buffer = render_context.device.create_buffer_with_data(
            bytemuck::cast_slice(&self.indices[..]),
            wgpu::BufferUsage::INDEX    
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

/*struct Triangle {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    vertices: Vec<rise::graphics::Vertex>,
    indices: Vec<u16>,
    material: rise::graphics::MaterialInstance
}

impl Triangle {
    fn new(render_context: &rise::graphics::RenderContext, material: rise::graphics::MaterialInstance) -> Triangle {
        
        use rise::point;

        let vertices = vec!(
            point!(0.0, 0.5, 0.0),
            point!(-0.5, -0.5, 0.0),
            point!(0.5, -0.5, 0.0)
        );

        let indices : Vec<u16> = vec!(0, 1, 2);
        
        let vertex_buffer = render_context.device.create_buffer_with_data(
            bytemuck::cast_slice(&vertices[..]),
            wgpu::BufferUsage::VERTEX,
        );
        
        let index_buffer = render_context.device.create_buffer_with_data(
            bytemuck::cast_slice(&indices[..]),
            wgpu::BufferUsage::INDEX,
        );
        
        Triangle {
            vertex_buffer,
            index_buffer,
            indices,
            vertices,
            material
        }
    }
}

impl rise::graphics::Drawable for Triangle {
    fn get_material(&self) -> &rise::graphics::MaterialInstance {
        &self.material
    }

    fn get_vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }
    
    fn get_index_buffer(&self) -> &wgpu::Buffer {
        &self.index_buffer
    }

    fn num_indices(&self) -> u32 {
        self.indices.len() as u32
    }
} */