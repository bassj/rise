use std::rc::Rc;
use cgmath::Matrix4;

#[derive(Copy, Clone, Debug)]
pub struct CameraUniform {
    view: Matrix4<f32>,
    proj: Matrix4<f32>
}

unsafe impl bytemuck::Zeroable for CameraUniform {}
unsafe impl bytemuck::Pod for CameraUniform {}

pub struct MaterialInstance {
    render_pipeline: Rc<wgpu::RenderPipeline>
}

impl MaterialInstance {
    pub fn get_render_pipeline(&self) -> &wgpu::RenderPipeline {
        return self.render_pipeline.as_ref();
    }
}

//The material class serves to be a layout for how data will be sent to the shader. 
pub struct Material {
    render_pipeline: Rc<wgpu::RenderPipeline>
}

impl Material {
    pub fn new(render_pipeline: wgpu::RenderPipeline) -> Material {
        Material {
            render_pipeline: Rc::new(render_pipeline)
        }
    }

    pub fn create_instance(&self) -> MaterialInstance {
        MaterialInstance {
            render_pipeline: Rc::clone(&self.render_pipeline)
        }
    }
}

use std::path::Path;

pub trait MaterialBuilder {
    fn create_material<P: AsRef<Path>>(&self, material: P) -> Material;
}