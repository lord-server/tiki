#![allow(clippy::new_without_default)]
#![allow(dead_code)]

use anyhow::Result;
use std::io::ErrorKind;
use std::net::{ToSocketAddrs, UdpSocket};
use std::time::Duration;

use tiki_proto::{ConnectionState, Input, Output};

const MAX_FRAME_SIZE: usize = 1536;

pub struct Connection {
    socket: UdpSocket,
    state: ConnectionState,
}

impl Connection {
    pub fn new(address: impl ToSocketAddrs) -> Self {
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

        socket.connect(address).unwrap();

        let state = ConnectionState::new();

        socket.set_read_timeout(Some(Duration::from_millis(200))).unwrap();

        Self { socket, state }
    }

    pub fn poll(&mut self) -> Result<()> {
        let mut buf = [0; MAX_FRAME_SIZE];

        let output = self.state.poll_output();
        match output {
            Output::SendData(data) => {
                self.socket.send(&data).unwrap();
            }
            Output::Wait => {
                std::thread::sleep(Duration::from_millis(200));
            }
        }

        match self.socket.recv(&mut buf) {
            Ok(packet_len) => {
                self.state.submit_input(Input::ReceivedData(&buf[..packet_len]));
            }
            Err(e) => match e.kind() {
                ErrorKind::TimedOut | ErrorKind::WouldBlock => {
                    self.state.submit_input(Input::TimedOut);
                },
                _ => Err(e)?,
            }
        }

        Ok(())
    }
}

fn main() {
    let address = std::env::args().nth(1).unwrap();

    let mut connection = Connection::new(address);

    loop {
        connection.poll().unwrap();
        std::thread::sleep(Duration::from_secs(1));
    }
}
