#[macro_use] extern crate rocket; 

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let _rocket = rocket::build()
        .mount("/", routes![
        ])
        .launch()
        .await?;

    Ok(())
    
}