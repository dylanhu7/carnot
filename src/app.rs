use crate::ecs::World;
use crate::render::Renderer;
use crate::window::Window;

pub struct App {
    window: Window,
    renderer: Renderer,
    world: World,
}

impl App {
    pub async fn new(width: u32, height: u32, title: &str) -> Self {
        let window = Window::new(width, height, title);
        let renderer = Renderer::new(&window.window).await;
        Self {
            window,
            renderer,
            world: World::new(),
        }
    }

    pub fn run(mut self) {
        self.window.run(&mut self.world, &mut self.renderer);
    }
}
