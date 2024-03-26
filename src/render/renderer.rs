// src/render/renderer.rs

use super::context;
use std::sync::Arc;

pub struct Renderer {
    window: Arc<winit::window::Window>,
    context: context::RenderContext<'static>,
}

impl Renderer {
    pub async fn new(window: &Arc<winit::window::Window>) -> Self {
        let context = context::RenderContext::new(window).await;
        Self {
            window: window.clone(),
            context,
        }
    }
}
