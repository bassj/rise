pub struct Transform {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
}

impl Transform {
    pub fn new() -> Transform {
        Transform { 
            position: cgmath::Vector3::new(0., 0., 0.),
            rotation: cgmath::Quaternion::new(1.0, 0.0, 0.0, -1.0),
        }
    }

    pub fn build_transform_matrix() -> cgmath::Matrix4<f32> {
        unimplemented!();
    }

    pub fn build_inverse_transform_matrix() -> cgmath::Matrix4<f32> {
        unimplemented!();
    }
}