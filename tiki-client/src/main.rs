#![allow(clippy::new_without_default)]
#![allow(clippy::single_match)]
#![allow(dead_code)]

pub mod connection;

use tiki_input::InputHandler;
use tiki_render::Renderer;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::keyboard::KeyCode;
use winit::window::{Window, WindowId};

struct App {
    window: Option<Window>,
    renderer: Option<Renderer>,
    input_handler: InputHandler,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            renderer: None,
            input_handler: InputHandler::new(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes().with_title("Tiki");

        let window = event_loop.create_window(window_attributes).unwrap();
        let window_size = window.inner_size();

        self.renderer = Some(Renderer::new(
            &window,
            window_size.width,
            window_size.height,
        ));
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        self.input_handler.submit_winit_event(&event);

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(size.width, size.height);
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.input_handler.is_key_pressed(KeyCode::Escape) {
            event_loop.exit();
        }

        if let Some(renderer) = &mut self.renderer {
            renderer.render(&mut []);
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let mut app = App::new();

    event_loop.run_app(&mut app).unwrap();

    // let args: Vec<_> = std::env::args().collect();
    // if args.len() < 4 {
    //     eprintln!("usage: tiki <address:port> <player_name> <password>");
    //     std::process::exit(1);
    // }

    // let address = args[1].clone();
    // let name = args[2].clone();
    // let password = args[3].clone();

    // let mut connection = Connection::new(address, Credentials { name, password });

    // loop {
    //     connection.poll().unwrap();
    // }
}
