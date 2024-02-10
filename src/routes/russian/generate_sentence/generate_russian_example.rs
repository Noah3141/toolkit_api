use openai_api_rs::v1::completion::CompletionRequest;
use rocket::{State, http::Status, serde::json::Json};
use openai_api_rs::v1::chat_completion;
use openai_api_rs::v1::api::Client;

#[get("/generate-sentence/<req>")]
pub fn gpt_gen_russian_sentence(req: String) -> Result<String, Status> {

    let lemma = req;
    println!("Request for word: {lemma}");
    use std::env;
    let client = Client::new(env::var("CHATGPT_API_KEY").unwrap().to_string());


    let req = chat_completion::ChatCompletionRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: chat_completion::Content::Text(format!("Write me an example Russian sentence that uses the word {lemma}, followed by the English translation in parentheses.")),
            name: None,
        }],
        temperature: None,
        top_p: None,
        n: None,
        response_format: None,
        stream: None,
        stop: None,
        max_tokens: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
        seed: None,
        tools: None,
        tool_choice: None
        
    };
    
    let result = match client.chat_completion(req)
        {
            Ok(t) => t, 
            Err(e) => {
                println!("{:?}", e);
                return Err(Status::FailedDependency)
            }
        };
    let response_sentence = match &result.choices[0].message.content {Some(s)=>s.to_owned(), None=>return Err(Status::FailedDependency)};
    
    Ok(response_sentence)

}