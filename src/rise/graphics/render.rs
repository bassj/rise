use winit::{
    window::Window,
    dpi::PhysicalSize
};

use wgpu::{
    Surface,
    Adapter,
    Device,
    Queue,
    CommandEncoder,
    SwapChainDescriptor,
    SwapChainOutput,
    SwapChain
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
    pub async fn create(window: &Window) -> RenderContext {
        
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
            present_mode: wgpu::PresentMode::Fifo, //TODO:
        };

        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        RenderContext {
            surface,
            adapter,
            device,
            queue,
            sc_desc,
            swap_chain,
            size
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
        self.sc_desc.width = size.width;
        self.sc_desc.height = size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }
}

pub trait Drawable {
    fn get_material(&self) -> &crate::graphics::MaterialInstance;
    fn get_vertex_buffer(&self) -> &wgpu::Buffer;
    fn get_index_buffer(&self) -> &wgpu::Buffer;
    fn num_indices(&self) -> u32;
}

pub struct Frame<'r> {
    render_context: &'r RenderContext,
    frame: SwapChainOutput,
    encoder: CommandEncoder
}

impl<'r> Frame<'r> {
    pub fn render<D: Drawable>(&mut self, objects: &[&D], material: &crate::graphics::Material) {
        let mut render_pass = self.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[
                wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &self.frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    },
                }
            ],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                attachment: material.get_depth_texture().get_view(),
                depth_load_op: wgpu::LoadOp::Clear,
                depth_store_op: wgpu::StoreOp::Store,
                clear_depth: 1.0,
                stencil_load_op: wgpu::LoadOp::Clear,
                stencil_store_op: wgpu::StoreOp::Store,
                clear_stencil: 0
            }),
        });

        for obj in objects {
            render_pass.set_bind_group(0, material.get_uniforms().get_camera(), &[]);
            render_pass.set_pipeline(material.get_render_pipeline());
            render_pass.set_vertex_buffer(0, obj.get_vertex_buffer(), 0, 0);
            render_pass.set_index_buffer(obj.get_index_buffer(), 0, 0);

            render_pass.draw_indexed(0..obj.num_indices(), 0, 0..1);
        }
    }
}

pub fn begin_frame<'frame>(render_context: &'frame mut RenderContext) -> Frame {
    
    let encoder = render_context.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Render Encoder"),
    });

    let frame = render_context.swap_chain.get_next_texture()
        .expect("Timeout getting texture");

    Frame {
        render_context,
        frame,
        encoder,
    }
}

pub fn end_frame(frame: Frame) {
    let command_buffer = frame.encoder.finish();

    frame.render_context.queue.submit(&[command_buffer]);
}
