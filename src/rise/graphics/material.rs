const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);


#[derive(Copy, Clone, Debug)]
pub struct CameraUniform {
    view_mat: cgmath::Matrix4<f32>,
    proj_mat: cgmath::Matrix4<f32>
}

impl CameraUniform {
    pub fn new(view_mat: cgmath::Matrix4<f32>, proj_mat: cgmath::Matrix4<f32>) -> CameraUniform {
        CameraUniform {
            view_mat,
            proj_mat: OPENGL_TO_WGPU_MATRIX * proj_mat
        }
    }
}

unsafe impl bytemuck::Zeroable for CameraUniform {}
unsafe impl bytemuck::Pod for CameraUniform {}

pub struct UniformManager {
    camera_uniform_bind_group: wgpu::BindGroup,
    camera_uniform_buffer: wgpu::Buffer
}

impl UniformManager {
    pub fn set_camera(&self, render_context: &crate::graphics::RenderContext, cam : CameraUniform) {
        let mut encoder = render_context.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("uniform buffer encoder.")
        });

        let staging_buffer = render_context.device.create_buffer_with_data(
            bytemuck::cast_slice(&[cam]),
            wgpu::BufferUsage::COPY_SRC,
        );

        encoder.copy_buffer_to_buffer(&staging_buffer, 0, &self.camera_uniform_buffer, 0, std::mem::size_of::<CameraUniform>() as wgpu::BufferAddress);

        render_context.queue.submit(&[encoder.finish()]);
    }

    pub fn get_camera(&self) -> &wgpu::BindGroup {
        &self.camera_uniform_bind_group
    }
}

pub struct Material {
    render_pipeline: wgpu::RenderPipeline,
    depth_texture: crate::graphics::Texture,
    uniforms: UniformManager
}

impl Material {
    pub fn new(render_context: &crate::graphics::RenderContext, vertex_stage: Vec<u8>, fragment_stage: Vec<u8>) -> Material {
        let vs_spirv = vertex_stage;
        let fs_spirv = fragment_stage;

        let vs_data = wgpu::read_spirv(std::io::Cursor::new(vs_spirv)).unwrap();
        let fs_data = wgpu::read_spirv(std::io::Cursor::new(fs_spirv)).unwrap();

        let vs_module = render_context.device.create_shader_module(&vs_data);
        let fs_module = render_context.device.create_shader_module(&fs_data);

        let camera_uniform_buffer = render_context.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("camera_uniform_buffer"),
            size: std::mem::size_of::<CameraUniform>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let camera_uniform_bind_group_layout = render_context.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                    },
                }
            ],
            label: Some("camera_uniform_bind_group_layout"),
        });

        let camera_uniform_bind_group = render_context.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_uniform_bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &camera_uniform_buffer,
                        range: 0..(std::mem::size_of::<CameraUniform>() as u64)
                    }
                }
            ],
            label: Some("camera_uniform_bind_group"),
        });

        let depth_texture = crate::graphics::Texture::create_depth_texture(&render_context);

        let render_pipeline_layout =
            render_context.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    bind_group_layouts: &[&camera_uniform_bind_group_layout],
                });

                
        use crate::graphics::Vertex;

        let render_pipeline = render_context
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                layout: &render_pipeline_layout,
                vertex_stage: wgpu::ProgrammableStageDescriptor {
                    module: &vs_module,
                    entry_point: "main",
                },
                fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                    module: &fs_module,
                    entry_point: "main",
                }),
                rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: wgpu::CullMode::Back,
                    depth_bias: 0,
                    depth_bias_slope_scale: 0.0,
                    depth_bias_clamp: 0.0,
                }),
                color_states: &[wgpu::ColorStateDescriptor {
                    format: render_context.sc_desc.format,
                    color_blend: wgpu::BlendDescriptor::REPLACE,
                    alpha_blend: wgpu::BlendDescriptor::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                }],

                primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::LessEqual,
                    stencil_front: wgpu::StencilStateFaceDescriptor::IGNORE,
                    stencil_back: wgpu::StencilStateFaceDescriptor::IGNORE,
                    stencil_read_mask: 0,
                    stencil_write_mask: 0,
                }),
                vertex_state: wgpu::VertexStateDescriptor {
                    index_format: wgpu::IndexFormat::Uint16,
                    vertex_buffers: &[Vertex::desc()],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            });

        
        
        let uniforms = UniformManager {
            camera_uniform_bind_group,
            camera_uniform_buffer
        };

        use cgmath::SquareMatrix;
        uniforms.set_camera(render_context,
            CameraUniform {
                view_mat: cgmath::Matrix4::identity(),
                proj_mat: cgmath::Matrix4::identity(),
            }
        );

        Material {
            render_pipeline,
            depth_texture,
            uniforms
        }
    }

    pub fn load<P: AsRef<std::path::Path>>(render_context: &crate::graphics::RenderContext, path: P) -> std::rc::Rc<Material> {

        let material_src = std::fs::read_to_string(path).unwrap();

        let material_layout : serde_json::Value = serde_json::from_str(&material_src).unwrap();

        let vs_path = material_layout["vertex_stage"].as_str().unwrap();
        let fs_path = material_layout["fragment_stage"].as_str().unwrap();


        let fs_spirv = std::fs::read(fs_path).unwrap();
        let vs_spirv = std::fs::read(vs_path).unwrap();

        std::rc::Rc::new(Material::new(render_context, vs_spirv, fs_spirv))
    }

    pub fn set_camera<C: Into<CameraUniform>>(&self, render_context: &crate::graphics::RenderContext, camera: C) {
        self.uniforms.set_camera(render_context, camera.into());
    }

    pub fn get_depth_texture(&self) -> &crate::graphics::Texture {
        &self.depth_texture
    }

    pub fn get_render_pipeline(&self) -> &wgpu::RenderPipeline {
        &self.render_pipeline
    }

    pub fn get_uniforms(&self) -> &UniformManager {
        &self.uniforms
    }
}

pub struct MaterialInstance {
    base_material: std::rc::Rc<Material>,
}

impl MaterialInstance {
    pub fn get_base_material(&self) -> &Material {
        self.base_material.as_ref()
    }
}

pub trait MaterialInstanceBuilder {
   fn create_instance(&self) -> MaterialInstance;
}

impl MaterialInstanceBuilder for std::rc::Rc<Material> {
    fn create_instance(&self) -> MaterialInstance {
        MaterialInstance {
            base_material: self.clone()
        }
    }
}
