use std::sync::Arc;

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use crate::{ecs::World, render::Renderer};

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

    pub fn run(self, world: &mut World, _renderer: &mut Renderer) {
        let _ = self.event_loop.run(move |event, elwt| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::RedrawRequested => {
                        world.update();
                    }
                    WindowEvent::CloseRequested => {
                        // Close the window
                        elwt.exit();
                    }
                    WindowEvent::Resized(_) => {
                        // Update the window size
                        // ...
                    }
                    WindowEvent::ScaleFactorChanged { .. } => {
                        // Update the window size
                        // ...
                    }
                    _ => {}
                },
                Event::AboutToWait { .. } => {}
                _ => {}
            }
        });
    }
}
