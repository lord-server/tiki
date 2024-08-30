use anyhow::{bail, Result};
use std::io::ErrorKind;
use std::net::{ToSocketAddrs, UdpSocket};
use std::time::Duration;

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
