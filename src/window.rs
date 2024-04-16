use std::sync::Arc;

use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct Window {
    pub event_loop: EventLoop<()>,
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
}
