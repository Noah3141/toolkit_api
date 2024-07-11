
use std::{borrow::BorrowMut, path::PathBuf};

use openai_rs::*;
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};


#[derive(Deserialize)]
pub struct Request {
    /// What to pair your lines with when sent to GPT
    pub prompt: String,
    /// The lines of text you want formatted. Should include enough iters to catpure the proper patterns behind the mistakes.
    pub text: String,
    /// If provided, only send to GPT this particular portion of each line
    pub temperature: f32,
    pub model: GptModel
}


#[derive(Serialize)]
pub struct Response {
    pub err: Option<String>,
    pub text: String,
    pub process_time: i64,
    pub cached: bool
}



#[post("/string-to-string", format = "json", data = "<request>")]
pub async fn handle_str_to_str(request: Json<Request>) -> Json<Response> {
    let start = chrono::Utc::now().timestamp();

    let mut client = openai_rs::OpenAIAccount::new( &Opts {
        model: request.model,
        temperature: request.temperature,
        bill_filepath:  PathBuf::from("./src/routes/ai/client/bill.json"),
        cache_filepath: PathBuf::from("./src/routes/ai/client/cache.json"),
        ..Default::default()
    }).await.expect("initialization of openai_rs client");

    let cquery = client
        .get_completion(
            format!("
            {}

            {}
            ", request.prompt, request.text).as_str()
        )
        .await
        .expect("Succesful API repsonse");
    

    let text = cquery.response.choices[0].clone().message.content.unwrap();

    let end = chrono::Utc::now().timestamp();
    let process_time = end - start;

    println!("GPT: \n'{}'\n", text);

    Json(Response {
        text: text,
        process_time,
        err: None,
        cached: cquery.from_cache,
    })
}