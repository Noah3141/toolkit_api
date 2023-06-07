use rocket::serde::json::Json;
use crate::routes::russian::utils::{input_cleaning::clean, input_parsing::wordify};

#[post("/delint", format = "json", data = "<text>")]
pub async fn delint(mut text: Json<String>) -> String {

    clean(text.into_inner())

}

#[get("/test")]
pub async fn test() -> Json<String> {

    Json(
        String::from("djfaposdijfaposdijf poidsjf poaisjdfpo asidjfpo aisjdfpoasijdfpo iasjdfp oasjdf ;oasijdfpoasidjf poasidjf paosdifjapsodifj\n a'pdsfjaosidfjaposdifjpoasidfjaspodifjapsdofija\n ajdfpoaisdjfpoasidf")
    )
}

#[post("/wordify", format = "json", data = "<text>")]
pub async fn wordify_text(mut text: Json<String>) -> Json<Vec<String>> {

    Json(
        wordify(clean(text.into_inner()))
    )
}