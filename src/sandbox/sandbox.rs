struct Triangle {
    mesh: rise::graphics::Mesh,
    material: rise::graphics::MaterialInstance
}

impl Triangle {
    fn new(render_context: &rise::graphics::RenderContext, material: rise::graphics::MaterialInstance) -> Triangle {
        
        use rise::point;

        let vertices = vec!(
            point!(-1., -1., 1.),
            point!(1., -1., 1.),
            point!(1., 1., 1.),
            point!(-1., 1., 1.),

            point!(-1., 1., -1.),
            point!(1., 1., -1.),
            point!(1., -1., -1.),
            point!(-1., -1., -1.),

            point!(1., -1., -1.),
            point!(1., 1., -1.),
            point!(1., 1., 1.),
            point!(1., -1., 1.),

            point!(-1., -1., 1.),
            point!(-1., 1., 1.),
            point!(-1., 1., -1.),
            point!(-1., -1., -1.),

            point!(1., 1., -1.),
            point!(-1., 1., -1.),
            point!(-1., 1., 1.),
            point!(1., 1., 1.),

            point!(1., -1., 1.),
            point!(-1., -1., 1.),
            point!(-1., -1., -1.),
            point!(1., -1., -1.),
        );

        let indices : Vec<u16> = vec!(0, 1, 2, 2, 3, 0, // top
            4, 5, 6, 6, 7, 4, // bottom
            8, 9, 10, 10, 11, 8, // right
            12, 13, 14, 14, 15, 12, // left
            16, 17, 18, 18, 19, 16, // front
            20, 21, 22, 22, 23, 20, // back
        );
        
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
    env: rise::core::ApplicationEnvironment,
    triangle: Triangle,
    standard_material: std::rc::Rc<rise::graphics::Material>,
    camera: rise::core::PerspectiveCamera,
    input: rise::core::InputManager,
    cam_yaw: f32,
    cam_pitch: f32,
    mouse_locked: bool,
    mouse_sensitivity: f32
}

impl rise::core::Application for Game {
    fn new(env: rise::core::ApplicationEnvironment, render_context: &mut rise::graphics::RenderContext) -> Game {
        
        let camera = rise::core::PerspectiveCamera::new(render_context, 60.0, 100.0);
        
        let standard_material = rise::graphics::Material::load(render_context, "res/mat/standard.mat");
       
        use rise::graphics::MaterialInstanceBuilder;

        let triangle = Triangle::new(render_context, standard_material.create_instance());

        let input = rise::core::InputManager::new();

        Game {
            triangle,
            standard_material,
            camera,
            input,
            env,
            cam_yaw: 0.,
            cam_pitch: 0.,
            mouse_locked: false,
            mouse_sensitivity: 0.2
        }
    }
    
    fn update(&mut self, delta: f32) {
        use rise::core::Key;

        use cgmath::Rotation;

        let forward = self.camera.rotation.rotate_vector(cgmath::Vector3::new(0., 0., -delta));
        let right = self.camera.rotation.rotate_vector(cgmath::Vector3::new(delta, 0., 0.));

        if self.input.is_pressed(Key::S) {
            self.camera.position -= forward;
        } else if self.input.is_pressed(Key::W) {
            self.camera.position += forward;
        }

        if self.input.is_pressed(Key::D) {
            self.camera.position += right;
        } else if self.input.is_pressed(Key::A) {
            self.camera.position -= right;
        }

        let (delta_x, delta_y) = self.input.mouse_motion();

        if self.input.is_mouse_just_pressed(rise::core::MouseButton::Left) {
            self.mouse_locked = !self.mouse_locked;

            self.env.get_window().set_cursor_grab(self.mouse_locked).unwrap();
            self.env.get_window().set_cursor_visible(!self.mouse_locked);
        }
        
        if self.mouse_locked {
            println!("DX: {}, DY: {}", delta_x, delta_y);

            self.cam_yaw -= (delta_x as f32) * delta;
            self.cam_pitch -= (delta_y as f32) * delta;

            let size = self.env.get_window().inner_size();
            
            self.env.get_window().set_cursor_position(winit::dpi::PhysicalPosition::new(size.width / 2, size.height / 2)).unwrap();
        }

        use cgmath::Rotation3;

        self.camera.rotation = cgmath::Quaternion::from(cgmath::Euler {
            x: cgmath::Rad(0.0),
            y: cgmath::Rad(self.cam_yaw),
            z: cgmath::Rad(0.0)
        });

        let right = cgmath::Vector3::new(1.0, 0.0, 0.0);

        self.camera.rotation = self.camera.rotation * cgmath::Quaternion::from_axis_angle(right, cgmath::Rad(self.cam_pitch));

        self.input.update();
    }
    
    fn render(&self, render_context: &mut rise::graphics::RenderContext) {
        //Draw the frame through our camera.
        self.standard_material.as_ref().set_camera(render_context, &self.camera);
        
        let mut frame = rise::graphics::begin_frame(render_context);

        frame.render(&[&self.triangle]);
        
        rise::graphics::end_frame(frame);
    }

    fn process_event(&mut self, event: &winit::event::Event<()>) {
        self.input.process_event(event);
    }
}

fn main() {
    rise::core::run_application::<Game>();
}
