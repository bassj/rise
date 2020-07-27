use winit::{
    event::*,
    event_loop::{EventLoop, ControlFlow},
    window::{Window, WindowBuilder},
};

pub trait Application {
    fn new(render_context: &mut crate::graphics::RenderContext) -> Self;
    fn update(&self, delta: f32);
    fn render(&self, render_context: &mut crate::graphics::RenderContext);
    fn process_input(&self, event: &Event<()>);
}

pub struct ApplicationEnvironment {
    window: Window,
    event_loop: EventLoop<()>,
}

fn build_environment() -> (Window, EventLoop<()>) {
    //Build the window.
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(winit::dpi::PhysicalSize::new(800, 600))
        .with_resizable(false)
        .with_title("RISE Window Title")
        .build(&event_loop)
        .unwrap();
    
    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;

        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.body())
            .and_then(|body| body.append_child(&web_sys::Element::from(window.canvas()))
                                 .ok());
    }

    (window, event_loop)
}

///The main entry point for any rise application.
pub fn run_application<A: 'static + Application>() {
    
    //Build our application environment.
    let (window, event_loop) = build_environment();

    //Set up the renderering context.
    use futures::executor::block_on;
    let mut render_context = block_on(crate::graphics::RenderContext::create(&window));

    //Initialize the application
    let app : A =  A::new(&mut render_context);

    //Start the main loop.
    use std::time::Instant;
    let mut last_frame = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        
        //If we dont do this, apparently rust won't clean up all the wgpu stuff properly.
        let _ = (
            &render_context,
            &app
        );

        match event {
            Event::WindowEvent {
                ref event,
                window_id
            } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    WindowEvent::Resized(physical_size) => {
                        render_context.resize(*physical_size)
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        render_context.resize(**new_inner_size);
                    }
                    _ => {},
                }
            },
            Event::RedrawRequested(_) => {
                let frame_time = Instant::now();
                let delta = frame_time - last_frame;
                last_frame = frame_time;
                let delta_flt: f32 = delta.as_secs_f32();

                app.update(delta_flt);


                app.render(&mut render_context);
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => ()
        }
    });
}