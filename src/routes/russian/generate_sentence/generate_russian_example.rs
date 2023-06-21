use openai_api_rs::v1::completion::CompletionRequest;
use rocket::{State, http::Status, serde::json::Json};
use openai_api_rs::v1::chat_completion;

#[get("/generate-sentence/<req>")]
pub async fn gpt_gen_russian_sentence(req: String) -> Result<String, Status> {

    let lemma = req;
    println!("Request for word: {lemma}");
    use openai_api_rs::v1::api::Client;
    use std::env;
    let client = Client::new(env::var("CHATGPT_API_KEY").unwrap().to_string());

    use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
    let req = chat_completion::ChatCompletionRequest {
        model: chat_completion::GPT3_5_TURBO.to_string(),
        messages: vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: Some(format!("Write me an example Russian sentence that uses the word {lemma}, followed by the English translation in parentheses.")),
            name: None,
            function_call: None,
        }],
        functions: None,
        function_call: None,
    };
    
    let result = match client.chat_completion(req)
        .await {
            Ok(t) => t, 
            Err(e) => {
                println!("{:?}", e);
                return Err(Status::FailedDependency)
            }
        };
    let response_sentence = match &result.choices[0].message.content {Some(s)=>s.to_owned(), None=>return Err(Status::FailedDependency)};
    
    Ok(response_sentence)

}