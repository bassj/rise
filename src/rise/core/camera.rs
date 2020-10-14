use crate::graphics::{Uniform, UniformBinding, RenderContext, CameraUniform};

pub trait Camera {
    fn get_depth_texture(&self) -> &crate::graphics::Texture;
    fn view_matrix(&self) -> cgmath::Matrix4<f32>;
    fn proj_matrix(&self) -> cgmath::Matrix4<f32>;
    fn update(&self, render_context: &RenderContext);
    fn get_binding(&self) -> Option<&UniformBinding>;
}

pub struct PerspectiveCamera {
    pub transform: super::Transform,
    pub fov: f32,
    aspect_ratio: f32,
    near_plane: f32,
    far_plane: f32,
    depth_texture: crate::graphics::Texture,
    binding: Option<UniformBinding>
}

impl PerspectiveCamera {
    pub fn new(render_context: &RenderContext, fov: f32, far_plane: f32) -> PerspectiveCamera {
        
        let aspect_ratio = (render_context.sc_desc.width as f32) / (render_context.sc_desc.height as f32);

        let depth_texture = render_context
            .create_texture()
            .make_depth_texture()
            .build();


        

        PerspectiveCamera {
            transform: crate::core::Transform::new(),
            fov,
            aspect_ratio,
            near_plane: 0.1,
            far_plane,
            depth_texture,
            binding: None
        }
    }
}

impl PerspectiveCamera {
    pub fn bind(&mut self, render_context: &RenderContext, camera_uniform: &Uniform) {
        /*let uniform_buffer = render_context.device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera Uniform Buffer"),
                contents: bytemuck::cast_slice(&[camera]),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            }
        );*/

        let uniform_buffer = render_context.device.create_buffer(
            &wgpu::BufferDescriptor {
                label: Some("Camera Uniform Buffer"),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                size: std::mem::size_of::<CameraUniform>() as wgpu::BufferAddress,
                mapped_at_creation: false,
            }
        );
        
        let camera_bind_group = render_context.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: camera_uniform.get_bind_group_layout(),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(uniform_buffer.slice(..)),
            }],
            label: Some("camera_bind_group"),
        });

        let binding = UniformBinding::new(camera_bind_group, Some(uniform_buffer));

        self.binding = Some(binding);
    }
}

impl Camera for PerspectiveCamera {
    fn get_depth_texture(&self) -> &crate::graphics::Texture {
        &self.depth_texture
    }

    fn view_matrix(&self) -> cgmath::Matrix4<f32> {

        use cgmath::Rotation;

        let forward = self.transform.rotation.rotate_vector(cgmath::Vector3::new(0., 0., -1.));
        let up = self.transform.rotation.rotate_vector(cgmath::Vector3::new(0., 1., 0.));

        let view_mat = cgmath::Matrix4::look_at_dir(
            cgmath::Point3::from((self.transform.position.x, self.transform.position.y, self.transform.position.z)),
            forward,
            up
        );

        view_mat
    }

    fn proj_matrix(&self) -> cgmath::Matrix4<f32> {
        let proj_mat = cgmath::perspective(cgmath::Deg(self.fov), self.aspect_ratio, self.near_plane, self.far_plane);

        proj_mat
    }

    fn update(&self, render_context: &RenderContext) {
        if self.binding.is_none() {
            panic!("Attempted to update an unbound camera");
        }

        let view_mat : cgmath::Matrix4<f32> = self.view_matrix();
        let proj_mat : cgmath::Matrix4<f32> = self.proj_matrix();

        let camera = crate::graphics::CameraUniform::new(view_mat, proj_mat);
    
        if let Some(binding) = &self.binding {
           if let Some(buffer) = binding.get_buffer() {
            render_context.queue.write_buffer(buffer, 0, bytemuck::cast_slice(&[camera]));
           }
        }

    }

    fn get_binding(&self) -> Option<&UniformBinding> {
        self.binding.as_ref()
    }
}

impl Into<crate::graphics::CameraUniform> for PerspectiveCamera {
    fn into(self) -> crate::graphics::CameraUniform {
        let view_mat : cgmath::Matrix4<f32> = self.view_matrix();
        let proj_mat : cgmath::Matrix4<f32> = self.proj_matrix();

        crate::graphics::CameraUniform::new(view_mat, proj_mat)
    }
}
