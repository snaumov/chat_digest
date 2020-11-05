use std::sync::Arc;
use std::convert::{TryInto, TryFrom};

use chrono::{ Utc, Duration };
use super::db::{Db, Message, User, ReplyTo, MessageInput, Digest};
use telegram_bot::{types as tb_types, Error, Api, SendMessage, GetMe};
use futures::StreamExt;

use dgraph::{DgraphError};

pub struct Bot {
    api: Api,
    db: Arc<Db>,
}

const DIGEST_COMMAND: &str = "/digest@summarizatorBot";
const T_ME_LINK: &str = "http://t.me/c/";

#[derive(PartialEq)]
enum BotCommands {
    Digest(String),
}

impl<'a> Bot {
    pub fn new (token: String, db: Arc<Db>) -> Bot {
        let api = Api::new(token);
        
        Bot {
            api,
            db,
        }
    }

    fn telegram_message_to_message_input(&self, tb_message: Option<tb_types::Message>) -> Option<MessageInput> {
        match tb_message {
            Some(message) => Some(MessageInput::new(
                message.id.to_string(),
                match message.kind {
                    tb_types::MessageKind::Text { data, .. } => {
                        data
                    },
                    _ => "".to_string(),
                },
                message.date,
                // from: User {
                //     uid: "_:user-".to_string() + &message.from.id.to_string(),
                //     first_name: message.from.first_name,
                //     last_name: message.from.last_name,
                //     is_bot: message.from.is_bot,
                //     username: message.from.username,
                // },
                match message.reply_to_message {
                    Some(reply_to_message_or_channel_post) => {
                        match *reply_to_message_or_channel_post {
                            tb_types::MessageOrChannelPost::Message(reply_to_message) => {
                                let db_message = self.db.message.get_message_by_message_id(reply_to_message.id.to_string());

                                match db_message {
                                    Ok(mes) => {
                                        Some(ReplyTo {
                                            uid: mes.uid,
                                        })
                                    },
                                    Err(_) => {
                                        None
                                    },
                                }
                            },
                            _ => {
                                return None;
                            }
                        }
                    },
                    None => None,
                },
                // chat_id
                match i64::try_from(message.chat.id()) {
                    Ok(chat_id) => { chat_id },
                    Err(_) => { 0 }
                },
            )),
            None => None,
        }
    }
    
    fn get_bot_command(&self, message: &tb_types::Message) -> Option<BotCommands> {
        let chat_id = -463169522;
        // TODO enable below when ready for deploy
        // let chat_id = match i64::try_from(message.chat.id()) {
        //     Ok(chat_id) => { chat_id },

        //     // If no chat_id is provided, return immediately
        //     Err(_) => { return None },
        // };

        let (entities, text) = match &message.kind {
            tb_types::MessageKind::Text { entities, data } => {
                (entities, data)
            },
            _ => {
                return None;
            },
        };

        if entities.len() < 1 {
            return None;
        }

        let first_entity = &entities[0];

        if first_entity.offset != 0 {
            return None;
        }

        let bot_command: Option<String> = match first_entity.kind {
            tb_types::MessageEntityKind::BotCommand => {
                Some(text.chars().take(first_entity.length as usize).collect())
            },
            _ => None,
        };

        if bot_command != Some(DIGEST_COMMAND.to_string()) {
            return None;
        }

        return Some(BotCommands::Digest(text.chars().take(first_entity.length as usize + 1).collect()));
    }

    fn make_digests_response(&self, digests: Option<Vec<Digest>>) -> String {
        if digests == None {
            return "No digests found for requested period".to_string();
        }

        let digests_to_use = digests.unwrap();

        if digests_to_use.len() < 1 {
            return "No digests found for requested period".to_string()
        }

        let mut res = "We discussed the following topics:\n\n".to_string();

        for digest in digests_to_use {
            res += &digest.text;
            res += "\n";
            res += &(T_ME_LINK.to_string() + &digest.chat_id.to_string() + "/" + &digest.message_uid);
        }

        return res;
    }

    async fn send_message(&self, message: String, user_id: tb_types::UserId) -> Result<(), Error> {
        self.api.send(SendMessage::new(user_id, message)).await?;
        
        Ok(())
    }



    async fn handle_digest_command(&self, chat_id: i64, days: String, user_id: tb_types::UserId) {
        // User is hardcoded for now.
        let permitted_user_ids = vec![3147171];
        
        if !permitted_user_ids.contains(&i64::try_from(user_id).unwrap_or(0)) {
            return;
        }

        let parsed_days = days.parse::<u32>();

        let days_to_use = match parsed_days {
            Ok(pars_days) => { pars_days },
            // TODO const
            Err(_) => { 3 }
        };

        let start_timestamp = (Utc::now() - Duration::days(days_to_use as i64)).timestamp();

        let digests_res = self.db.digest.get_digests_past_timestamp(start_timestamp);

        let response = match digests_res {
            Ok(digests) => {
                self.make_digests_response(Some(digests))
            },
            Err(_) => {
                self.make_digests_response(None)
            },
        };

        self.send_message(response, user_id).await;
    }

    async fn handle_message(&self, message: tb_types::Message) -> Result<(), Error> {
        let get_bot_command = self.get_bot_command(&message);

        match get_bot_command {
            Some(BotCommands::Digest(days)) => {
                self.handle_digest_command(days, message.from.id).await;
                return Ok(());
            },
            // just continue
            None => {},
        }
        
        let converted_message = self.telegram_message_to_message_input(Some(message));

        match converted_message {
            Some(mes) => {
                match self.db.message.new_message(mes) {
                    Err(e) => {
                        println!("{}", e);
                        return Ok(());
                    },
                    Ok(_) => {
                        return Ok(());
                    }
                }
            },
            None => {
                Ok(())
            }
        }

    }

    pub async fn run(&self) -> Result<(), Error> {
        let mut stream = self.api.stream();
        while let Some(update) = stream.next().await {
            // If the received update contains a new message...
            let update = update?;
            if let tb_types::update::UpdateKind::Message(message) = update.kind {
                if let tb_types::message::MessageKind::Text { .. } = message.kind {
                    // // Print received text message to stdout.
                    // println!("<{}>: {}", &message.from.first_name, data);
                    self.handle_message(message).await?;
                }
            }
        }
        Ok(())
    }
        
}