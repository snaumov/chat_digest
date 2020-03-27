use super::db::{Db, DbError};
use super::user;
use dgraph::{Dgraph, Mutation, DgraphError};
use serde::{Serialize, Deserialize};
use serde_json;
use std::rc::{Rc};
use std::sync::Arc;
use std::collections::HashMap;
use std::boxed::Box;
use std::error;

#[derive(Serialize, Deserialize, Debug)]
pub struct ReplyTo {
    pub uid: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub uid: String,
    pub message_id: String,
    pub text: String,
    pub date: i64,
    // pub from: user::User,
    pub reply_to: Option<ReplyTo>,
    // pub reply_to: Option<Box<Message>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AllMessages {
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GroupedMessage {
    pub uid: String,
    pub message_id: String,
    pub text: String,
    pub date: i64,
    pub reply_to: Option<Vec<Box<GroupedMessage>>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AllGroupedMessages {
    messages: Vec<GroupedMessage>,
}

pub struct MessageDb {
    pub schema: String,
    db: Arc<Dgraph>,
}


#[derive(Serialize, Deserialize, Debug)]
struct QueryResponse {
    data: AllMessages,
}

impl MessageDb {
    pub fn new(client: Arc<dgraph::Dgraph>) -> MessageDb {
        
        MessageDb {
            db: client,
            schema: "message_id: string @index(exact) .".to_string() + 
                    &"date: int @index(int) .".to_string(),
        }
    }

    // pub fn boot(&mut self, db: &'a Dgraph) {
    //     self.db = Some(db);
    // }

    pub fn new_message(&self, message: Message) -> Result<(), DgraphError> {
        let dgraph = &self.db;
        let mut txn = dgraph.new_txn();

        let mut mutation = dgraph::Mutation::new();

        // println!("{:?}", serde_json::to_vec(&message));
        // println!("{}", message.reply_to.unwrap().uid.clone());
        mutation.set_set_json(serde_json::to_vec(&message).expect("invalid json"));

        txn.mutate(mutation)?;

        let res = txn.commit();

        match res {
            Ok(_) => {
                return Ok(());
            },
            Err(err) => {
                return Err(err);
            },
        }
    }

    pub fn get_primary_messages_past_date(&self, date: i64) -> Result<Vec<Message>, DbError> {
        let q = r#"query all($date: number) {
            messages(func: ge(date, $date)) @filter(NOT has(reply_to)) {
              uid
              text
              date
            }
          }"#.to_string();

        let mut vars = HashMap::new();
        vars.insert("$date".to_string(), date.to_string());

        let resp = &self.db.new_readonly_txn().query_with_vars(q, vars).expect("query");

        let messages: AllMessages = serde_json::from_slice(&resp.json).expect("parsing");

        if messages.messages.len() < 1 {

            return Err(DbError::Custom("No messages found".to_string()));

        }

        return Ok(messages.messages);
    }

    pub fn get_message_group_by_id(&self, uid: String) -> Result<GroupedMessage, DbError> {

        let q = r#"query all($uid: uid) {
            messages(func: uid($uid)) @recurse(depth: 100)  {
              uid
              text
              date
              reply_to: ~reply_to
            }
        }"#.to_string();

        let mut vars = HashMap::new();
        vars.insert("$uid".to_string(), uid);

        let resp = &self.db.new_readonly_txn().query_with_vars(q, vars).expect("query");

        let messages: AllGroupedMessages = serde_json::from_slice(&resp.json).expect("parsing");

        if messages.messages.len() < 1 {

            return Err(DbError::Custom("No messages found".to_string()));

        }

        return Ok(messages.messages.into_iter().nth(0).unwrap());
    }

    pub fn get_message_by_message_id(&self, message_id: String) -> Result<Message, DbError> {
        let q = r#"query all($a: string) {
            messages(func: eq(message_id, $a)) {
              uid
              message_id
              text
              date
              from
              reply_to
            }
          }"#.to_string();

        let mut vars = HashMap::new();
        vars.insert("$a".to_string(), message_id);

        let resp = &self.db.new_readonly_txn().query_with_vars(q, vars).expect("query");

        let messages: AllMessages = serde_json::from_slice(&resp.json).expect("parsing");

        if messages.messages.len() < 1 {

            return Err(DbError::Custom("No messages found".to_string()));

        }

        return Ok(messages.messages.into_iter().nth(0).unwrap());
    }
}