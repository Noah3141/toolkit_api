use rocket_dyn_templates::{Template, context};

#[get("/home")]
pub async fn home_page() -> Template {
    Template::render("home", context! { user: "unknown"})
}