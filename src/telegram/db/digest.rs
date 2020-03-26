use dgraph::{Dgraph, DgraphError};
use serde::{Serialize, Deserialize};
use super::db::DbError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Digest {
    uid: String,
    message_uid: String,
    text: String,
    date: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DigestInput {
    pub message_uid: String,
    pub text: String,
    pub date: i64,
}

pub struct DigestDb<'a> {
    db: &'a Dgraph
}

impl<'a> DigestDb<'a> {

    pub fn new(db: &'a Dgraph) -> DigestDb<'a> {
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

}