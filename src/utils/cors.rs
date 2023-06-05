use std::str::FromStr;

// use rocket::http::{Header, Method};
// use rocket::{Request, Response};
// use rocket::fairing::{Fairing, Info, Kind};

// pub struct CORS;

// #[rocket::async_trait]
// impl Fairing for CORS {
//     fn info(&self) -> Info {
//         Info {
//             name: "Add CORS headers to responses",
//             kind: Kind::Response
//         }
//     }

// }

// ?) Method 2, involving actual setting of the ALLOWED_METHODS


use rocket_cors::{
    AllowedHeaders, AllowedOrigins, Error,
    Cors, CorsOptions, AllowedMethods, 
};


pub fn setup_cors() -> Cors {
    
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:3000",
        "http://127.0.0.1:3000",
        "http://localhost:3000",
        "http://0.0.0.0:3000",
    ]);

    let allowed_methods: AllowedMethods = ["Get", "Post", "Delete"]
        .iter()
        .map(|s| FromStr::from_str(s).unwrap())
        .collect();

    CorsOptions { 
        allowed_origins,
        allowed_methods, 
        // allowed_headers: AllowedHeaders::some(&[
        //     "Authorization",
        //     "Accept",
        //     "Access-Control-Allow-Origin", 
        // ]),
        // allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}
