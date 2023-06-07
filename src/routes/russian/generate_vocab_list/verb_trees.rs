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
pub struct VerbTreeList {
    list_entry: Vec<VerbTreeEntry>
}
#[derive(Serialize)]
pub struct VerbTreeEntry {
    forms: Vec<String>,
    tree_stem: String,
    prefixes_encountered: Vec<String>,
    
}


#[post("/generate-list/verb-trees", format = "json", data = "<list_req>")]
pub async fn list_trees(db: &State<DatabaseConnection> , list_req: Json<GenerateListRequest>) -> Result<Json<VerbTreeList>, Status> {

    todo!();

}