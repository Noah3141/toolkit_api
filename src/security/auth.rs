use rocket::serde::json::Json;


struct Auth {
    name: String,
    token: String,
}

#[get("/auth")]
pub fn first_time_auth() -> () {

    todo!();
    

}