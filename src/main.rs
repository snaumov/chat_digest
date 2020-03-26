use std::env;

use telegram_bot::{Api, Error, GetMe};
use dgraph::make_dgraph;
// use tokio::runtime::Handle;
// use tokio::reactor::Core;

mod telegram;
use telegram::{bot, db};


#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = "private";
    let db_host = "private";
    
    let client = make_dgraph!(dgraph::new_dgraph_client(&db_host));
    let db = db::Db::new(&client);
    
    let summary_bot = bot::Bot::new(token.to_string(), &db);

    summary_bot.run().await;

    Ok(())
}