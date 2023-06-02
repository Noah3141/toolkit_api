use rocket_dyn_templates::Template;

#[get("/index")]
pub fn index() -> Template {
    let context = serde_json::json!({"logged-in": false});
    Template::render("index", &context)
}
