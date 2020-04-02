use dgraph::{Dgraph, DgraphError};
use serde::{Serialize, Deserialize};
use super::db::DbError;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Digest {
    uid: String,
    pub message_uid: String,
    pub chat_id: i64,
    pub text: String,
    pub date: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DigestInput {
    pub message_uid: String,
    pub text: String,
    pub date: i64,
    pub chat_id: i64,
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
            digests(func: has(message_uid), orderdesc: date, first: 1) {
              uid
              message_uid
              text
              date
              chat_id
            }
          }"#.to_string();

        let resp = &self.db.new_readonly_txn().query(q)?;

        let digests: AllDigests = serde_json::from_slice(&resp.json)?;

        if digests.digests.len() < 1 {
            // TODO INFO error
            return Err(DbError::Custom("No digests found".to_string()));

        }

        return Ok(digests.digests.into_iter().nth(0).unwrap());

    }

    pub fn get_digests_past_timestamp(&self, timestamp: i64) -> Result<Vec<Digest>, DbError> {

        let q = r#"query digests($date: int) {
            digests(func: ge(date, $date)) @filter(has(message_uid)) {
              uid
              message_uid
              text
              date
              chat_id
            }
          }"#.to_string();

        let mut vars = HashMap::new();
        vars.insert("$date".to_string(), timestamp.to_string());

        let resp = &self.db.new_readonly_txn().query_with_vars(q, vars)?;

        let digests: AllDigests = serde_json::from_slice(&resp.json)?;

        if digests.digests.len() < 1 {
            // TODO INFO error
            return Err(DbError::Custom("No digests found".to_string()));

        }

        return Ok(digests.digests);

    }

}