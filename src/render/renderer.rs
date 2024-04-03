// src/render/renderer.rs

use super::context;
use crate::render::context::RenderContext;
use std::sync::Arc;

pub struct Renderer {
    pub window: Arc<winit::window::Window>,
    pub context: RenderContext<'static>,
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
