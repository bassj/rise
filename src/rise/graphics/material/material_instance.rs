use crate::{RISEError, Result};

use std::collections::HashMap;

pub struct MaterialInstance {
  base_material: std::rc::Rc<super::Material>,
  uniform_values: HashMap<usize, super::UniformBinding>,
}

impl MaterialInstance {
  pub fn use_texture(
    &mut self,
    name: &str,
    texture: &crate::graphics::Texture,
    render_context: &crate::graphics::RenderContext,
  ) -> Result<()> {
    let base = self.base_material.as_ref();

    

    match base.get_binding_by_name(name) {
      Some(binding_index) => {
        println!("Putting {} texture in bind_group {}", name, binding_index);
        let uniform = &base.get_uniforms()[*binding_index];
        let bind_group_layout = uniform.get_bind_group_layout();

        let bind_group = render_context
          .device
          .create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
              wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(texture.get_view()),
              },
              wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(texture.get_sampler()),
              },
            ],
            label: Some("texture_bind_group"),
          });

        self
          .uniform_values
          .insert(*binding_index, super::UniformBinding::new(bind_group, None));

        Ok(())
      }
      None => Err(RISEError {}),
    }
  }

  pub fn get_base_material(&self) -> &super::Material {
    self.base_material.as_ref()
  }

  pub fn get_uniforms(&self) -> &HashMap<usize, super::UniformBinding> {
    &self.uniform_values
  }
}

pub trait MaterialInstanceBuilder {
  fn create_instance(&self) -> MaterialInstance;
}

impl MaterialInstanceBuilder for std::rc::Rc<super::Material> {
  fn create_instance(&self) -> MaterialInstance {
    MaterialInstance {
      base_material: self.clone(),
      uniform_values: HashMap::new(),
    }
  }
}
