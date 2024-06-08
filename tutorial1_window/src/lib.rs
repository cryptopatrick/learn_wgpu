#![allow(deprecated)]

use winit::{
    event::*,
    event_loop::EventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::WindowBuilder,
};

pub fn run() {
    // We have to enable logging via env_logger::init() below.
    // The reason is; when wgpu hits any error, it will panic with a generic message
    // but the *real* error is sent to the log crate.
    // So, if we fail to include env_logger::init(), wgpu will fail silently.
    env_logger::init();

    // Create a window and keep it open until the user closes it or presses escape. 
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    

    event_loop.run(move |event, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::Escape),
                        ..
                    },
                ..
            } => control_flow.exit(),
            _ => {}
        },
        _ => {}
    });
}

