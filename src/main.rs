use std::env;

use telegram_bot::{Api, Error, GetMe};
use dgraph::make_dgraph;
use std::sync::Arc;
// use tokio::runtime::Handle;
// use tokio::reactor::Core;

mod telegram;
use telegram::{bot, db, digest, scheduler};


#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = "private";
    let db_host = "private";
    
    let client = make_dgraph!(dgraph::new_dgraph_client(&db_host));
    let db = db::Db::new(Arc::new(client));

    let arc_db = Arc::new(db);

    let digest = digest::Digest::new(arc_db.clone());

    scheduler::run(digest);
    
    let summary_bot = bot::Bot::new(token.to_string(), arc_db.clone());

    summary_bot.run().await;

    Ok(())
}