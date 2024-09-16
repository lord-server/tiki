use std::collections::VecDeque;
use std::io::{Cursor, Read};

use crate::clientbound::Clientbound;
use crate::serialize::Serialize;
use crate::serverbound::{Hello, Init, Init2, Serverbound};
use crate::transport::{ControlHeader, Frame, FrameType, Reliability, TransportError};

pub mod clientbound;
pub mod serialize;
pub mod serverbound;
pub mod transport;
pub mod common;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("i/o error")]
    Io(#[from] std::io::Error),

    #[error("transport protocol error: {0}")]
    Transport(#[from] TransportError),

    #[error("peer sent unknown packet: {0}")]
    UnknownPacket(u16),

    #[error("unexpected non-Unicode string: {0:?}")]
    NonUnicodeString(Vec<u8>),
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
    SendHello,
    AwaitHello,
    SendAuth1,
    RecvAuth1,
    SendAuth2,
    RecvAuth2,
    ReceivingMedia,
    InGame,
    Disconnected,
}

pub struct ClientConnectionState {
    phase: Phase,

    peer_id: u16,

    send_queue: VecDeque<Vec<u8>>,
    recv_packet_queue: VecDeque<Clientbound>,

    credentials: Credentials,
}

pub struct Credentials {
    pub name: String,
    pub password: String,
}

impl ClientConnectionState {
    pub fn new(credentials: Credentials) -> Self {
        Self {
            phase: Phase::SendHello,

            peer_id: 0,

            send_queue: VecDeque::new(),
            recv_packet_queue: VecDeque::new(),

            credentials,
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
        println!("Phase: {:?}", self.phase);

        match self.phase {
            Phase::SendHello => {
                self.send_original(Serverbound::Hello(Hello {}));
            }
            Phase::AwaitHello => self.send_original(Serverbound::Init(Init {
                client_max_serialization_ver: 255,
                supp_compr_modes: 0,
                min_net_proto_version: 0,
                max_net_proto_version: 100,
                player_name: self.credentials.name.clone(),
            })),
            Phase::Disconnected => {
                return Output::Disconnect;
            }
            _ => {}
        }

        if let Some(buf) = self.send_queue.pop_front() {
            return Output::SendData(buf);
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
                ControlHeader::Ack { .. } => {
                    // re-send un-acked frames
                }
                ControlHeader::SetPeerId { peer_id } => {
                    if self.phase == Phase::SendHello {
                        self.peer_id = peer_id;
                        self.phase = Phase::AwaitHello;
                    }
                }
                ControlHeader::Ping => {
                    // pong
                }
                ControlHeader::Disco => {
                    self.phase = Phase::Disconnected;
                }
            },
            FrameType::Original => {
                let clientbound = Clientbound::deserialize(r)?;
                match clientbound {
                    Clientbound::Hello(ref hello) => {
                        println!("Got Clientbound::Hello: {hello:?}");
                        self.phase = Phase::SendAuth1;
                    }
                    _ => {}
                }

                self.recv_packet_queue.push_back(clientbound);
            }
            FrameType::Split(_) => {}
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
