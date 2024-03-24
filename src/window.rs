use std::sync::Arc;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::WindowBuilder,
};

pub struct Window {
    event_loop: EventLoop<()>,
    pub window: Arc<winit::window::Window>,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(width, height))
            .build(&event_loop)
            .unwrap();

        Self {
            event_loop,
            window: Arc::new(window),
        }
    }

    pub fn run(mut self) {
        let _ = self
            .event_loop
            .run(move |event, elwt: &EventLoopWindowTarget<()>| {
                match event {
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => {
                            // Close the window
                            elwt.exit();
                        }
                        WindowEvent::Resized(size) => {
                            // Update the window size
                            // ...
                        }
                        WindowEvent::ScaleFactorChanged {
                            scale_factor,
                            inner_size_writer,
                        } => {
                            // Update the window size
                            // ...
                        }
                        _ => {}
                    },
                    _ => {}
                }
            });
    }
}
