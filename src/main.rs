extern crate winit;

#[allow(unused_imports)]
mod graphics;
mod game;

use std::time::Instant;
use std::time::Duration;

use winit::{
    event::*,
    event_loop::{EventLoop, ControlFlow},
    window::{Window, WindowBuilder},
};

use game::Game;



fn main() {
    //TODO: Read from file.
    let env = game::GameEnvironment {
        window: game::Window {
            width: 800.,
            height: 600.,
            resizable: false,
            title: "RISE Test Window",
        }
    };
    
    //Build the window.
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(winit::dpi::PhysicalSize::new(env.window.width, env.window.height))
        .with_resizable(env.window.resizable)
        .with_title(env.window.title)
        .build(&event_loop)
        .unwrap();

    //Set up the renderer.
    //let renderer = graphics::Renderer::new();

    //Initialize the game
    //let mut game = Game::new(env, &renderer);

    //Start the main loop.
    
    let mut last_frame = Instant::now();


    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event, 
                window_id
            } if window_id == window.id() => {
                
            },
            Event::RedrawRequested(_) => {
                //game.render(&renderer);
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => ()
        }
    });

    /*el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => {
                return;
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(physical_size);
                    renderer.resize(physical_size);
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                //Calculate time since last frame.
                let frame_time = Instant::now();
                let delta = frame_time - last_frame;
                last_frame = frame_time;
                let delta_flt: f32 = delta.as_secs_f32();//Delta time in seconds.

                renderer.clear();

                //Render the frame.
                game.render(&renderer);

                //update our scene
                game.update(delta_flt);

                //update window
                windowed_context.swap_buffers().unwrap();
            },
            Event::MainEventsCleared => {
                windowed_context.window().request_redraw();
            },
            _ => {
                
                let next_frame = Instant::now() + Duration::from_millis(16); //TODO: 
                *control_flow = ControlFlow::WaitUntil(next_frame);
            },
        }
    });*/
}