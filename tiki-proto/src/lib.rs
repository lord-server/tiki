#![allow(clippy::new_without_default)]

use std::collections::VecDeque;
use std::io::{Cursor, Read};

use crate::serialize::Serialize;
use crate::serverbound::{Hello, Serverbound};
use crate::transport::{ControlHeader, Frame, FrameType, Reliability, TransportError};

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
    Disconnect,
    Wait,
}

/// States for connection state machine.
#[derive(Debug, PartialEq, Eq)]
enum Phase {
    BeforeHello,
    Hello,
    ReceivingMedia,
    InGame,
    Disconnected,
}

pub struct ConnectionState {
    phase: Phase,

    peer_id: u16,

    send_queue: VecDeque<Vec<u8>>,
    recv_queue: VecDeque<Vec<u8>>,

    disconnected: bool,
}

impl ConnectionState {
    pub fn new() -> Self {
        Self {
            phase: Phase::BeforeHello,

            peer_id: 0,

            send_queue: VecDeque::new(),
            recv_queue: VecDeque::new(),

            disconnected: false,
        }
    }

    pub fn submit_input(&mut self, input: Input) -> Result<(), crate::Error> {
        println!("input: {:?}", input);

        match input {
            Input::ReceivedData(data) => {
                self.handle_frame(&mut Cursor::new(data))?;
            }
            Input::TimedOut => {}
        }

        Ok(())
    }

    pub fn poll_output(&mut self) -> Output {
        if self.phase == Phase::BeforeHello {
            self.send_original(Serverbound::Hello(Hello {}));
        }

        if self.disconnected {
            return Output::Disconnect;
        }

        if let Some(frame) = self.send_queue.pop_front() {
            return Output::SendData(frame);
        }

        Output::Wait
    }

    fn handle_frame(&mut self, r: &mut impl Read) -> Result<(), crate::Error> {
        let frame = Frame::deserialize(r)?;

        if let Reliability::Reliable { seqnum } = frame.reliability {
            self.send_ack(frame.channel, seqnum);
        }

        println!("{:?}", frame);

        match frame.ty {
            FrameType::Control(control) => match control {
                ControlHeader::Ack { .. } => {}
                ControlHeader::SetPeerId { peer_id } => {
                    if self.phase == Phase::BeforeHello {
                        self.peer_id = peer_id;
                        self.phase = Phase::Hello;
                    }
                }
                ControlHeader::Ping => {
                    // pong
                }
                ControlHeader::Disco => {}
            },
            FrameType::Original => todo!(),
            FrameType::Split(_) => todo!(),
        }

        Ok(())
    }

    fn send_ack(&mut self, channel: u8, seqnum: u16) {
        let mut data = Vec::new();

        let frame = Frame {
            peer_id: self.peer_id,
            channel,
            reliability: Reliability::Unreliable,
            ty: FrameType::Control(ControlHeader::Ack { seqnum }),
        };

        frame.serialize(&mut data);

        self.send_raw(data);
    }

    fn send_original(&mut self, payload: impl Serialize) {
        let mut buf = Vec::new();

        let client_hello_packet = Frame {
            peer_id: self.peer_id,
            channel: 0,
            reliability: Reliability::Unreliable,
            ty: FrameType::Original,
        };

        client_hello_packet.serialize(&mut buf);
        payload.serialize(&mut buf);

        self.send_raw(buf);
    }

    fn send_raw(&mut self, data: Vec<u8>) {
        self.send_queue.push_back(data);
    }

    pub fn send_packet(&mut self, _packet: impl Serialize) {
        unimplemented!()
    }

    pub fn recv_packets<S: Serialize>(&mut self) -> Result<S, crate::Error> {
        unimplemented!()
    }
}
