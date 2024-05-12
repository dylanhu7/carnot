use crate::builtins::systems::camera::{camera_startup_system, camera_update_system};
use crate::builtins::systems::render::{render_startup_system, render_system};
use crate::builtins::systems::ActiveCamera;
use crate::ecs::query::Query;
use crate::ecs::resource::ResMut;
use crate::ecs::system::{BoxedSystem, IntoSystem, System, SystemOrWorldParam, SystemParam};
use crate::ecs::World;
use crate::graphics::PerspectiveCamera;
use crate::input::InputState;
use crate::render::Renderer;
use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ControlFlow,
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemStage {
    Startup,
    Update,
}

use SystemStage::*;

pub struct App {
    title: String,
    pub window: Option<Arc<Window>>,
    pub world: World,
    pub startup_systems: Vec<BoxedSystem>,
    pub systems: Vec<BoxedSystem>,
}

impl Default for App {
    fn default() -> Self {
        let mut world = World::new();
        world.add_resource(InputState::default());
        Self {
            title: "Carnot Application".to_string(),
            world,
            window: Default::default(),
            startup_systems: Default::default(),
            systems: Default::default(),
        }
    }
}

impl App {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    /// Adds the default systems to the application.
    ///
    /// The default systems are:
    /// - [`camera_startup_system`]
    ///   - Provides a camera entity centered at origin looking down -Z composed of:
    ///     - [`PerspectiveCamera`]
    ///     - [`Transform`](crate::graphics::Transform)
    ///     - [`ActiveCamera`](crate::builtins::systems::ActiveCamera)
    /// - [`camera_update_system`]
    /// - [`render_system`]
    ///   - Renders all entities with a [`Mesh`] and [`Transform`] component using the [`ActiveCamera`]
    pub fn with_default_systems(self) -> Self {
        self.with_system(Startup, camera_startup_system)
            .with_system(Startup, render_startup_system)
            .with_system(Update, camera_update_system)
            .with_system(Update, render_system)
    }

    pub fn with_system<F: IntoSystem<M>, M: SystemOrWorldParam>(
        mut self,
        stage: SystemStage,
        function: F,
    ) -> Self {
        match stage {
            Startup => self.startup_systems.push(Box::new(function.into_system())),
            Update => self.systems.push(Box::new(function.into_system())),
        }
        self
    }

    pub fn run(mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        let _ = event_loop.run_app(&mut self);
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = Window::default_attributes()
            .with_title(self.title.clone())
            .with_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
        let window = Arc::new(event_loop.create_window(attributes).unwrap());
        self.window = Some(window);
        self.world
            .add_resource::<Arc<Window>>(self.window.clone().unwrap());
        for system in self.startup_systems.iter_mut() {
            system.run(&mut self.world);
        }
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
                self.world
                    .get_resource_mut::<Renderer>()
                    .unwrap()
                    .resize(physical_size);
                let mut query =
                    <Query<(&mut PerspectiveCamera, &ActiveCamera)> as SystemParam>::fetch(
                        &self.world,
                    );
                if let Some((camera, _)) = (&mut query).into_iter().next() {
                    camera.update_aspect_ratio(
                        physical_size.width as f32 / physical_size.height as f32,
                    );
                }
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
