use std::collections::HashMap;
use std::io::{Read, Write};
use std::path::Path;

use tiki_proto::serialize::Serialize;

use crate::postgres::PostgresBackend;

#[cfg(feature = "postgres")]
pub mod postgres;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to connect to database: {0}")]
    Connection(String),

    #[error("failed to execute query: {0}")]
    DatabaseQuery(String),

    #[error("serialization error: {0}")]
    Serialization(#[from] tiki_proto::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid meta file: {0}")]
    InvalidMeta(String),

    #[error("unsupported world backend: {0}")]
    UnsupportedBackend(String),
}

pub type NodeId = u16;

pub struct Node {
    pub id: NodeId,
    pub param1: u8,
    pub param2: u8,
}

pub struct Block {
    flags: u8,
    lighting_complete: u16,
    timestamp: u32,
    block_data: Vec<u8>,
    id_to_name: HashMap<NodeId, String>,
    name_to_id: HashMap<String, NodeId>,
}

impl Block {
    pub fn get_node(&self, pos: Pos) -> Node {
        assert!(pos.x >= 0 && pos.y >= 0 && pos.z >= 0);

        let index = 16 * 16 * pos.z as usize + 16 * pos.y as usize + pos.x as usize;

        let id_hi = self.block_data[2 * index] as u16;
        let id_lo = self.block_data[2 * index + 1] as u16;

        Node {
            id: (id_hi << 8) | id_lo,
            param1: self.block_data[2 * 16 * 16 * 16 + index],
            param2: self.block_data[3 * 16 * 16 * 16 + index],
        }
    }

    pub fn name(&self, id: u16) -> &str {
        self.id_to_name.get(&id).unwrap()
    }

    pub fn id(&self, name: &str) -> u16 {
        *self.name_to_id.get(name).unwrap()
    }
}

impl Serialize for Block {
    fn serialize<W: Write>(&self, w: &mut W) {
        todo!()
    }

    fn deserialize<R: Read>(r: &mut R) -> Result<Self, tiki_proto::Error> {
        let version = u8::deserialize(r)?;

        if version < 29 {
            return Self::deserialize_before_v29(r);
        }

        let r = &mut zstd::Decoder::new(r)?;

        let flags = u8::deserialize(r)?;
        let lighting_complete = u16::deserialize(r)?;

        let timestamp = u32::deserialize(r)?;
        let _mapping_version = u8::deserialize(r)?;

        let mut id_to_name = HashMap::new();
        let mut name_to_id = HashMap::new();

        let name_id_mapping_count = u16::deserialize(r)?;
        for _ in 0..name_id_mapping_count {
            let id = u16::deserialize(r)?;
            let name = String::deserialize(r)?;

            id_to_name.insert(id, name.clone());
            name_to_id.insert(name, id);
        }

        let _content_width = u8::deserialize(r);
        let _params_width = u8::deserialize(r);

        let mut block_data = vec![0; 4 * 16 * 16 * 16];
        r.read_exact(&mut block_data)?;

        Ok(Self {
            flags,
            lighting_complete,
            timestamp,
            block_data,
            id_to_name,
            name_to_id,
        })
    }
}

impl Block {
    fn deserialize_before_v29<R: Read>(r: &mut R) -> Result<Block, tiki_proto::Error> {
        unimplemented!()
    }
}

pub trait Backend {
    fn get_block_data(&mut self, pos: Pos) -> Result<Vec<u8>, Error>;
}

pub type Pos = glam::IVec3;

pub fn pos(x: i32, y: i32, z: i32) -> Pos {
    Pos::new(x, y, z)
}

pub struct World {
    meta: Meta,
    backend: Box<dyn Backend>,
}

impl World {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Error> {
        let meta = Meta::open(path.as_ref().join("world.mt"))?;

        let Some(backend_name) = meta.get_str("backend") else {
            return Err(Error::InvalidMeta("no backend specified".to_owned()));
        };

        let backend: Box<dyn Backend> = match backend_name {
            #[cfg(feature = "postgres")]
            "postgresql" => {
                let Some(params) = meta.get_str("pgsql_connection") else {
                    return Err(Error::InvalidMeta(
                        "no PostgreSQL connection specified".to_owned(),
                    ));
                };

                Box::new(PostgresBackend::new(params)?)
            }
            name => return Err(Error::UnsupportedBackend(name.to_owned())),
        };

        Ok(Self { meta, backend })
    }

    pub fn get_block(&mut self, pos: Pos) -> Result<Block, Error> {
        let data = self.backend.get_block_data(pos)?;

        Ok(Block::deserialize(&mut data.as_slice())?)
    }
}

pub struct Meta {
    data: HashMap<String, String>,
}

impl Meta {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, Error> {
        let file_data = std::fs::read_to_string(path)?;

        let mut data = HashMap::new();

        for line in file_data.lines() {
            let Some((key, value)) = line.split_once("=") else {
                continue;
            };

            let key = key.trim();
            let value = value.trim();

            data.insert(key.to_owned(), value.to_owned());
        }

        Ok(Self { data })
    }

    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.data.get(key).map(|value| value.as_str())
    }
}
