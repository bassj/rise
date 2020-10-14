use winit::{dpi::PhysicalSize, window::Window};

use log::{error, info};

use wgpu::{
    Adapter, CommandEncoder, Device, Queue, Surface, SwapChain, SwapChainDescriptor,
    SwapChainTexture,
};

pub struct RenderContext {
    surface: Surface,
    adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub sc_desc: SwapChainDescriptor,
    swap_chain: SwapChain,
    size: PhysicalSize<u32>,
}

impl RenderContext {
    pub async fn create(window: &Window) -> Result<RenderContext, Box<dyn std::error::Error>> {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        /*let adapter = Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            },
            wgpu::BackendBit::PRIMARY,
        )
        .await
        .unwrap();*/

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    shader_validation: true,
                },
                None,
            )
            .await
            .unwrap();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo, //TODO:
        };

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        Ok(RenderContext {
            surface,
            adapter,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
        })
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
        self.sc_desc.width = size.width;
        self.sc_desc.height = size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    pub fn create_drawable(&self) -> DrawableBuilder {
        DrawableBuilder::new(&self)
    }

    pub fn create_material(&self) -> crate::graphics::MaterialBuilder {
        crate::graphics::MaterialBuilder::new(&self)
    }

    pub fn create_texture(&self) -> crate::graphics::TextureBuilder {
        crate::graphics::TextureBuilder::new(&self)
    }
}

pub struct DrawableBuilder<'a> {
    render_context: &'a RenderContext,
    mesh: Option<crate::graphics::Mesh>,
    material: Option<crate::graphics::MaterialInstance>,
}

impl<'a> DrawableBuilder<'a> {
    pub fn new(r: &'a RenderContext) -> Self {
        Self {
            render_context: r,
            mesh: None,
            material: Option::<crate::graphics::MaterialInstance>::None,
        }
    }

    pub fn with_mesh(mut self, mesh: crate::graphics::Mesh) -> Self {
        self.mesh = Some(mesh);
        self
    }

    pub fn with_material(mut self, material: crate::graphics::MaterialInstance) -> Self {
        self.material = Some(material);
        self
    }

    pub fn build(self) -> Drawable {
        let mut mesh = self.mesh.unwrap();
        mesh.create(&self.render_context);

        Drawable {
            material: self.material.unwrap(),
            mesh,
        }
    }
}

pub struct Drawable {
    material: crate::graphics::MaterialInstance,
    mesh: crate::graphics::Mesh,
}

impl Drawable {
    fn get_material(&self) -> &crate::graphics::MaterialInstance {
        &self.material
    }
    fn get_vertex_buffer(&self) -> &wgpu::Buffer {
        self.mesh.get_vertex_buffer().unwrap()
    }
    fn get_index_buffer(&self) -> &wgpu::Buffer {
        self.mesh.get_index_buffer().unwrap()
    }
    fn num_indices(&self) -> u32 {
        self.mesh.get_indices().len() as u32
    }
}

pub struct Frame<'r> {
    render_context: &'r RenderContext,
    frame: SwapChainTexture,
    encoder: CommandEncoder,
}

impl<'r> Frame<'r> {
    pub fn render<C: crate::core::Camera>(
        &mut self,
        objects: &[&Drawable],
        camera: &'r C,
    ) {
        //let base_material = material.get_base_material();
        

        let mut render_pass = self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &self.frame.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                attachment: camera.get_depth_texture().get_view(),
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: true,
                }),
                stencil_ops: None,
            }),
        });

        for obj in objects {
            let material = obj.get_material();
            let base_material = material.get_base_material();

            camera.update(self.render_context);
            
            let camera_binding = camera.get_binding().unwrap();
            let camera_bind_index = *base_material.get_binding_by_name("camera").expect("Shader must have a camera uniform.");

            render_pass.set_bind_group(camera_bind_index as u32, camera_binding.get_bind_group(), &[]);

            for i in 0..base_material.get_uniforms().len() {
                if let Some(binding) = material.get_uniforms().get(&i) {
                    let bind_group = binding.get_bind_group();

                    //println!("Binding: {}");

                    render_pass.set_bind_group(i as u32, &bind_group, &[]);
                }
            }
            render_pass.set_pipeline(base_material.get_render_pipeline());
            render_pass.set_vertex_buffer(0, obj.get_vertex_buffer().slice(..));
            render_pass.set_index_buffer(obj.get_index_buffer().slice(..));

            render_pass.draw_indexed(0..obj.num_indices(), 0, 0..1);
        }
    }
}

pub fn begin_frame<'frame>(render_context: &'frame mut RenderContext) -> Frame {
    let encoder = render_context
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

    let frame = render_context
        .swap_chain
        .get_current_frame()
        .expect("Timeout getting texture")
        .output;

    Frame {
        render_context,
        frame,
        encoder,
    }
}

pub fn end_frame(frame: Frame) {
    let command_buffer = frame.encoder.finish();

    frame
        .render_context
        .queue
        .submit(std::iter::once(command_buffer));
}
