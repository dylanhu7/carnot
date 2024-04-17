use crate::ecs::{system::System, World};
use crate::input::InputState;
use crate::render::Renderer;
use crate::window::Window;
use winit::{event::Event, event::WindowEvent};

pub struct App {
    window: Window,
    pub renderer: Renderer,
    pub world: World,
    systems: Vec<System>,
    input_state: InputState,
}

impl App {
    pub async fn new(width: u32, height: u32, title: &str) -> Self {
        let window = Window::new(width, height, title);
        window.window.set_cursor_visible(false);
        window
            .window
            .set_cursor_position(winit::dpi::PhysicalPosition::new(
                width as f64 / 2.0,
                height as f64 / 2.0,
            ))
            .expect("Failed to set cursor position");
        let renderer = Renderer::new(&window.window).await;

        Self {
            window,
            renderer,
            world: World::new(),
            systems: vec![],
            input_state: InputState::default(),
        }
    }

    pub fn add_system(&mut self, system: System) {
        self.systems.push(system);
    }

    pub fn run(mut self) {
        let _ = self.window.event_loop.run(move |event, elwt| {
            match event {
                Event::DeviceEvent { event, .. } => match event {
                    winit::event::DeviceEvent::MouseMotion { delta } => {
                        self.input_state.mouse_delta = delta;
                    }
                    winit::event::DeviceEvent::MouseWheel { delta } => {
                        self.input_state.mouse_wheel_delta = delta;
                    }
                    _ => {}
                },
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::RedrawRequested => {
                        for system in self.systems.iter() {
                            system(&mut self.world, &mut self.renderer, &mut self.input_state);
                        }
                        self.window.window.request_redraw();
                    }
                    WindowEvent::KeyboardInput { event, .. } => {
                        if event.state == winit::event::ElementState::Pressed {
                            self.input_state.keys.insert(event.logical_key);
                        } else {
                            self.input_state.keys.remove(&event.logical_key);
                        }
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        self.input_state.last_mouse_position =
                            Some(self.input_state.mouse_position);
                        self.input_state.mouse_position = position;
                    }
                    WindowEvent::CloseRequested => {
                        // Close the window
                        elwt.exit();
                    }
                    WindowEvent::Resized(physical_size) => {
                        self.renderer.resize(physical_size);
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
