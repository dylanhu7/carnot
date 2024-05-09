use crate::builtins::systems::ActiveCamera;
use crate::ecs::query::Query;
use crate::ecs::resource::ResMut;
use crate::ecs::system::{BoxedSystem, IntoSystem, System, SystemParam};
use crate::ecs::World;
use crate::graphics::PerspectiveCamera;
use crate::input::InputState;
use crate::render::Renderer;
use std::sync::Arc;
use tokio::runtime::Runtime;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ControlFlow,
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
};

pub struct App {
    title: String,
    pub window: Option<Arc<Window>>,
    pub world: World,
    pub systems: Vec<BoxedSystem>,
}

impl Default for App {
    fn default() -> Self {
        let mut world = World::new();
        world.add_resource(InputState::default());
        Self {
            title: "Carnot Application".to_string(),
            window: Default::default(),
            world,
            systems: Default::default(),
        }
    }
}

impl App {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            ..Default::default()
        }
    }

    pub fn init_renderer(window: Arc<Window>) -> Renderer<'static> {
        let rt = Runtime::new().unwrap();
        rt.block_on(async { Renderer::new(window).await })
    }

    pub fn add_system<F: IntoSystem<Params>, Params: SystemParam>(&mut self, function: F) {
        self.systems.push(Box::new(function.into_system()));
    }

    pub fn run(mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        let _ = event_loop.run_app(&mut self);
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut attributes = Window::default_attributes();
        attributes.title.clone_from(&self.title);
        let window = Arc::new(event_loop.create_window(attributes).unwrap());
        let renderer = Self::init_renderer(window.clone());
        self.world.add_resource::<Renderer>(renderer);
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::RedrawRequested => {
                for system in self.systems.iter_mut() {
                    system.run(&mut self.world);
                }
                // self.renderer.as_ref().unwrap().window.request_redraw();
                (|renderer: ResMut<Renderer>| renderer.window.request_redraw())
                    .into_system()
                    .run(&mut self.world);
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == winit::event::ElementState::Pressed {
                    self.world
                        .get_resource_mut::<InputState>()
                        .unwrap()
                        .keys
                        .insert(event.logical_key);
                } else {
                    self.world
                        .get_resource_mut::<InputState>()
                        .unwrap()
                        .keys
                        .remove(&event.logical_key);
                }
            }
            // WindowEvent::CursorMoved { position, .. } => {
            //     self.world
            //         .get_resource_mut::<InputState>()
            //         .unwrap()
            //         .last_mouse_position = Some(
            //         self.world
            //             .get_resource_mut::<InputState>()
            //             .unwrap()
            //             .mouse_position,
            //     );
            //     self.world
            //         .get_resource_mut::<InputState>()
            //         .unwrap()
            //         .mouse_position = position;
            // }
            WindowEvent::CloseRequested => {
                // Close the window
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                (move |mut renderer: ResMut<Renderer>| renderer.resize(physical_size))
                    .into_system()
                    .run(&mut self.world);
                // self.renderer.as_mut().unwrap().resize(physical_size);
                let query = Query::<(&ActiveCamera, &PerspectiveCamera)>::fetch(&self.world);
                let (_, camera) = query.into_iter().next().unwrap();
                let mut camera = (*camera).borrow_mut();
                camera
                    .update_aspect_ratio(physical_size.width as f32 / physical_size.height as f32);
            }
            WindowEvent::ScaleFactorChanged { .. } => {
                // Update the window size
                // ...
            }
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        match event {
            winit::event::DeviceEvent::MouseMotion { delta } => {
                self.world
                    .get_resource_mut::<InputState>()
                    .unwrap()
                    .mouse_delta = delta;
            }
            winit::event::DeviceEvent::MouseWheel { delta } => {
                self.world
                    .get_resource_mut::<InputState>()
                    .unwrap()
                    .mouse_wheel_delta = delta;
            }
            _ => {}
        }
    }
}
