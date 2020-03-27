use chrono::NaiveTime;
use chrono::Utc;
use chrono::Duration;
use super::db::{Db, DbError, GroupedMessage, DigestInput};
// use super::db::m
use std::error;
use std::fmt;
use std::convert;
use std::sync::Arc;

pub struct Digest {
    db: Arc<Db>,
}

#[derive(Debug)]
pub enum DigestError {
  Custom(String),
  DbError(DbError),
}

impl fmt::Display for DigestError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
        DigestError::Custom(ref custom_error) => write!(f, "[Digest]: {}", custom_error),
        DigestError::DbError(ref db_error) => write!(f, "[DB]: {}", db_error),
    }
  }
}

impl convert::From<DbError> for DigestError {
    fn from (error: DbError) -> Self {
        DigestError::DbError(error)
    }
}

// impl convert::From<DgraphError> for DigestError {

// }

// impl From::from(DbError) for DigestError {

// }

impl error::Error for DigestError {

  fn source(&self) -> Option<&(dyn error::Error + 'static)> {
    None
  }
}

impl<'a> Digest {
    pub fn new(db: Arc<Db>) -> Digest {
        Digest {
            db,
        }
    }

    pub fn build_digest(&self) -> Result<(), DigestError> {

        let start_timestamp = self.pick_start_timestamp();

        let all_primary_messages_past_date = self.db.message.get_primary_messages_past_date(start_timestamp)?;

        for primary_message in all_primary_messages_past_date.iter() {
            let grouped_message = self.db.message.get_message_group_by_id(primary_message.uid.clone())?;

            let summary = self.summary_message_group(grouped_message);
            
            self.db.digest.new_digest(
                DigestInput{
                    message_uid: primary_message.uid.clone(),
                    date: primary_message.date,
                    text: summary,
                }
            )?

        }

        Ok(())
    
    }

}

impl<'a> Digest {
    fn summary_message_group(&self, grouped_message: GroupedMessage) -> String {
        let mut sum_string = "".to_string();

        sum_string += &grouped_message.text;

        if grouped_message.reply_to != None {
            for nested_group_message in grouped_message.reply_to.unwrap().into_iter() {
                sum_string += &self.summary_message_group(*nested_group_message)
            }
        }

        return sum_string;
    }

    fn pick_start_timestamp(&self) -> i64 {
        // TODO more than 10 days?
        let ten_days_ago_timestamp = (Utc::now() - Duration::days(10)).timestamp();

        let recent_digest = self.db.digest.get_most_recent_digest();

        match recent_digest {
            Ok(digest) => {
                return if digest.date < ten_days_ago_timestamp { digest.date } else { ten_days_ago_timestamp };
            },
            Err(_) => {
                return ten_days_ago_timestamp;
            } 
        }
    }
}

// Get message group
// {
//     messages(func: uid(0x6)) @recurse(depth: 10)  {
//       uid
//       text
//       date
//       ~reply_to
//     }
//   }