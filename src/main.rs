#[macro_use] extern crate rocket; 

pub mod basic_conversions;
pub mod data_cleaning;
pub mod email;
pub mod security;
pub mod utils;

use {
    basic_conversions::{
        url_encode::url_decode}, 
};

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let _rocket = rocket::build()
        .mount("/", routes![
            url_decode
        ])
        .launch()
        .await?;

    Ok(())
    
}