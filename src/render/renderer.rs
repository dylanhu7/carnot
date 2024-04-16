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

    pub fn create_command_encoder(&self, label: Option<&str>) -> wgpu::CommandEncoder {
        self.context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label })
    }

    pub fn get_current_surface_texture(&self) -> wgpu::SurfaceTexture {
        self.context.surface.get_current_texture().unwrap()
    }

    pub fn get_current_texture_view(surface_texture: &wgpu::SurfaceTexture) -> wgpu::TextureView {
        surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default())
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.context.resize(new_size);
        self.window.request_redraw();
    }
}
