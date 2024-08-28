#![allow(clippy::new_without_default)]
#![allow(clippy::single_match)]
#![allow(dead_code)]

use anyhow::{bail, Result};
use std::io::ErrorKind;
use std::net::{ToSocketAddrs, UdpSocket};
use std::process::exit;
use std::time::Duration;
use tiki_render::Renderer;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

use tiki_proto::{ClientConnectionState, Credentials, Input, Output};

const MAX_FRAME_SIZE: usize = 1536;

pub struct Connection {
    socket: UdpSocket,
    state: ClientConnectionState,
}

impl Connection {
    pub fn new(address: impl ToSocketAddrs, credentials: Credentials) -> Self {
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

        socket.connect(address).unwrap();

        let state = ClientConnectionState::new(credentials);

        socket
            .set_read_timeout(Some(Duration::from_millis(200)))
            .unwrap();

        Self { socket, state }
    }

    pub fn poll(&mut self) -> Result<()> {
        let mut buf = [0; MAX_FRAME_SIZE];

        let output = self.state.poll_output();

        println!("{:?}", output);

        match output {
            Output::SendData(data) => {
                self.socket.send(&data).unwrap();
            }
            Output::Wait => {
                std::thread::sleep(Duration::from_millis(200));
            }
            Output::Disconnect => {
                bail!("disconnect");
            }
        }

        match self.socket.recv(&mut buf) {
            Ok(packet_len) => self
                .state
                .submit_input(Input::ReceivedData(&buf[..packet_len]))
                .unwrap(),
            Err(e) => match e.kind() {
                ErrorKind::TimedOut | ErrorKind::WouldBlock => {
                    self.state.submit_input(Input::TimedOut).unwrap();
                }
                _ => Err(e)?,
            },
        }

        Ok(())
    }
}

struct App {
    window: Option<Window>,
    renderer: Option<Renderer>,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            renderer: None,
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

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(renderer) = &mut self.renderer {
            renderer.render();
        }
    }
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 4 {
        eprintln!("usage: tiki <address:port> <player_name> <password>");
        std::process::exit(1);
    }

    let address = args[1].clone();
    let name = args[2].clone();
    let password = args[3].clone();

    let mut connection = Connection::new(address, Credentials { name, password });

    loop {
        connection.poll().unwrap();
    }
}
