#[macro_use] extern crate rocket; 

pub mod routes;
pub mod security;
pub mod utils;

use {
    sea_orm::{
        TransactionTrait, ConnectionTrait,
        Statement,
        Set, NotSet,
        Database, DatabaseBackend, DatabaseConnection,
        ActiveModelTrait
    },
    dotenvy::dotenv,

    rocket_dyn_templates::{
        Template,
    },
    routes::{
        pages::home::{
            home_page
        },
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

    let db: DatabaseConnection = match Database::connect(dotenvy::var("DATABASE_URL")?).await {
        Ok(db) => db,
        Err(e) => panic!("Error launching: {e}")
    };


    let _rocket = rocket::build()
        .manage(db) // Send db as state to routes

        // Mount my handlers upon this base route for access
        .mount("/", routes![
            home_page
        ])
        .mount("/russian", routes![ 
            word_in_db
        ])
        .mount("/email", routes![
            validate_email,
        ])
        .mount("/convert", routes![
            url_decode,
        ])
        .mount("/clean", routes![

        ])
        .launch()
        .await?;

    Ok(())
    
}