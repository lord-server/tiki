use egui::ClippedPrimitive;
use tiki_render::Renderer;
use tiki_world::{pos, World};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

struct App {
    app_window: Option<AppWindow>,
    ectx: egui::Context,
}

struct AppWindow {
    window: Window,
    renderer: Renderer,
    ewinit: egui_winit::State,
    ewgpu: egui_wgpu::Renderer,
}

impl App {
    pub fn new() -> Self {
        let ectx = egui::Context::default();

        ectx.begin_pass(egui::RawInput::default());

        Self {
            ectx,
            app_window: None,
        }
    }

    fn draw(&mut self) {
        let Some(app_window) = &mut self.app_window else {
            return;
        };

        let full_output = self.ectx.end_pass();

        self.ectx
            .begin_pass(app_window.ewinit.take_egui_input(&app_window.window));

        app_window
            .ewinit
            .handle_platform_output(&app_window.window, full_output.platform_output);

        let scale_factor = app_window.window.current_monitor().unwrap().scale_factor();

        let clipped_primitives = self
            .ectx
            .tessellate(full_output.shapes, scale_factor as f32);
        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: app_window.window.inner_size().into(),
            pixels_per_point: scale_factor as f32,
        };

        egui::Window::new("Hello").show(&self.ectx, |ui| {
            ui.label("Hello, world!");
        });

        for (id, image_delta) in full_output.textures_delta.set {
            app_window.ewgpu.update_texture(
                app_window.renderer.device(),
                app_window.renderer.queue(),
                id,
                &image_delta,
            );
        }

        app_window.renderer.render(&mut [&mut EguiWgpuTechnique {
            ewgpu: &mut app_window.ewgpu,
            paint_jobs: &clipped_primitives,
            screen_descriptor,
        }]);

        for id in full_output.textures_delta.free {
            app_window.ewgpu.free_texture(&id);
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title("Tiki Editor");

        let window = event_loop.create_window(window_attributes).unwrap();
        let window_size = window.inner_size();

        let ewinit = egui_winit::State::new(
            self.ectx.clone(),
            self.ectx.viewport_id(),
            &window,
            None,
            None,
            None,
        );

        let renderer = Renderer::new(&window, window_size.width, window_size.height);

        let ewgpu =
            egui_wgpu::Renderer::new(renderer.device(), renderer.surface_format(), None, 1, false);

        self.app_window = Some(AppWindow {
            window,
            renderer,
            ewinit,
            ewgpu,
        });
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(app_window) = &mut self.app_window {
            let output = app_window
                .ewinit
                .on_window_event(&app_window.window, &event);

            if output.repaint {
                app_window.window.request_redraw();
            }
        }

        match event {
            WindowEvent::RedrawRequested => self.draw(),
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(app_window) = &mut self.app_window {
                    app_window.renderer.resize(size.width, size.height);
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        let Some(app_window) = &mut self.app_window else {
            return;
        };

        if self.ectx.has_requested_repaint() {
            app_window.window.request_redraw();
        }
    }
}

struct EguiWgpuTechnique<'a> {
    ewgpu: &'a mut egui_wgpu::Renderer,
    paint_jobs: &'a [ClippedPrimitive],
    screen_descriptor: egui_wgpu::ScreenDescriptor,
}

impl<'a> tiki_render::Technique for EguiWgpuTechnique<'a> {
    fn render(
        &mut self,
        r: &Renderer,
        rp: &mut egui_wgpu::wgpu::RenderPass<'static>,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        self.ewgpu.update_buffers(
            r.device(),
            r.queue(),
            encoder,
            self.paint_jobs,
            &self.screen_descriptor,
        );

        self.ewgpu
            .render(rp, self.paint_jobs, &self.screen_descriptor);
    }
}

fn main() {
    let path = std::env::args().nth(1).unwrap();

    let mut world = World::open(path).unwrap();
    world.get_block(pos(0, 0, 0)).unwrap();

    let event_loop = EventLoop::new().unwrap();

    let mut app = App::new();

    event_loop.run_app(&mut app).unwrap();
}
