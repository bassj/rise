pub struct PerspectiveCamera {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
    pub fov: f32,
    aspect_ratio: f32,
    near_plane: f32,
    far_plane: f32,
}

impl PerspectiveCamera {
    pub fn new(render_context: &crate::graphics::RenderContext, fov: f32, far_plane: f32) -> PerspectiveCamera {
        
        let aspect_ratio = (render_context.sc_desc.width as f32) / (render_context.sc_desc.height as f32);

        PerspectiveCamera {
            position: cgmath::Vector3::new(0., 0., 0.),
            rotation: cgmath::Quaternion::new(0., 0., 0., 0.),
            fov,
            aspect_ratio,
            near_plane: 0.1,
            far_plane
        }
    }

    pub fn view_matrix(&self) -> cgmath::Matrix4<f32> {

        use cgmath::Rotation;

        let forward = self.rotation.rotate_vector(cgmath::Vector3::new(0., 0., -1.));
        let up = self.rotation.rotate_vector(cgmath::Vector3::new(0., 1., 0.));

        let view_mat = cgmath::Matrix4::look_at_dir(
            cgmath::Point3::from((self.position.x, self.position.y, self.position.z)),
            forward,
            up
        );

        view_mat
    }

    pub fn proj_matrix(&self) -> cgmath::Matrix4<f32> {
        let proj_mat = cgmath::perspective(cgmath::Deg(self.fov), self.aspect_ratio, self.near_plane, self.far_plane);

        proj_mat
    }
}

impl Into<crate::graphics::CameraUniform> for &PerspectiveCamera {
    fn into(self) -> crate::graphics::CameraUniform {
        let view_mat : cgmath::Matrix4<f32> = self.view_matrix();
        let proj_mat : cgmath::Matrix4<f32> = self.proj_matrix();

        crate::graphics::CameraUniform::new(view_mat, proj_mat)
    }
}
