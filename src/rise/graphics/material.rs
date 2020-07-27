pub struct Material {
    render_pipeline: std::rc::Rc<wgpu::RenderPipeline>
}

impl Material {
    pub fn new(render_context: &crate::graphics::RenderContext) -> Material {
        let vs_src = std::fs::read_to_string("res/shader/standard.vert").unwrap();
        let fs_src = std::fs::read_to_string("res/shader/standard.frag").unwrap();

        let mut compiler = shaderc::Compiler::new().unwrap();
        let vs_spirv = compiler
            .compile_into_spirv(
                &vs_src,
                shaderc::ShaderKind::Vertex,
                "standard.vert",
                "main",
                None,
            )
            .unwrap();
        let fs_spirv = compiler
            .compile_into_spirv(
               &fs_src,
                shaderc::ShaderKind::Fragment,
                "standard.frag",
                "main",
                None,
            )
            .unwrap();

        let vs_data = wgpu::read_spirv(std::io::Cursor::new(vs_spirv.as_binary_u8())).unwrap();
        let fs_data = wgpu::read_spirv(std::io::Cursor::new(fs_spirv.as_binary_u8())).unwrap();

        let vs_module = render_context.device.create_shader_module(&vs_data);
        let fs_module = render_context.device.create_shader_module(&fs_data);


        /*let uniform_bind_group_layout = render_context.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                    },
                }
            ],
            label: Some("uniform_bind_group_layout"),
        });*/

        let render_pipeline_layout =
            render_context.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    bind_group_layouts: &[],
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
                depth_stencil_state: None,
                vertex_state: wgpu::VertexStateDescriptor {
                    index_format: wgpu::IndexFormat::Uint16,
                    vertex_buffers: &[Vertex::desc()],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            });

        Material {
            render_pipeline: std::rc::Rc::new(render_pipeline)
        }
    }

    pub fn create_instance(&self) -> MaterialInstance {
        MaterialInstance {
            render_pipeline: self.render_pipeline.clone()
        }
    }
}

pub struct MaterialInstance {
    render_pipeline: std::rc::Rc<wgpu::RenderPipeline>   
}

impl<'mat> MaterialInstance {
    pub fn get_render_pipeline(&self) -> &wgpu::RenderPipeline {
        self.render_pipeline.as_ref()
    }
}