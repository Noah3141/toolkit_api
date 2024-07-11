#![allow(unused)]

use std::path::PathBuf;

use openai_rs::Opts;
#[macro_use]
extern crate rocket;

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
        rubit::{
            generate_vocab_list::{
                raw_vocabulary::list_vocab,
                verb_pairs::list_pairs,
                verb_trees::list_trees,
            },
            generate_sentence:: {
                generate_russian_example::gpt_gen_russian_sentence,
            }
        },
        data_cleaning::{
            wordify::{
                test,
                delint,
                wordify_text
            },
            ai_format_fix::{
                fix_formatting
            }  
        },
        ai::{
            clear_cache::clear_cache,
            list_to_list::handle_list_to_list,
            str_to_str::handle_str_to_str
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

    let mut client = openai_rs::OpenAIAccount::new( &Opts {
        temperature: 0.2,
        bill_filepath:  PathBuf::from("./routes/ai/client/bill.json"),
        cache_filepath: PathBuf::from("./routes/ai/client/cache.json"),
        ..Default::default()
    }).await.expect("initialization of openai_rs client");

    let _rocket = rocket::build()
        .manage(db) // Send db as state to routes
        .attach(utils::cors::setup_cors())

        // Mount my handlers upon this base route for access
        .mount("/russian", routes![ 
            list_vocab,
            list_pairs,
            list_trees,
            gpt_gen_russian_sentence,
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
            fix_formatting
        ])
        .mount("/ai", routes![
            handle_list_to_list,
            handle_str_to_str,
            clear_cache
        ])
        .launch()
        .await?;

    Ok(())
    
}

