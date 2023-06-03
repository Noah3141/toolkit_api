use {
    rocket::{serde::{json::Json, Deserialize}},
    urlencoding::decode
};

#[get("/convert/urldecode/<req>")]
pub async fn url_decode(req: String) -> String {
    let content = req;

    let decoded = match decode(&content) {
        Ok(decoded) => decoded.into_owned(),
        Err(e) => e.utf8_error().to_string() 
    };

    let res = decoded;
    String::from(res)
}