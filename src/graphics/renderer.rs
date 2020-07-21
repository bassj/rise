use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::dpi::PhysicalSize;
use winit::window::Window;

use wgpu::{Surface, Adapter, Device, Queue, SwapChainDescriptor, SwapChain};

use super::mesh::Mesh;
use super::material::Material;
use super::material::MaterialInstance;

pub struct RenderObject {
    
}

impl RenderObject {

}

pub struct Renderer {
    surface: Surface,
    adapter: Adapter,
    device: Device,
    queue: Queue,
    sc_desc: SwapChainDescriptor,
    swap_chain: SwapChain,
    size: PhysicalSize<u32>
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
            wgpu::BackendBit::PRIMARY
        ).await.unwrap();

        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: Default::default(),
        }).await;

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
            swap_chain
        }
    }

    pub fn create_object(&self, mesh : Mesh, material : MaterialInstance) -> RenderObject {
        unimplemented!();
    }

    //For the time being, our clear will just be a render pass that clears the depth buffer and the frame buffer.
    pub fn render(&mut self, scene : &RenderObject) {

        let frame = self.swap_chain.get_next_texture()
        .expect("Timeout getting texture");

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Clear Render-Pass Encoder")
            }
        );

        let mut render_pass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Clear,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }
                    }
                ],
                depth_stencil_attachment: None,
            });
            

            
            //TODO:
    }

    pub fn input(&mut self, event: &WindowEvent, control_flow: &mut ControlFlow) -> bool {
        match event {
            WindowEvent::CloseRequested => {
                *control_flow = ControlFlow::Exit;
                return true;
            },
            WindowEvent::Resized(physical_size) => {
                self.resize(*physical_size);
                return true;
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                // new_inner_size is &mut so w have to dereference it twice
                self.resize(**new_inner_size);
                return true;
            },
            _ => {false}
        }
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
        self.sc_desc.width = size.width;
        self.sc_desc.height = size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }
}