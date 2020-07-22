use std::path::Path;

use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::window::Window;

use wgpu::{Adapter, Device, Queue, Surface, SwapChain, SwapChainDescriptor};

use super::material::Material;
use super::mesh::Mesh;

use serde_json::Result;

pub struct RenderObject {
    num_vertices: u32,
    num_indices: u32,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
}

impl RenderObject {}

pub struct Renderer {
    surface: Surface,
    adapter: Adapter,
    device: Device,
    queue: Queue,
    sc_desc: SwapChainDescriptor,
    swap_chain: SwapChain,
    size: PhysicalSize<u32>,
    active_material: Option<Material>,
}

impl Renderer {
    pub async fn new(window: &Window) -> Renderer {
        let size = window.inner_size();
        let surface = Surface::create(window);

        let adapter = Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            },
            wgpu::BackendBit::PRIMARY,
        )
        .await
        .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                extensions: wgpu::Extensions {
                    anisotropic_filtering: false,
                },
                limits: Default::default(),
            })
            .await;

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        Renderer {
            size,
            surface,
            adapter,
            device,
            queue,
            sc_desc,
            swap_chain,
            active_material: None,
        }
    }

    pub fn use_material(&mut self, material: Material) {
        self.active_material = Some(material);
    }

    //For the time being, our clear will just be a render pass that clears the depth buffer and the frame buffer.
    pub fn render(&mut self, scene: &RenderObject) {
        let frame = self
            .swap_chain
            .get_next_texture()
            .expect("Timeout getting texture");

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Clear Render-Pass Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    },
                }],
                depth_stencil_attachment: None,
            });

            let mat = self.active_material.as_ref().unwrap();

            render_pass.set_vertex_buffer(0, &scene.vertex_buffer, 0, 0);
            render_pass.set_index_buffer(&scene.index_buffer, 0, 0);
            render_pass.set_pipeline(&mat.render_pipeline);
            render_pass.draw_indexed(0..scene.num_indices, 0, 0..1);
        }
        self.queue.submit(&[encoder.finish()])
    }

    pub fn input(&mut self, event: &WindowEvent, control_flow: &mut ControlFlow) -> bool {
        match event {
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
                return true;
            }
            WindowEvent::Resized(physical_size) => {
                self.resize(*physical_size);
                return true;
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                // new_inner_size is &mut so w have to dereference it twice
                self.resize(**new_inner_size);
                return true;
            }
            _ => false,
        }
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
        self.sc_desc.width = size.width;
        self.sc_desc.height = size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }
}

pub trait RenderObjectBuilder {
    fn create_object(&self, mesh: &Mesh) -> RenderObject;
}

impl RenderObjectBuilder for Renderer {
    fn create_object(&self, mesh: &Mesh) -> RenderObject {
        //Convert our mesh into gpu buffers.
        let vertex_buffer = self.device.create_buffer_with_data(
            bytemuck::cast_slice(&mesh.vertices[..]),
            wgpu::BufferUsage::VERTEX,
        );

        let index_buffer = self.device.create_buffer_with_data(
            bytemuck::cast_slice(&mesh.indices[..]),
            wgpu::BufferUsage::INDEX,
        );

        let num_vertices = mesh.vertices.len() as u32;
        let num_indices = mesh.indices.len() as u32;
        RenderObject {
            vertex_buffer,
            index_buffer,
            num_vertices,
            num_indices,
        }
    }
}

use super::material::MaterialBuilder;

impl MaterialBuilder for Renderer {
    fn create_material<P: AsRef<Path>>(&self, material: P) -> Material {

        let shader_directory = (material.as_ref() as &Path).parent().unwrap();

        let material_layout : serde_json::Value = serde_json::from_str(&std::fs::read_to_string(material.as_ref() as &Path).unwrap()).unwrap();

        let vs_path = shader_directory.join(material_layout["vertex"]["source"].as_str().unwrap());
        let fs_path = shader_directory.join(material_layout["fragment"]["source"].as_str().unwrap());

        let vs_src = std::fs::read_to_string(&vs_path).unwrap();
        let fs_src = std::fs::read_to_string(&fs_path).unwrap();

        let mut compiler = shaderc::Compiler::new().unwrap();
        let vs_spirv = compiler
            .compile_into_spirv(
                &vs_src,
                shaderc::ShaderKind::Vertex,
                vs_path.file_name().unwrap().to_str().unwrap(),
                "main",
                None,
            )
            .unwrap();
        let fs_spirv = compiler
            .compile_into_spirv(
               &fs_src,
                shaderc::ShaderKind::Fragment,
                fs_path.file_name().unwrap().to_str().unwrap(),
                "main",
                None,
            )
            .unwrap();

        let vs_data = wgpu::read_spirv(std::io::Cursor::new(vs_spirv.as_binary_u8())).unwrap();
        let fs_data = wgpu::read_spirv(std::io::Cursor::new(fs_spirv.as_binary_u8())).unwrap();

        let vs_module = self.device.create_shader_module(&vs_data);
        let fs_module = self.device.create_shader_module(&fs_data);

        let render_pipeline_layout =
            self.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    bind_group_layouts: &[],
                });
        use super::mesh::Vertex;

        let render_pipeline = self
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
                    format: self.sc_desc.format,
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
                alpha_to_coverage_enabled: true,
            });

        return Material::new(render_pipeline);
    }
}
