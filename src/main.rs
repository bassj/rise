extern crate glutin;
extern crate gl;

mod renderer;

use renderer::*;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

struct GLShader {
    shaderHandle: u32
}

trait Shader {
    fn create(&mut self, renderer : &GLRenderer);

    fn bind(&self, renderer: &GLRenderer);
}

fn main() {
    //Build the window.
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("A fantastic window!");

    let windowed_context =
        ContextBuilder::new().build_windowed(wb, &el).unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    //Set up the renderer.

    let renderer = GLRenderer::new(&windowed_context);

    //Create our mesh.

    let mut mesh: Mesh = Mesh::new();

    mesh.add_vert((0., 0.));
    mesh.add_vert((1., 0.));
    mesh.add_vert((1., 1.));

    mesh.set_indices(vec![0, 1, 2]);

    let mut drawable: Drawable = Drawable::new();

    drawable.set_mesh(mesh);

    drawable.create(&renderer);
    
    //Start the main loop.

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => {
                //Cleanup.
                drawable.destroy(&renderer);

                return
            },
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(physical_size);
                    renderer.resize(physical_size);
                }
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                renderer.clear();

                drawable.render(&renderer);
                
                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}