use std::sync::Arc;

use tokio::runtime::Runtime;
use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::ecs::query::Query;
use crate::ecs::resource::ResMut;
use crate::ecs::World;
use crate::graphics::camera::{CameraTransform, CameraUniform};
use crate::graphics::mesh::MeshVertex;
use crate::graphics::transform::Mat4Uniform;
use crate::graphics::{Mesh, PerspectiveCamera, Transform};
use crate::render::render_pass::RenderPassBuilder;
use crate::render::texture;
use crate::render::vertex::Vertex;
use crate::render::Renderer;

use super::ActiveCamera;

pub fn init_renderer_system(world: &mut World) {
    let window = world.get_resource::<Arc<Window>>().unwrap().clone();
    let rt = Runtime::new().unwrap();
    let renderer = rt.block_on(async { Renderer::new(window).await });
    world.add_resource(renderer);
}

pub fn update_render_system(
    renderer: ResMut<Renderer>,
    models: Query<(&Mesh, &Transform)>,
    camera: Query<(&PerspectiveCamera, &CameraTransform, &ActiveCamera)>,
) {
    let (camera, camera_transform, _) = camera.into_iter().next().expect("No active camera found");

    let device = &renderer.context.device;

    let camera_uniform = CameraUniform::from_inv_view_proj(
        &Transform::from(camera_transform).into(),
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

    let scene_render_pipeline = renderer.pipelines.get("scene").unwrap();
    let crosshair_render_pipeline = renderer.pipelines.get("crosshair").unwrap();

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

    for (mesh, transform) in &models {
        meshes.push(mesh);
        transforms.push(transform);
    }

    let vertex_buffers = meshes
        .iter()
        .map(|mesh| {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&mesh.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            })
        })
        .collect::<Vec<_>>();

    let uniforms = transforms
        .iter()
        .map(|transform| {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Model Buffer"),
                contents: bytemuck::cast_slice(&[Mat4Uniform::from(*transform)]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            })
        })
        .collect::<Vec<_>>();

    let uniform_bind_groups = uniforms
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
        .collect::<Vec<_>>();

    let settings = CrosshairSettings {
        color: [0.0, 1.0, 1.0, 1.0],
        length: 4,
        thickness: 2,
        gap: 0,
        padding: 0,
    };

    let settings_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Crosshair Settings Buffer"),
        contents: bytemuck::cast_slice(&[settings]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    // Create a bind group layout for the settings
    let settings_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(
                        std::mem::size_of::<CrosshairSettings>() as wgpu::BufferAddress,
                    ),
                },
                count: None,
            }],
            label: Some("settings_bind_group_layout"),
        });

    let settings_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &settings_bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: settings_buffer.as_entire_binding(),
        }],
        label: Some("Crosshair Settings Bind Group"),
    });

    let index_buffers = meshes
        .iter()
        .map(|mesh| {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&mesh.indices),
                usage: wgpu::BufferUsages::INDEX,
            })
        })
        .collect::<Vec<_>>();

    let indices_counts = meshes
        .iter()
        .map(|mesh| mesh.indices.len() as u32)
        .collect::<Vec<_>>();

    {
        let mut render_pass = render_pass_builder
            .color_attachment(&view)
            .depth_stencil_attachment(&depth_texture.view)
            .begin_render_pass(&mut encoder, None);

        render_pass.set_pipeline(scene_render_pipeline);
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

        render_pass.set_pipeline(crosshair_render_pipeline);
        render_pass.set_bind_group(0, &settings_bind_group, &[]);
        render_pass.draw(0..3, 0..1);
    }

    renderer.context.queue.submit([encoder.finish()]);
    surface_texture.present();
}

pub fn init_pipeline_system(world: &mut World) {
    let mut renderer = world.get_resource_mut::<Renderer>().unwrap();
    let device = &renderer.context.device;

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/scene.wgsl").into()),
    });

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
                blend: Some(wgpu::BlendState::REPLACE),
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

    renderer
        .pipelines
        .insert("scene".to_string(), render_pipeline);
}

pub fn init_crosshair_pipeline_system(world: &mut World) {
    let mut renderer = world.get_resource_mut::<Renderer>().unwrap();
    let device = &renderer.context.device;

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Crosshair Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/crosshair.wgsl").into()),
    });

    let settings_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: wgpu::BufferSize::new(
                        std::mem::size_of::<CrosshairSettings>() as wgpu::BufferAddress,
                    ),
                },
                count: None,
            }],
            label: Some("settings_bind_group_layout"),
        });

    let crosshair_pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Crosshair Pipeline Layout"),
            bind_group_layouts: &[&settings_bind_group_layout],
            ..Default::default()
        });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Crosshair Pipeline"),
        layout: Some(&crosshair_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format: renderer.context.config.format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
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
        multiview: None,
    });

    renderer
        .pipelines
        .insert("crosshair".to_string(), render_pipeline);
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CrosshairSettings {
    pub color: [f32; 4],
    pub length: u32,
    pub thickness: u32,
    pub gap: u32,
    pub padding: u32,
}
