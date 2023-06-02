use rocket_dyn_templates::Template;

#[macro_use] extern crate rocket; 

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let _rocket = rocket::build()
        .mount("/", routes![
        ])
        .attach(Template::fairing())
        .launch()
        .await?;

    Ok(())
    
}