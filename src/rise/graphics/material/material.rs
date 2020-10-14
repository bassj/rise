pub struct Material {
    render_pipeline: wgpu::RenderPipeline,
    uniforms: Vec<super::Uniform>,
    uniform_names: HashMap<String, usize>
}

impl Material {
    pub fn get_render_pipeline(&self) -> &wgpu::RenderPipeline {
        &self.render_pipeline
    }

    pub fn get_uniforms(&self) -> &Vec<super::Uniform> {
        &self.uniforms
    }

    pub fn get_camera_uniform(&self) -> &super::Uniform {
        &self.uniforms[self.uniform_names["camera"]]
    }

    pub fn get_binding_by_name(&self, name: &str) -> Option<&usize> {
        self.uniform_names.get(name)
    }
}

use std::collections::HashMap;

pub struct MaterialBuilder<'a> {
    render_context: &'a crate::graphics::RenderContext,
    vertex_stage: Option<Vec<u8>>,
    fragment_stage: Option<Vec<u8>>,
    uniforms: Vec<super::Uniform>,
    uniform_map: HashMap<String, usize>
}

impl<'a> MaterialBuilder<'a> {
    pub fn new(r: &'a crate::graphics::RenderContext) -> Self {
        Self {
            render_context: r,
            vertex_stage: None,
            fragment_stage: None,
            uniforms: Vec::new(),
            uniform_map: HashMap::new(),
        }
    }

    pub fn from_file<P: AsRef<std::path::Path>>(mut self, p: P) -> Self {
        let material_src =
            std::fs::read_to_string(p).expect("Invalid path when creating material.");

        let material_layout: serde_json::Value =
            serde_json::from_str(&material_src).expect("Unable to parse material.");

        let vs_path = material_layout["vertex_stage"].as_str().unwrap();
        let fs_path = material_layout["fragment_stage"].as_str().unwrap();

        let fs_spirv = std::fs::read(fs_path).unwrap();
        let vs_spirv = std::fs::read(vs_path).unwrap();
        self.fragment_stage = Some(fs_spirv);
        self.vertex_stage = Some(vs_spirv);

        let uniform_descriptors = material_layout["uniforms"].as_array().unwrap();

        for value in uniform_descriptors {
            let name = value["name"].as_str().unwrap();
            let uniform_type = value["type"].as_str().unwrap();
            // Big ol todo.


            let bind_group_layout = if uniform_type == "camera" {
                self.render_context.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStage::VERTEX,
                            ty: wgpu::BindingType::UniformBuffer {
                                dynamic: false,
                                min_binding_size: None
                            },
                            count: None
                        }
                    ],
                    label: Some(name),
                })
            } else {
                self.render_context.device.create_bind_group_layout(
                    &wgpu::BindGroupLayoutDescriptor {
                        entries: &[
                            wgpu::BindGroupLayoutEntry {
                                binding: 0,
                                visibility: wgpu::ShaderStage::FRAGMENT,
                                ty: wgpu::BindingType::SampledTexture {
                                    multisampled: false,
                                    dimension: wgpu::TextureViewDimension::D2,
                                    component_type: wgpu::TextureComponentType::Uint,
                                },
                                count: None,
                            },
                            wgpu::BindGroupLayoutEntry {
                                binding: 1,
                                visibility: wgpu::ShaderStage::FRAGMENT,
                                ty: wgpu::BindingType::Sampler {
                                    comparison: false,
                                },
                                count: None,
                            },
                        ],
                        label: Some(name),
                    })
            };

            self.uniforms.push(super::Uniform::new(name, bind_group_layout));

            self.uniform_map.insert(name.to_string(), self.uniforms.len() - 1);
        }

        self
    }

    pub fn build(self) -> Material {
        let vs_spirv = self
            .vertex_stage
            .expect("Attempted to build material without vertex stage.");
        let fs_spirv = self
            .fragment_stage
            .expect("Attempted to build material without fragment stage.");

        let vs_module = self
            .render_context
            .device
            .create_shader_module(wgpu::util::make_spirv(&vs_spirv));
        let fs_module = self
            .render_context
            .device
            .create_shader_module(wgpu::util::make_spirv(&fs_spirv));

        let uniform_layouts: Vec<&wgpu::BindGroupLayout> = 
            self.uniforms.iter()
            .map(|uniform| uniform.get_bind_group_layout())
            .collect();

        let render_pipeline_layout =
            &self
                .render_context
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Default Render Pipeline Layout"),
                    push_constant_ranges: &[],
                    bind_group_layouts: &uniform_layouts[..],
                });

        use crate::graphics::Vertex;

        let render_pipeline =
            self.render_context
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Default RISE Render Pipeline"),
                    layout: Some(&render_pipeline_layout),
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
                        clamp_depth: false,
                    }),
                    color_states: &[wgpu::ColorStateDescriptor {
                        format: self.render_context.sc_desc.format,
                        color_blend: wgpu::BlendDescriptor::REPLACE,
                        alpha_blend: wgpu::BlendDescriptor::REPLACE,
                        write_mask: wgpu::ColorWrite::ALL,
                    }],

                    primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                    depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
                        format: wgpu::TextureFormat::Depth32Float,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::LessEqual,
                        stencil: wgpu::StencilStateDescriptor::default(),
                    }),
                    vertex_state: wgpu::VertexStateDescriptor {
                        index_format: wgpu::IndexFormat::Uint16,
                        vertex_buffers: &[Vertex::desc()],
                    },
                    sample_count: 1,
                    sample_mask: !0,
                    alpha_to_coverage_enabled: false,
                });

        Material {
            render_pipeline,
            uniforms: self.uniforms,
            uniform_names: self.uniform_map
        }
    }
}
