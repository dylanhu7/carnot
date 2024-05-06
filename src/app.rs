use crate::builtins::systems::ActiveCamera;
use crate::ecs::system::{BoxedSystem, IntoSystem, SystemParam};
use crate::ecs::{system::System, World};
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

pub struct App<'a> {
    title: String,
    pub window: Option<Arc<Window>>,
    pub renderer: Option<Renderer<'a>>,
    pub world: World,
    pub systems: Vec<BoxedSystem>,
    pub input_state: InputState,
}

impl Default for App<'_> {
    fn default() -> Self {
        Self {
            title: "Carnot Application".to_string(),
            window: Default::default(),
            renderer: Default::default(),
            world: Default::default(),
            systems: Default::default(),
            input_state: Default::default(),
        }
    }
}

impl<'a> App<'a> {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            ..Default::default()
        }
    }

    fn init_renderer(window: Arc<Window>) -> Renderer<'a> {
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

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let mut attributes = Window::default_attributes();
        attributes.title = self.title.clone();
        let window = Arc::new(event_loop.create_window(attributes).unwrap());
        self.renderer = Some(Self::init_renderer(window.clone()));
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
                self.renderer.as_ref().unwrap().window.request_redraw();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == winit::event::ElementState::Pressed {
                    self.input_state.keys.insert(event.logical_key);
                } else {
                    self.input_state.keys.remove(&event.logical_key);
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.input_state.last_mouse_position = Some(self.input_state.mouse_position);
                self.input_state.mouse_position = position;
            }
            WindowEvent::CloseRequested => {
                // Close the window
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                self.renderer.as_mut().unwrap().resize(physical_size);
                let mut active_camera_vec = self
                    .world
                    .borrow_component_vec_mut::<ActiveCamera>()
                    .unwrap();
                let mut perspective_camera_vec = self
                    .world
                    .borrow_component_vec_mut::<PerspectiveCamera>()
                    .unwrap();
                let (_, camera) = active_camera_vec
                    .iter_mut()
                    .zip(perspective_camera_vec.iter_mut())
                    .filter(|(_, camera)| camera.is_some())
                    .find_map(|(active, camera)| Some((active.as_mut()?, camera.as_mut()?)))
                    .expect("No active camera found");
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
                self.input_state.mouse_delta = delta;
            }
            winit::event::DeviceEvent::MouseWheel { delta } => {
                self.input_state.mouse_wheel_delta = delta;
            }
            _ => {}
        }
    }
}
