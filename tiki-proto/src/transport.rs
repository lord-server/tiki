use std::io::{Read, Write};

use tiki_macros::Serialize;

use crate::serialize::Serialize;
use crate::Error;

const PROTOCOL_ID: u32 = 0x4F457403;

#[derive(thiserror::Error, Debug)]
pub enum TransportError {
    #[error("unknown protocol ID: {0:08X}")]
    UnknownProtocolId(u32),

    #[error("unknown frame type: {0}")]
    UnknownFrameType(u8),

    #[error("unknown control frame type: {0}")]
    UnknownControlType(u8),
}

#[derive(Debug)]
pub struct Frame {
    pub peer_id: u16,
    pub channel: u8,
    pub reliability: Reliability,
    pub ty: FrameType,
}

impl Serialize for Frame {
    fn serialize<W: Write>(&self, w: &mut W) {
        PROTOCOL_ID.serialize(w);

        self.peer_id.serialize(w);
        self.channel.serialize(w);

        if let Reliability::Reliable { seqnum } = self.reliability {
            3u8.serialize(w);
            seqnum.serialize(w);
        }

        match self.ty {
            FrameType::Control(ref control) => {
                0u8.serialize(w);
                control.serialize(w);
            }
            FrameType::Original => 1u8.serialize(w),
            FrameType::Split(ref split) => {
                2u8.serialize(w);
                split.serialize(w);
            }
        }
    }

    fn deserialize<R: Read>(r: &mut R) -> Result<Self, Error> {
        let protocol_id = u32::deserialize(r)?;

        if protocol_id != PROTOCOL_ID {
            Err(TransportError::UnknownProtocolId(protocol_id))?
        }

        let peer_id = u16::deserialize(r)?;
        let channel = u8::deserialize(r)?;
        let mut ty = u8::deserialize(r)?;

        let reliability = if ty == 3 {
            let seqnum = u16::deserialize(r)?;
            ty = u8::deserialize(r)?;
            Reliability::Reliable { seqnum }
        } else {
            Reliability::Unreliable
        };

        let ty = match ty {
            0 => FrameType::Control(ControlHeader::deserialize(r)?),
            1 => FrameType::Original,
            2 => FrameType::Split(SplitHeader::deserialize(r)?),
            _ => Err(TransportError::UnknownFrameType(ty))?,
        };

        Ok(Self {
            peer_id,
            channel,
            reliability,
            ty,
        })
    }
}

#[derive(Debug)]
pub enum Reliability {
    Reliable { seqnum: u16 },
    Unreliable,
}

#[derive(Debug)]
pub enum FrameType {
    Control(ControlHeader),
    Original,
    Split(SplitHeader),
}

#[derive(Debug)]
pub enum ControlHeader {
    Ack { seqnum: u16 },
    SetPeerId { peer_id: u16 },
    Ping,
    Disco,
}

impl Serialize for ControlHeader {
    fn serialize<W: Write>(&self, w: &mut W) {
        match self {
            ControlHeader::Ack { seqnum } => {
                0u8.serialize(w);
                seqnum.serialize(w);
            }
            ControlHeader::SetPeerId { peer_id } => {
                1u8.serialize(w);
                peer_id.serialize(w);
            }
            ControlHeader::Ping => 2u8.serialize(w),
            ControlHeader::Disco => 3u8.serialize(w),
        }
    }

    fn deserialize<R: Read>(r: &mut R) -> Result<Self, Error> {
        let ty = u8::deserialize(r)?;

        Ok(match ty {
            0 => ControlHeader::Ack {
                seqnum: u16::deserialize(r)?,
            },
            1 => ControlHeader::SetPeerId {
                peer_id: u16::deserialize(r)?,
            },
            2 => ControlHeader::Ping,
            3 => ControlHeader::Disco,
            _ => Err(TransportError::UnknownControlType(ty))?,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct SplitHeader {
    pub seqnum: u16,
    pub chunk_count: u16,
    pub chunk_number: u16,
}
