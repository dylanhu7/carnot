use wgpu::util::DeviceExt;

use crate::ecs::query::Query;
use crate::ecs::resource::ResMut;
use crate::graphics::camera::CameraUniform;
use crate::graphics::mesh::MeshVertex;
use crate::graphics::transform::Mat4Uniform;
use crate::graphics::{Mesh, PerspectiveCamera, Transform};
use crate::render::render_pass::RenderPassBuilder;
use crate::render::texture;
use crate::render::vertex::Vertex;
use crate::render::Renderer;

pub struct ActiveCamera;

pub fn render_system(
    renderer: ResMut<Renderer>,
    models: Query<(&Mesh, &Transform)>,
    camera: Query<(&PerspectiveCamera, &Transform, &ActiveCamera)>,
) {
    let (camera, camera_transform, _) = camera.into_iter().next().expect("No active camera found");
    let camera = (*camera).borrow();
    let camera_transform = (*camera_transform).borrow();

    let device = &renderer.context.device;

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader.wgsl").into()),
    });
    let camera_uniform = CameraUniform::from_inv_view_proj(
        &(&*camera_transform).into(),
        &camera.get_projection_matrix(),
    );
    let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Camera Buffer"),
        contents: bytemuck::cast_slice(&[camera_uniform]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    renderer
        .context
        .queue
        .write_buffer(&camera_buffer, 0, bytemuck::cast_slice(&[camera_uniform]));

    let camera_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("camera_bind_group_layout"),
        });
    let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &camera_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: camera_buffer.as_entire_binding(),
        }],
        label: Some("camera_bind_group"),
    });

    let model_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("model_bind_group_layout"),
        });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[&camera_bind_group_layout, &model_bind_group_layout],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[MeshVertex::desc()],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: renderer.context.config.format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            // Setting this to anything other than Fill requires Features::POLYGON_MODE_LINE
            // or Features::POLYGON_MODE_POINT
            polygon_mode: wgpu::PolygonMode::Fill,
            // Requires Features::DEPTH_CLIP_CONTROL
            unclipped_depth: false,
            // Requires Features::CONSERVATIVE_RASTERIZATION
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        // If the pipeline will be used with a multiview render pass, this
        // indicates how many array layers the attachments will have.
        multiview: None,
    });

    let mut encoder = renderer.create_command_encoder(None);
    let render_pass_builder = RenderPassBuilder::new();
    let surface_texture = renderer.context.surface.get_current_texture().unwrap();
    let view = Renderer::get_current_texture_view(&surface_texture);
    let depth_texture = texture::Texture::create_depth_texture(
        &renderer.context.device,
        &renderer.context.config,
        "depth_texture",
    );

    let mut meshes = Vec::new();
    let mut transforms = Vec::new();

    for (mesh, transform) in models {
        meshes.push(mesh);
        transforms.push(transform);
    }

    let vertex_buffers: Vec<_> = meshes
        .iter()
        .map(|mesh| {
            let mesh = (**mesh).borrow();
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&mesh.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            })
        })
        .collect();

    let uniforms: Vec<_> = transforms
        .iter()
        .map(|transform| {
            let transform = &*((**transform).borrow());
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Model Buffer"),
                contents: bytemuck::cast_slice(&[Mat4Uniform::from(transform)]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            })
        })
        .collect();

    let uniform_bind_groups: Vec<_> = uniforms
        .iter()
        .map(|uniform| {
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &model_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform.as_entire_binding(),
                }],
                label: Some("model_bind_group"),
            })
        })
        .collect();

    let index_buffers: Vec<_> = meshes
        .iter()
        .map(|mesh| {
            let mesh = (**mesh).borrow();
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&mesh.indices),
                usage: wgpu::BufferUsages::INDEX,
            })
        })
        .collect();

    let indices_counts: Vec<_> = meshes
        .iter()
        .map(|mesh| {
            let mesh = (**mesh).borrow();
            mesh.indices.len() as u32
        })
        .collect();

    {
        let mut render_pass = render_pass_builder
            .color_attachment(&view)
            .depth_stencil_attachment(&depth_texture.view)
            .begin_render_pass(&mut encoder, None);
        render_pass.set_pipeline(&render_pipeline);
        render_pass.set_bind_group(0, &camera_bind_group, &[]);
        for (((vertex_buffer, index_buffer), num_indices), uniform_bind_group) in vertex_buffers
            .iter()
            .zip(index_buffers.iter())
            .zip(indices_counts.iter())
            .zip(uniform_bind_groups.iter())
        {
            render_pass.set_bind_group(1, uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..*num_indices, 0, 0..1);
        }
    }
    renderer.context.queue.submit([encoder.finish()]);
    surface_texture.present();
}
