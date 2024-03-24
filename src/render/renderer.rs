use std::sync::Arc;

use super::context;

pub struct Renderer {
    context: context::RenderContext<'static>,
}

impl Renderer {
    pub async fn new(window: &Arc<winit::window::Window>) -> Self {
        let context = context::RenderContext::new(window).await;
        Self { context }
    }
}
