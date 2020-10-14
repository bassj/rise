const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
  1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 1.0,
);

#[derive(Copy, Clone, Debug)]
pub struct CameraUniform {
    view_mat: cgmath::Matrix4<f32>,
    proj_mat: cgmath::Matrix4<f32>,
}

impl CameraUniform {
    pub fn new(view_mat: cgmath::Matrix4<f32>, proj_mat: cgmath::Matrix4<f32>) -> CameraUniform {
        CameraUniform {
            view_mat,
            proj_mat: OPENGL_TO_WGPU_MATRIX * proj_mat,
        }
    }
}

impl<C: crate::core::Camera + Into<CameraUniform>> std::convert::From<&C> for CameraUniform {
  fn from(c: &C) -> CameraUniform {
    c.into()
  }
}

unsafe impl bytemuck::Zeroable for CameraUniform {}
unsafe impl bytemuck::Pod for CameraUniform {}

pub struct Uniform {
    bind_group_layout: wgpu::BindGroupLayout,
    name: String
}

impl Uniform {
  pub fn new(name: &str, bind_group_layout: wgpu::BindGroupLayout) -> Self {
    Uniform {
      name: String::from(name),
      bind_group_layout
    }
  }

  pub fn get_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
    &self.bind_group_layout
  }

  pub fn get_name(&self) -> &str {
    &self.name
  }
}

pub struct UniformBinding {
  bind_group: wgpu::BindGroup,
  buffer: Option<wgpu::Buffer>
}

impl UniformBinding {
  pub fn new(bind_group: wgpu::BindGroup, buffer: Option<wgpu::Buffer>) -> Self {
    UniformBinding {
      bind_group,
      buffer
    }
  }

  pub fn get_bind_group(&self) -> &wgpu::BindGroup {
    &self.bind_group
  }

  pub fn get_buffer(&self) -> Option<&wgpu::Buffer> {
    self.buffer.as_ref()
  }
}