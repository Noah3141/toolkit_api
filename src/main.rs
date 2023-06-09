#![allow(unused)]
#[macro_use]
extern crate rocket;
extern crate rocket_cors;

pub mod routes;
pub mod security;
pub mod utils;

use {
    sea_orm::{
        Database, DatabaseConnection,
    },

    routes::{
        basic_conversions::{
            url_decode::url_decode,
            wages::{hourly, yearly}
        },
        email::{
            validate_email::validate_email
        },
        russian::{
            word_in_db::word_in_db,
            add_form_lemma::scrape_add,
            generate_vocab_list::{
                raw_vocabulary::list_vocab,
                verb_pairs::list_pairs,
                verb_trees::list_trees,
            },
            generate_sentence::{
                generate_russian_example::gpt_gen_russian_sentence,
            }
        },
        data_cleaning::wordify::{
            test,
            delint,
            wordify_text
        }   
    }
};



#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let gpt_key = dotenvy::var("CHATGPT_API_KEY").expect("presence and access to api key");
    let db: DatabaseConnection = match Database::connect(dotenvy::var("DATABASE_URL")?).await {
        Ok(db) => db,
        Err(e) => panic!("Error launching: {e}")
    };
    let cors = utils::cors::setup_cors();

    let _rocket = rocket::build()
        .manage(db) // Send db as state to routes
        .attach(cors)

        // Mount my handlers upon this base route for access
        .mount("/russian", routes![ 
            word_in_db,
            list_vocab,
            list_pairs,
            list_trees,
            scrape_add,
            gpt_gen_russian_sentence,
        ])
        .mount("/email", routes![
            validate_email,
        ])
        .mount("/convert", routes![
            url_decode,
            hourly,
            yearly
        ])
        .mount("/clean", routes![
            test,
            delint,
            wordify_text,
        ])
        .launch()
        .await?;

    Ok(())
    
}

