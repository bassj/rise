pub struct MaterialInstance<'a> {
    base_material: &'a Material,
}

impl<'a> MaterialInstance<'a> {}

//The material class serves to be a layout for how data will be sent to the shader. 
pub struct Material {
    pub render_pipeline: wgpu::RenderPipeline,
}

impl Material {
    pub fn new(render_pipeline: wgpu::RenderPipeline) -> Material {
        Material {
            render_pipeline
        }
    }

    pub fn create_instance(&self) -> MaterialInstance {
        MaterialInstance {
            base_material: self
        }
    }
}

use std::path::Path;
use super::renderer::Renderer;

pub trait MaterialBuilder {
    fn create_material<P: AsRef<Path>>(&self, material: P) -> Material;
}