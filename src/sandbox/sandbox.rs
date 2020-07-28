struct Triangle {
    mesh: rise::graphics::Mesh,
    material: rise::graphics::MaterialInstance
}

impl Triangle {
    fn new(render_context: &rise::graphics::RenderContext, material: rise::graphics::MaterialInstance) -> Triangle {
        
        use rise::point;

        let vertices = vec!(
            point!(0.0, 0.5, -1.0),
            point!(-0.5, -0.5, -1.0),
            point!(0.5, -0.5, -1.0)
        );

        let indices : Vec<u16> = vec!(0, 1, 2);
        
        let mut mesh = rise::graphics::Mesh::new();

        mesh.set_vertices(vertices);
        mesh.set_indices(indices);
        
        mesh.create(&render_context);

        Triangle {
            mesh,
            material
        }
    }
}

impl rise::graphics::Drawable for Triangle {
    fn get_material(&self) -> &rise::graphics::MaterialInstance {
        &self.material
    }

    fn get_vertex_buffer(&self) -> &wgpu::Buffer {
        self.mesh.get_vertex_buffer().unwrap()
    }
    
    fn get_index_buffer(&self) -> &wgpu::Buffer {
        self.mesh.get_index_buffer().unwrap()
    }

    fn num_indices(&self) -> u32 {
        self.mesh.indices.len() as u32
    }
}

struct Game {
    triangle: Triangle,
    standard_material: std::rc::Rc<rise::graphics::Material>,
}

impl rise::core::Application for Game {
    fn new(render_context: &mut rise::graphics::RenderContext) -> Game {
        let standard_material = rise::graphics::Material::load(render_context, "res/mat/standard.mat");
       
        use rise::graphics::MaterialInstanceBuilder;

        let triangle = Triangle::new(render_context, standard_material.create_instance());

        Game {
            triangle,
            standard_material,
        }
    }
    
    fn update(&self, _delta: f32) {}
    
    fn render(&self, render_context: &mut rise::graphics::RenderContext) {

        let camera_uniform = {
            use cgmath::SquareMatrix;
            let proj_mat = cgmath::perspective(cgmath::Deg(60.0), 800.0 / 600.0, 0.1, 100.);
            let view_mat = cgmath::Matrix4::identity();
        
            rise::graphics::CameraUniform::new(view_mat, proj_mat)
        };

        self.standard_material.as_ref().set_camera(render_context, camera_uniform);    
        
        let mut frame = rise::graphics::begin_frame(render_context);

        frame.render(&[&self.triangle]);
        
        rise::graphics::end_frame(frame);
    }

    fn process_input(&self, _event: &winit::event::Event<()>) {}
}

fn main() {
    rise::core::run_application::<Game>();
}
