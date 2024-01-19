use std::str::FromStr;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

pub fn setup_cors() -> rocket_cors::Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "https://rubit.vercel.app/"
    ]);

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Options, Method::Delete, Method::Put].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    cors
}