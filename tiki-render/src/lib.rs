pub mod mesh;

use bytemuck::Zeroable;
use glam::{vec2, vec3, Vec3};
use pollster::FutureExt;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use wgpu::include_wgsl;
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::mesh::{Mesh, Vertex};

pub struct Renderer {
    _instance: wgpu::Instance,
    surface: wgpu::Surface<'static>,
    _adapter: wgpu::Adapter,
    surface_config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,

    triangle: wgpu::Buffer,

    pipeline_layout: wgpu::PipelineLayout,
    pipeline: wgpu::RenderPipeline,
    uniform_buffer: wgpu::Buffer,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
}

#[derive(bytemuck::Zeroable, bytemuck::Pod, Clone, Copy)]
#[repr(C)]
struct Uniforms {
    position: Vec3,
}

impl Renderer {
    pub fn new<H: HasWindowHandle + HasDisplayHandle>(handle: H, width: u32, height: u32) -> Self {
        let instance = wgpu::Instance::new(Default::default());

        let surface = unsafe {
            instance
                .create_surface_unsafe(wgpu::SurfaceTargetUnsafe::RawHandle {
                    raw_display_handle: handle.display_handle().unwrap().as_raw(),
                    raw_window_handle: handle.window_handle().unwrap().as_raw(),
                })
                .unwrap()
        };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .block_on()
            .unwrap();

        let mut surface_config = surface.get_default_config(&adapter, width, height).unwrap();

        // NOTE: PresentMode::Fifo causes significant input lag, mailbox
        // (vsync off) is much better for editor. Ideally this should be
        // configurable by user.
        surface_config.present_mode = wgpu::PresentMode::AutoNoVsync;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::default(),
                    required_limits: wgpu::Limits::default(),
                    memory_hints: wgpu::MemoryHints::default(),
                },
                None,
            )
            .block_on()
            .unwrap();

        let mut mesh = Mesh::new();
        mesh.add_vertex(Vertex {
            position: vec3(-1.0, -1.0, 0.0),
            normal: vec3(0.0, 0.0, 0.0),
            texcoord: vec2(0.0, 0.0),
        });
        mesh.add_vertex(Vertex {
            position: vec3(-1.0, 3.0, 0.0),
            normal: vec3(1.0, 0.0, 0.0),
            texcoord: vec2(0.0, 1.0),
        });
        mesh.add_vertex(Vertex {
            position: vec3(3.0, -1.0, 0.0),
            normal: vec3(1.0, 0.0, 0.0),
            texcoord: vec2(1.0, 0.0),
        });

        let triangle = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(mesh.data()),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::bytes_of(&Uniforms::zeroed()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(uniform_buffer.as_entire_buffer_binding()),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let world_shader = include_wgsl!("../../data/fullscreen.wgsl");
        let vertex_shader = device.create_shader_module(world_shader.clone());
        let fragment_shader = device.create_shader_module(world_shader);

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: None,
            vertex: wgpu::VertexState {
                module: &vertex_shader,
                entry_point: "vs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                buffers: &[Vertex::buffer_layout()],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &fragment_shader,
                entry_point: "fs_main",
                compilation_options: wgpu::PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::all(),
                })],
            }),
            multiview: None,
            cache: None,
        });

        Self {
            _instance: instance,
            surface,
            _adapter: adapter,
            surface_config,
            device,
            queue,
            triangle,

            pipeline_layout,
            pipeline,
            uniform_buffer,
            bind_group_layout,
            bind_group,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface_config.width = width;
        self.surface_config.height = height;

        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn render(&mut self, techniques: &mut [&mut dyn Technique]) {
        let surface_texture = self.surface.get_current_texture().unwrap();
        let surface_texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        {
            let mut rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &surface_texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            }).forget_lifetime();

            rp.set_pipeline(&self.pipeline);
            rp.set_vertex_buffer(0, self.triangle.slice(..));

            rp.draw(0..3, 0..1);

            for technique in techniques {
                technique.render(self, &mut rp, &mut encoder);
            }
        }

        self.queue.submit([encoder.finish()]);
        surface_texture.present();
    }

    pub fn surface_format(&self) -> wgpu::TextureFormat {
        self.surface_config.format
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }
}

pub trait Technique {
    fn render(&mut self, r: &Renderer, rp: &mut wgpu::RenderPass<'static>, encoder: &mut wgpu::CommandEncoder);
}
