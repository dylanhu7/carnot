use crate::ecs::World;
use crate::render::Renderer;
use crate::window::Window;
use winit::{event::Event, event::WindowEvent};

pub struct App {
    window: Window,
    pub renderer: Renderer,
    pub world: World,
    systems: Vec<Box<dyn FnMut(&mut World, &mut Renderer)>>,
}

impl App {
    pub async fn new(width: u32, height: u32, title: &str) -> Self {
        let window = Window::new(width, height, title);
        let renderer = Renderer::new(&window.window).await;

        Self {
            window,
            renderer,
            world: World::new(),
            systems: vec![],
        }
    }

    pub fn add_system(&mut self, system: Box<dyn FnMut(&mut World, &mut Renderer)>) {
        self.systems.push(system);
    }

    pub fn run(mut self) {
        let _ = self.window.event_loop.run(move |event, elwt| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::RedrawRequested => {
                        self.world.update();
                        for system in self.systems.iter_mut() {
                            system(&mut self.world, &mut self.renderer);
                        }
                        self.window.window.request_redraw();
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
