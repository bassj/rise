use rise::graphics::{Mesh, Material, MaterialInstance, MaterialInstanceBuilder};

struct Game {
    env: rise::core::ApplicationEnvironment,
    monkey: rise::graphics::Drawable,
    standard_material: std::rc::Rc<rise::graphics::Material>,
    camera: rise::core::PerspectiveCamera,
    input: rise::core::InputManager,
    cam_yaw: f32,
    cam_pitch: f32,
    mouse_locked: bool,
    mouse_sensitivity: f32,
}

impl rise::core::Application for Game {
    fn new(
        env: rise::core::ApplicationEnvironment,
        render_context: &mut rise::graphics::RenderContext,
    ) -> Game {
        let standard_material = render_context
            .create_material()
            .from_file("res/mat/standard.mat")
            .build();
        let standard_material = std::rc::Rc::new(standard_material);

        let camera_uniform = &standard_material.get_camera_uniform();

        let mut camera = rise::core::PerspectiveCamera::new(render_context, 60.0, 100.0);
        camera.transform.position.z = 5.;

        camera.bind(render_context, camera_uniform);

        let plane_mesh = Mesh::load_from_file("res/model/sphere.obj");

        /*let mut plane_mesh = Mesh::new();

        use rise::graphics::Vertex;
        use cgmath::{Vector3, Vector2};

        let vertices = vec!(
            Vertex::new(
                Vector3::new(-1., 1., 0.0),
                Vector3::new(0., 0., -1.),
                Vector2::new(0., 0.)
            ),
            Vertex::new(
                Vector3::new(1., 1., 0.0),
                Vector3::new(0., 0., -1.),
                Vector2::new(1., 0.)
            ),
            Vertex::new(
                Vector3::new(1., -1., 0.0),
                Vector3::new(0., 0., -1.),
                Vector2::new(1., 1.)
            ),
            Vertex::new(
                Vector3::new(-1., -1., 0.0),
                Vector3::new(0., 0., -1.),
                Vector2::new(0., 1.)
            )
        );

        
        plane_mesh.set_indices(vec![2 as u16, 1 as u16, 0 as u16, 0 as u16, 3 as u16, 2 as u16]);
        plane_mesh.set_vertices(vertices);*/

        // plane_mesh.create(render_context);



        let monkey_uv = render_context
            .create_texture()
            .load_file("res/tex/testimage.jpg")
            .unwrap();

        let mut material = standard_material.create_instance();
            material.use_texture("diffuse", &monkey_uv, render_context).expect("Error binding texture.");

        let monkey = render_context
            .create_drawable()
            .with_mesh(plane_mesh)
            .with_material(material)
            .build();

        
        let input = rise::core::InputManager::new();

        Game {
            monkey,
            standard_material,
            camera,
            input,
            env,
            cam_yaw: 0.,
            cam_pitch: 0.,
            mouse_locked: false,
            mouse_sensitivity: 0.2,
        }
    }
    fn update(&mut self, delta: f32) {
        use rise::core::Key;

        use cgmath::Rotation;

        let forward = self
            .camera
            .transform
            .rotation
            .rotate_vector(cgmath::Vector3::new(0., 0., -delta));
        let right = self
            .camera
            .transform
            .rotation
            .rotate_vector(cgmath::Vector3::new(delta, 0., 0.));
        let up = self
            .camera
            .transform
            .rotation
            .rotate_vector(cgmath::Vector3::new(0., delta, 0.));

        if self.input.is_pressed(Key::S) {
            self.camera.transform.position -= forward;
        } else if self.input.is_pressed(Key::W) {
            self.camera.transform.position += forward;
        }

        if self.input.is_pressed(Key::D) {
            self.camera.transform.position += right;
        } else if self.input.is_pressed(Key::A) {
            self.camera.transform.position -= right;
        }

        if self.input.is_pressed(Key::Space) {
            self.camera.transform.position += up;
        } else if self.input.is_pressed(Key::LShift) {
            self.camera.transform.position -= up;
        }

        let (delta_x, delta_y) = self.input.mouse_motion();

        if self
            .input
            .is_mouse_just_pressed(rise::core::MouseButton::Left)
        {
            self.mouse_locked = !self.mouse_locked;

            self.env
                .get_window()
                .set_cursor_grab(self.mouse_locked)
                .unwrap();
            self.env.get_window().set_cursor_visible(!self.mouse_locked);
        }
        if self.mouse_locked {
            // println!("DX: {}, DY: {}", delta_x, delta_y);

            self.cam_yaw -= (delta_x as f32) * self.mouse_sensitivity * delta;
            self.cam_pitch -= (delta_y as f32) * self.mouse_sensitivity * delta;

            let size = self.env.get_window().inner_size();

            self.env
                .get_window()
                .set_cursor_position(winit::dpi::PhysicalPosition::new(
                    size.width / 2,
                    size.height / 2,
                ))
                .unwrap();
        }

        use cgmath::Rotation3;

        self.camera.transform.rotation = cgmath::Quaternion::from(cgmath::Euler {
            x: cgmath::Rad(0.0),
            y: cgmath::Rad(self.cam_yaw),
            z: cgmath::Rad(0.0),
        });

        let right = cgmath::Vector3::new(1.0, 0.0, 0.0);

        self.camera.transform.rotation = self.camera.transform.rotation
            * cgmath::Quaternion::from_axis_angle(right, cgmath::Rad(self.cam_pitch));

        self.input.update();
    }
    fn render(&self, render_context: &mut rise::graphics::RenderContext) {
        let mut frame = rise::graphics::begin_frame(render_context);

        frame.render(&[&self.monkey], &self.camera);
        rise::graphics::end_frame(frame);
    }

    fn process_event(&mut self, event: &winit::event::Event<()>) {
        self.input.process_event(event);
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    rise::core::run_application::<Game>()
}
