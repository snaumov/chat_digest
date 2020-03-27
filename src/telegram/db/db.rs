use dgraph::{make_dgraph, DgraphClient, Dgraph, DgraphError};
use serde_json::Error as SerdeJSONError;
use super::message::MessageDb;
use super::digest::DigestDb;
use std::error;
use std::fmt;
use std::convert;
use std::sync::Arc;
// use dgraph::client::Dgraph;

pub struct Db {
  pub message: MessageDb,
  pub digest: DigestDb,
}

#[derive(Debug)]
pub enum DbError {
  Custom(String),
  DgraphError(DgraphError),
  SerdeJSONError(SerdeJSONError),
}

impl fmt::Display for DbError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      DbError::Custom(ref custom_error) => write!(f, "[DB]: {}", custom_error),
      DbError::DgraphError(ref dgraph_error) => write!(f, "[DB]: {}", dgraph_error),
      DbError::SerdeJSONError(ref serde_error) => write!(f, "[DB]: {}", serde_error),
    }
  }
}

impl convert::From<DgraphError> for DbError {
  fn from (error: DgraphError) -> Self {
      DbError::DgraphError(error)
  }
}

impl convert::From<SerdeJSONError> for DbError {
  fn from (error: SerdeJSONError) -> Self {
      DbError::SerdeJSONError(error)
  }
}

impl error::Error for DbError {

  // fn cause(&self) -> Option<&dyn error::Error> {
  //   match self {
  //       DbError::Custom(custom_error) => Some(custom_error),
  //       _ => None,
  //   }
  // }

  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    None
  }
}

impl <'a> Db {
  pub fn new(client: Arc<dgraph::Dgraph>) -> Db {
    let message = MessageDb::new(client.clone());
    let digest = DigestDb::new(client.clone());

    let op = dgraph::Operation{
      schema: message.schema.clone(), ..Default::default()
    };
    
    client.alter(&op);
    
    Db {
      message,
      digest,
    }
  }
}

// {
//     find_follower(func: uid(0x2f)) @recurse(depth:10) {
//       username
//       message
//       ~replyto
//     }
//   }