use rocket::{State, serde::json::Json, form::Form};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};


// INCOMING DATA
#[derive(FromForm, Deserialize)]
pub struct GenerateListRequest {
    input: String,
    breadth: String,
    style: String,
}

#[derive(FromFormField, Deserialize)]
pub enum Breadth {
    #[field(value = "Full List")]
    FullList,
    #[field(value = "Broad List")]
    BroadList,
    #[field(value = "Top Words")]
    TopWords,
    #[field(value = "Rare Words")]
    RareWords
}

#[derive(FromFormField, Deserialize)]
pub enum Style {
    #[field(value = "Raw Vocabulary")]
    RawVocab,
    #[field(value = "Verb Pairs")]
    VerbPairs,
    #[field(value = "Verb Trees")]
    VerbTrees,
}



// OUTGOING DATA
#[derive(Serialize)]
pub struct GenerateListResponse {
    list: Vec<VocabEntry>,
    metadata: String
}

#[derive(Serialize)]
pub struct VocabEntry {
    form: String,
    lemma: String,
    frequency: u16,
    part_of_speech: Option<String>
}

#[post("/generate-list", format = "json", data = "<list_req>")]
pub async fn generate_list(db: &State<DatabaseConnection> , list_req: Json<GenerateListRequest>) -> Json<GenerateListResponse> {
    
    let example = GenerateListResponse {
        list: vec![VocabEntry {form: "кошки".to_string() , lemma: "кошка".to_string() , frequency: 6, part_of_speech: Some("noun".to_string())}, VocabEntry {form: "человека".to_string() , lemma: "человек".to_string() , frequency: 28, part_of_speech: None}],
        metadata: String::from("Meta")
    };

    Json(example)
    
}