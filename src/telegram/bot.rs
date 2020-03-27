// use telegram_bot::{Api, Error, GetMe};
use super::db::{Db, Message, User, ReplyTo};
use telegram_bot::{types as tb_types, Error, Api};
use futures::StreamExt;
use std::sync::Arc;

use dgraph::{DgraphError};

pub struct Bot {
    api: Api,
    db: Arc<Db>,
}

#[derive(PartialEq)]
enum BotCommands {
    Digest,
}

impl<'a> Bot {
    pub fn new (token: String, db: Arc<Db>) -> Bot {
        let api = Api::new(token);
        
        Bot {
            api,
            db,
        }
    }

    fn telegram_message_to_message(&self, tb_message: Option<tb_types::Message>) -> Option<Message> {
        match tb_message {
            Some(message) => Some(Message {
                uid: "_:message-".to_string() + &message.id.to_string(),
                message_id: message.id.to_string(),
                text: match message.kind {
                    tb_types::MessageKind::Text { data, .. } => {
                        data
                    },
                    _ => "".to_string(),
                },
                date: message.date,
                // from: User {
                //     uid: "_:user-".to_string() + &message.from.id.to_string(),
                //     first_name: message.from.first_name,
                //     last_name: message.from.last_name,
                //     is_bot: message.from.is_bot,
                //     username: message.from.username,
                // },
                reply_to: match message.reply_to_message {
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
            }),
            None => None,
        }
    }
    
    fn get_bot_command(&self, message: &tb_types::Message) -> Option<BotCommands> {
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

        if bot_command != Some("digest".to_string()) {
            return None;
        }

        return Some(BotCommands::Digest);
    }

    fn handle_digest_command(&self, message: tb_types::Message) {

    }

    fn handle_message(&self, message: tb_types::Message) -> Result<(), Error> {
        let get_bot_command = self.get_bot_command(&message);

        if get_bot_command == Some(BotCommands::Digest) {
            self.handle_digest_command(message);
            return Ok(())
        }
        
        let converted_message = self.telegram_message_to_message(Some(message));

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
                    self.handle_message(message)?;
                }
            }
        }
        Ok(())
    }
        
}