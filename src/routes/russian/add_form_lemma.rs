
use std::fs::write;

use rocket::{http::Status, State};
use sea_orm::ConnectionTrait;
use sea_orm::Database;
use sea_orm::DatabaseBackend;
use sea_orm::DatabaseConnection;

use sea_orm::Statement;
use select::{document::Document};
use std::io::prelude::*;
use std::io::LineWriter;


#[get("/scrape-add/<req>")]
pub async fn scrape_add(db: &State<DatabaseConnection>, req: String) -> Result<Status, Status> {

    let word = req;

    
//

    Ok(Status::Ok)

}