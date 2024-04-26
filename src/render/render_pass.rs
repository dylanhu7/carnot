#[derive(Default)]
pub struct RenderPassBuilder<'pass> {
    color_attachments: Vec<Option<wgpu::RenderPassColorAttachment<'pass>>>,
    depth_stencil_attachment: Option<wgpu::RenderPassDepthStencilAttachment<'pass>>,
}

impl<'pass> RenderPassBuilder<'pass> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn color_attachment(mut self, view: &'pass wgpu::TextureView) -> Self {
        self.color_attachments
            .push(Some(wgpu::RenderPassColorAttachment {
                view,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
                resolve_target: None,
            }));
        self
    }

    pub fn depth_stencil_attachment(mut self, view: &'pass wgpu::TextureView) -> Self {
        self.depth_stencil_attachment = Some(wgpu::RenderPassDepthStencilAttachment {
            view,
            depth_ops: Some(wgpu::Operations {
                load: wgpu::LoadOp::Clear(1.0),
                store: wgpu::StoreOp::Store,
            }),
            stencil_ops: None,
        });
        self
    }

    pub fn begin_render_pass(
        self,
        encoder: &'pass mut wgpu::CommandEncoder,
        label: Option<&str>,
    ) -> wgpu::RenderPass<'pass> {
        let RenderPassBuilder {
            color_attachments,
            depth_stencil_attachment,
        } = self;
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &color_attachments,
            depth_stencil_attachment,
            occlusion_query_set: None,
            label,
            timestamp_writes: None,
        })
    }
}
