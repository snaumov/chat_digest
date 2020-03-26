use super::db::Db;
use dgraph::{Dgraph, Mutation};
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct User {
    /// Unique identifier for this user or bot.
    pub uid: String,
    /// User‘s or bot’s first name.
    pub first_name: String,
    /// User‘s or bot’s last name.
    pub last_name: Option<String>,
    /// User‘s or bot’s username.
    pub username: Option<String>,
    /// True, if this user is a bot.
    pub is_bot: bool,
}

pub struct UserDb {
    db: Dgraph,
}

impl UserDb {
    pub fn new(db: Dgraph) -> UserDb {
        UserDb {
            db,
        }
    }

    pub fn new_user(&self, user: User) {
        let dgraph = &self.db;
        let mut txn = dgraph.new_txn();

        let mut mutation = dgraph::Mutation::new();
        mutation.set_set_json(serde_json::to_vec(&user).expect("invalid json"));

        let assigned = txn.mutate(mutation).expect("failed to create data");

        let res = txn.commit();
    }
}