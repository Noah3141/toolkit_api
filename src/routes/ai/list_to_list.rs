
use std::{borrow::BorrowMut, path::PathBuf};

use openai_rs::*;
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
    pub cached: bool
}



#[post("/list-to-list", format = "json", data = "<request>")]
pub async fn handle_list_to_list(request: Json<Request>) -> Result<Json<Response>, Json<Response>> {
    let start = chrono::Utc::now().timestamp();

    let mut client = openai_rs::OpenAIAccount::new( &Opts {
        temperature: request.temperature,
        bill_filepath:  PathBuf::from("./routes/ai/client/bill.json"),
        cache_filepath: PathBuf::from("./routes/ai/client/cache.json"),
        ..Default::default()
    }).await.expect("initialization of openai_rs client");

    let mut response_lines: Vec<String> = vec![];
    let mut cached = true;

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
        
        if !cquery.from_cache {
            cached = false
        }

        let msg = cquery.response.choices[0].clone().message.content.expect("msg content");

        let gpt_lines = serde_json::from_str::<Vec<String>>(msg.as_str())
            .map_err(|e| { 
                println!("GPT: \n{:?}\n", response_lines.clone());
                Json(Response {
                    err: Some(format!("GPT responded with a value that could not be JSON deserialized.{:#?}", e )),
                    lines: vec![],
                    process_time: 0,
                    cached: false,
                })
            })?;

        match &request.expose_indices {
            Some(indices) => {
                let mut i: usize = 0;
                for line in chunk {
                    let replacement_snippet = &gpt_lines[i];
                    let mut fixed_line = line.clone();
                    fixed_line.replace_range(indices.start..indices.end, replacement_snippet.as_str());
                    response_lines.push(fixed_line);
                    i += 1;
                }

            },
            None => {
                for line in gpt_lines {
                    response_lines.push(line)
                }
            }
            ,
        }
    }

    println!("GPT: \n{:?}\n", response_lines.clone());

    if response_lines.len() != request.lines.len() {
        return Err(Json(Response {
            err: Some(format!("Number of lines in response ({}) did not match number in request ({}).", response_lines.len(), request.lines.len() )),
            lines: vec![],
            process_time: 0,
            cached: false,
        }))
    }


    let end = chrono::Utc::now().timestamp();
    let process_time = end - start;


    Ok(Json(Response {
        lines: response_lines,
        process_time,
        err: None,
        cached
    }))
}