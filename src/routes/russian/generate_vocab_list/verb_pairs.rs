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
pub struct VerbPairList {
    list_entry: Vec<VerbPairEntry>,
}

#[derive(Serialize)]
pub struct VerbPairEntry {
    forms: Vec<String>,
    perfective_lemma: String,
    imperfective_lemma: String,
    frequency: u16, // Of either form
}


#[post("/generate-list/verb-pairs", format = "json", data = "<list_req>")]
pub async fn list_pairs(db: &State<DatabaseConnection> , list_req: Json<GenerateListRequest>) -> Result<Json<VerbPairList>, Status> {

    todo!();

}