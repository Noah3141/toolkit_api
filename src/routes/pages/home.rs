
use rocket::{response::Responder, fs::NamedFile};
use rocket_dyn_templates::{Template, context};

#[derive(Responder)]
pub enum FrontEndRes {
    Html(Template),
    Link(NamedFile)
}

#[get("/<raw_req>")]
pub async fn home_page(raw_req: String) -> FrontEndRes {

    if raw_req.starts_with("front_end_yew-") {

        return FrontEndRes::Link( NamedFile::open( format!("./routes/templates/front_end_yew/dist/{raw_req}") ).await.expect("hardcoded files") )
    }
    else if raw_req.starts_with("_trunk") {

    }


    FrontEndRes::Html(Template::render("index", context! { user: "unknown"}))
}