use std::sync::Arc;

use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::WindowAttributes,
};

pub struct Window {
    pub event_loop: EventLoop<()>,
    pub window: Arc<winit::window::Window>,
    pub monitor: winit::monitor::MonitorHandle,
    pub video_mode: winit::monitor::VideoModeHandle,
    pub video_modes: Vec<winit::monitor::VideoModeHandle>,
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        let monitor = event_loop.owned_display_handle();
        let video_modes = monitor.video_modes().collect::<Vec<_>>();
        let video_mode = video_modes.first().unwrap().clone();
        // for mode in &video_modes {
        //     println!(
        //         "VideoMode: {}x{}@{}",
        //         mode.size().width,
        //         mode.size().height,
        //         mode.refresh_rate_millihertz()
        //     );
        // }
        let window = WindowAttributes::new()
            .with_title(title)
            // .with_maximized(true)
            // .with_fullscreen(Some(winit::window::Fullscreen::Exclusive(video_mode)))
            // .with_fullscreen(Some(winit::window::Fullscreen::Borderless(Some(
            // monitor.clone(),
            // ))))
            .with_inner_size(winit::dpi::LogicalSize::new(width, height))
            .build(&event_loop)
            .unwrap();

        Self {
            event_loop,
            window: Arc::new(window),
            monitor,
            video_mode: video_mode.clone(),
            video_modes,
        }
    }
}
