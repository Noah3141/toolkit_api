use rocket::{State, serde::json::Json};
use sea_orm::DatabaseConnection;
use sea_orm::entity::prelude::*;
use super::models::russian_words::*;


#[get("/russian/in_check/<req>")]
pub async fn word_in_db(db: &State<DatabaseConnection> , req: String) -> String {
    
    println!("requested string: {req}");
    // validate req as a word
    let word = req;

    let result = Entity::find()
        .filter(Column::Form.contains(word))
        .one(db.inner())
        .await;
    
    let res = match result {
        Ok(r)  => match r {
            Some(m) => "true".to_string(),
            None => "false".to_string()
        },
        Err(e) => {
            println!("Error with database: {e}");
            String::from("There was an error acquiring a database response.")
        }
    };
    res
}