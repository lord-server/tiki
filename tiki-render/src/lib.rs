pub mod svdag;

use pollster::FutureExt;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use tiki_world::{pos, Map, Pos};

pub struct Renderer {
    instance: wgpu::Instance,
    surface: wgpu::Surface<'static>,
    adapter: wgpu::Adapter,
    surface_config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
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

        let surface_config = surface.get_default_config(&adapter, width, height).unwrap();

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

        Self {
            instance,
            surface,
            adapter,
            surface_config,
            device,
            queue,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface_config.width = width;
        self.surface_config.height = height;

        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn render(&mut self) {
        let surface_texture = self.surface.get_current_texture().unwrap();
        let surface_texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        {
            let _rp = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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
            });
        }

        self.queue.submit([encoder.finish()]);
        surface_texture.present();
    }
}

