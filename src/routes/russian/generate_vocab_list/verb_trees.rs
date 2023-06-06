use sea_orm::DatabaseConnection;
use rocket::{State, http::Status, serde::json::Json};
use super::super::exchange_models::{GenerateListRequest, GenerateListResponse};

#[post("/generate-list/verb-trees", format = "json", data = "<list_req>")]
pub async fn list_trees(db: &State<DatabaseConnection> , list_req: Json<GenerateListRequest>) -> Result<Json<GenerateListResponse>, Status> {
  
    todo!();

}