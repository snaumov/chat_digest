use dgraph::{Dgraph, DgraphError};
use serde::{Serialize, Deserialize};
use super::db::DbError;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub struct Digest {
    uid: String,
    message_uid: String,
    text: String,
    pub date: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DigestInput {
    pub message_uid: String,
    pub text: String,
    pub date: i64,
}

pub struct DigestDb {
    db: Arc<Dgraph>
}

#[derive(Serialize, Deserialize, Debug)]
struct AllDigests {
    digests: Vec<Digest>,
}

impl<'a> DigestDb {

    pub fn new(db: Arc<Dgraph>) -> DigestDb {
        DigestDb {
            db,
        }
    }

    pub fn new_digest(&self, digest: DigestInput) -> Result<(), DbError> {
        let dgraph = &self.db;
        let mut txn = dgraph.new_txn();

        let mut mutation = dgraph::Mutation::new();

        mutation.set_set_json(serde_json::to_vec(&digest).expect("invalid json"));

        txn.mutate(mutation)?;

        txn.commit()?;

        Ok(())
    }

    pub fn get_most_recent_digest(&self) -> Result<Digest, DbError> {

        let q = r#"query digests() {
            digests(func: has(text), orderasc: date, first: 1) {
              uid
              message_uid
              text
              date
            }
          }"#.to_string();

        let resp = &self.db.new_readonly_txn().query(q)?;

        let digests: AllDigests = serde_json::from_slice(&resp.json)?;

        if digests.digests.len() < 1 {

            return Err(DbError::Custom("No messages found".to_string()));

        }

        return Ok(digests.digests.into_iter().nth(0).unwrap());

    }

}