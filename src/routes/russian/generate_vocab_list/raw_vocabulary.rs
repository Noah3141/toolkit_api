use sea_orm::DatabaseConnection;
use rocket::{State, http::Status, serde::json::Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct GenerateListRequest {
    input_text: String,
    style: String,
    breadth: String,
}

#[derive(Serialize)]
pub struct RawVocabularyList {
    list_entry: Vec<RawVocabEntry>,
    metadata: String
}

#[derive(Serialize)]
pub struct RawVocabEntry { // Verb, Noun, Adjective ...
    forms: Vec<String>,
    lemma: String,
    frequency: u16,
    perfective: Option<bool>,
}


#[post("/generate-list/raw-vocabulary", format = "json", data = "<list_req>")]
pub async fn list_vocab(db: &State<DatabaseConnection> , list_req: Json<GenerateListRequest>) -> Result<Json<RawVocabularyList>, Status> {

    
    todo!();



}