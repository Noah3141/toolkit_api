
use std::borrow::BorrowMut;

use openai_rs::{OpenAIAccount, Opts};
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Request {
    /// What to pair your lines with when sent to GPT
    pub prompt: String,
    /// The lines of text you want formatted. Should include enough iters to catpure the proper patterns behind the mistakes.
    pub lines: Vec<String>,
    /// If provided, only send to GPT this particular portion of each line
    pub expose_indices: Option<ExposeOpts>,
    pub temperature: f32,
    pub chunk: usize,
}

#[derive(Deserialize)]
pub struct ExposeOpts {
    pub start: usize,
    pub end: usize,
}


#[derive(Serialize)]
pub struct Response {
    pub err: Option<String>,
    pub lines: Vec<String>,
    pub process_time: i64,
}



#[post("/fix-formatting", format = "json", data = "<request>")]
pub async fn fix_formatting(request: Json<Request>) -> Json<Response> {
    let start = chrono::Utc::now().timestamp();

    let mut client = openai_rs::OpenAIAccount::new( Opts {
        temperature: request.temperature,
        ..Default::default()
    }).await.expect("initialization of openai_rs client");

    let mut response_lines: Vec<String> = vec![];

    for chunk in request.lines.chunks(request.chunk) {
        let mut lines = String::new();
        for line in chunk {
            let line: String =  match &request.expose_indices {
                Some(idx) => {
                    line.get(idx.start.. idx.end).expect("indexing success").to_string()
                },
                None => {
                    line.to_owned()
                }
            };

            lines.push_str(format!("{line}\n").as_str())
        }


        let cquery = client.get_completion(
            format!("
            {}

            {}
            ", request.prompt, lines).as_str()
        ).await.expect("client completion success");

        let msg = cquery.response.choices[0].clone().message.content.expect("msg content");
        let res_lines: Vec<&str> = msg.split("\n").collect();

        match &request.expose_indices {
            Some(indices) => {
                let mut i: usize = 0;
                for line in chunk {
                    let replacement_snippet = res_lines[i];
                    let mut fixed_line = line.clone();
                    fixed_line.replace_range(indices.start..indices.end, replacement_snippet);
                    response_lines.push(fixed_line);
                    i += 1;
                }

            },
            None => response_lines.push(msg)
            ,
        }
    }


    client.cache.clear();

    let end = chrono::Utc::now().timestamp();
    let process_time = end - start;

    println!("GPT: \n{:?}\n", response_lines.clone());

    Json(Response {
        lines: response_lines,
        process_time,
        err: None,
    })
}