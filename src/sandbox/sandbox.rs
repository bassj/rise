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

struct FPSCamera {
    position: cgmath::Vector3<f32>,
    rotation: cgmath::Quaternion<f32>,
    fov: f32,
    aspect_ratio: f32,
    near_plane: f32,
    far_plane: f32,
}

impl FPSCamera {
    pub fn new(render_context: &rise::graphics::RenderContext, fov: f32, far_plane: f32) -> FPSCamera {
        
        let aspect_ratio = (render_context.sc_desc.width as f32) / (render_context.sc_desc.height as f32);

        FPSCamera {
            position: cgmath::Vector3::new(0., 0., 0.),
            rotation: cgmath::Quaternion::new(0., 0., 0., 0.),
            fov,
            aspect_ratio,
            near_plane: 0.1,
            far_plane
        }
    }

    pub fn view_matrix(&self) -> cgmath::Matrix4<f32> {
        let view_mat = cgmath::Matrix4::from_translation(self.position);

        view_mat
    }

    pub fn proj_matrix(&self) -> cgmath::Matrix4<f32> {
        let proj_mat = cgmath::perspective(cgmath::Deg(self.fov), self.aspect_ratio, self.near_plane, self.far_plane);

        proj_mat
    }
}

impl Into<rise::graphics::CameraUniform> for &FPSCamera {
    fn into(self) -> rise::graphics::CameraUniform {
        let view_mat : cgmath::Matrix4<f32> = self.view_matrix();
        let proj_mat : cgmath::Matrix4<f32> = self.proj_matrix();

        rise::graphics::CameraUniform::new(view_mat, proj_mat)
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
    camera: FPSCamera
}

impl rise::core::Application for Game {
    fn new(render_context: &mut rise::graphics::RenderContext) -> Game {
        
        let camera = FPSCamera::new(render_context, 60.0, 100.0);
        
        let standard_material = rise::graphics::Material::load(render_context, "res/mat/standard.mat");
       
        use rise::graphics::MaterialInstanceBuilder;

        let triangle = Triangle::new(render_context, standard_material.create_instance());

        Game {
            triangle,
            standard_material,
            camera
        }
    }
    
    fn update(&mut self, delta: f32) {
        self.camera.position.x += delta;
    }
    
    fn render(&self, render_context: &mut rise::graphics::RenderContext) {

        //Draw the frame through our camera.
        self.standard_material.as_ref().set_camera(render_context, &self.camera);
        
        let mut frame = rise::graphics::begin_frame(render_context);

        frame.render(&[&self.triangle]);
        
        rise::graphics::end_frame(frame);
    }

    fn process_input(&self, _event: &winit::event::Event<()>) {}
}

fn main() {
    rise::core::run_application::<Game>();
}
