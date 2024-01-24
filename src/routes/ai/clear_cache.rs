
use openai_rs::*;

#[get("/clear-cache")]
pub async fn clear_cache() -> Result<(), ()> {

    
    let mut client = openai_rs::OpenAIAccount::new( Opts {
        ..Default::default()
    }).await.expect("initialization of openai_rs client");

    client.cache.clear();

    Ok(())
}