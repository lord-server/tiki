#![allow(clippy::new_without_default)]

use std::io::Cursor;

use crate::serialize::Serialize;
use crate::transport::{Frame, FrameType, Reliability, TransportError};

pub mod clientbound;
pub mod serialize;
pub mod serverbound;
pub mod transport;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("i/o error")]
    Io(#[from] std::io::Error),

    #[error("transport protocol error: {0}")]
    Transport(#[from] TransportError),

    #[error("peer sent unknown packet: {0}")]
    UnknownPacket(u16),
}

#[derive(Debug)]
pub enum Input<'a> {
    ReceivedData(&'a [u8]),
    TimedOut,
}

#[derive(Debug)]
pub enum Output {
    SendData(Vec<u8>),
    Wait,
}

/// States for connection state machine.
#[derive(Debug)]
enum Phase {
    Start,
    ReceivingMedia,
    InGame,
    Disconnected,
}

pub struct ConnectionState {
    phase: Phase,
}

impl ConnectionState {
    pub fn new() -> Self {
        Self {
            phase: Phase::Start,
        }
    }

    pub fn submit_input(&mut self, input: Input) {
        println!("input: {:?}", input);
    }

    pub fn poll_output(&mut self) -> Output {
        let mut buf = Cursor::new(Vec::new());

        let client_hello_packet = Frame {
            peer_id: 0,
            channel: 0,
            reliability: Reliability::Unreliable,
            ty: FrameType::Original,
        };

        client_hello_packet.serialize(&mut buf);
        0u8.serialize(&mut buf);

        Output::SendData(buf.into_inner())
    }

    pub fn send_packet(&mut self) {
        unimplemented!()
    }

    pub fn recv_packets(&mut self) {
        unimplemented!()
    }
}
