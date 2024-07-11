
use std::path::PathBuf;

use openai_rs::*;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct Request {
    pub cache_key: String,
}

#[derive(Serialize)]
pub struct Response {
}

#[post("/remove-from-cache", format = "json", data = "<request>")]
pub async fn remove_from_cache(request: Json<Request>) -> Result<Json<Response>, String> {

    let key = request.0.cache_key;
    let mut client = openai_rs::OpenAIAccount::new(&Opts {
        bill_filepath:  PathBuf::from("./src/routes/ai/client/bill.json"),
        cache_filepath: PathBuf::from("./src/routes/ai/client/cache.json"),
        ..Default::default()
    }).await.expect("initialization of openai_rs client");

    client.cache.remove(key);

    Ok(Json(Response {

    }))
}