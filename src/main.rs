#[macro_use] extern crate rocket; 

pub mod basic_conversions;
pub mod data_cleaning;
pub mod email;
pub mod security;
pub mod utils;

// use {basic_conversions, 
//     data_cleaning, 
//     email, 
//     security, 
//     utils
// };

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let _rocket = rocket::build()
        .mount("/", routes![

        ])
        .launch()
        .await?;

    Ok(())
    
}