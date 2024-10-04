use postgres::NoTls;

use crate::{Backend, Error};

pub struct PostgresBackend {
    db: postgres::Client,
    get_block_data_stmt: postgres::Statement,
}

impl PostgresBackend {
    pub fn new(params: &str) -> Result<Self, Error> {
        let mut db = postgres::Client::connect(params, NoTls)
            .map_err(|e| Error::Connection(e.to_string()))?;

        let get_block_data_stmt = db
            .prepare("SELECT data FROM blocks WHERE posx = $1 AND posy = $2 AND posz = $3 LIMIT 1")
            .unwrap();

        Ok(Self {
            db,
            get_block_data_stmt,
        })
    }
}

impl Backend for PostgresBackend {
    fn get_block_data(&mut self, pos: crate::Pos) -> Result<Vec<u8>, Error> {
        let row = self
            .db
            .query_one(&self.get_block_data_stmt, &[&pos.x, &pos.y, &pos.z])
            .map_err(|e| Error::DatabaseQuery(e.to_string()))?;

        Ok(row.get(0))
    }
}
