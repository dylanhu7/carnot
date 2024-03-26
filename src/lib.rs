// src/lib.rs

pub mod render;
pub mod window;

pub use render::Renderer;
pub use window::Window;

pub struct App {
    window: Window,
    renderer: Renderer,
}

impl App {
    pub async fn new(width: u32, height: u32, title: &str) -> Self {
        let window = Window::new(width, height, title);
        let renderer = Renderer::new(&window.window).await;
        Self { window, renderer }
    }

    pub fn run(mut self) {
        self.window.run();
    }
}
