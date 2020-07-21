extern crate winit;


mod graphics;
mod game;

use std::time::Instant;

#[allow(unused_imports)]
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
    use futures::executor::block_on;
    let mut renderer = block_on(graphics::Renderer::new(&window));

    //Initialize the game
    let mut game = Game::new(env, &renderer);


    //Start the main loop.
    
    let mut last_frame = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event, 
                window_id
            } if window_id == window.id() => if !renderer.input(event, control_flow) {
                //Process input.
            },
            Event::RedrawRequested(_) => {
                let frame_time = Instant::now();
                let delta = frame_time - last_frame;
                last_frame = frame_time;
                let delta_flt: f32 = delta.as_secs_f32();

                game.render(&mut renderer);
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => ()
        }
    });
}