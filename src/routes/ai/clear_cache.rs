
use std::{borrow::Borrow, path::PathBuf};

use openai_rs::*;

#[get("/clear-cache")]
pub async fn clear_cache() -> Result<(), ()> {
    let mut client = openai_rs::OpenAIAccount::new(&Opts {
        bill_filepath:  PathBuf::from("./src/routes/ai/client/bill.json"),
        cache_filepath: PathBuf::from("./src/routes/ai/client/cache.json"),
        ..Default::default()
    }).await.expect("initialization of openai_rs client");

    client.cache.clear();

    Ok(())
}