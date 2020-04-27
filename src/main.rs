use telegram_bot::{Error};
use dgraph::make_dgraph;
use std::sync::Arc;
use std::thread;

#[macro_use]
extern crate dotenv_codegen;

mod telegram;
use telegram::{bot, db, digest, scheduler};


#[tokio::main]
async fn main() -> Result<(), Error> {
<<<<<<< HEAD
    let token = "private";
    let db_host = "private";
=======
    // TODO move env vars
    let token = dotenv!("TG_TOKEN");
    let db_host = dotenv!("DB_HOST");
>>>>>>> 9f9718b... Init commit
    
    let client = make_dgraph!(dgraph::new_dgraph_client(&db_host));
    let db = db::Db::new(Arc::new(client));

    let arc_db = Arc::new(db);

    let digest = digest::Digest::new(arc_db.clone());

    // thread::spawn(move || {
    //     scheduler::run(digest);
    // });
    
    let summary_bot = bot::Bot::new(token.to_string(), arc_db.clone());

    summary_bot.run().await;

    Ok(())
}