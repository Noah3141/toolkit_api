use sea_orm::{DatabaseBackend, ConnectionTrait};



#[macro_use] extern crate rocket; 

pub mod routes;
pub mod security;
pub mod utils;

use {
    sea_orm::{
        TransactionTrait, Statement,
        Set, 
        NotSet,
        Database,
        DatabaseConnection,
        ActiveModelTrait},
    dotenvy::dotenv,
    routes::{
        basic_conversions::{
            url_decode::url_decode
        },
        email::{
            validate_email::validate_email
        },
        russian::{
            word_in_db::word_in_db
        }
    }   
};

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    
    dotenv().expect(".env not found!");

    let db: DatabaseConnection = match Database::connect(dotenvy::var("DATABASE_URL")?).await {
        Ok(db) => db,
        Err(e) => panic!("Error launching: {e}")
    };


    let _rocket = rocket::build()
        .manage(db)
        .mount("/", routes![
            url_decode,
            validate_email,
            word_in_db
        ])
        .launch()
        .await?;

    Ok(())
    
}