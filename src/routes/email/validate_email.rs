use {
    rocket::{serde::{json::Json}},
    check_if_email_exists::{
        check_email, 
        CheckEmailInput,
        CheckEmailOutput 
    }
};

#[get("/email/validate/<req>")]
pub async fn validate_email(req: String) -> Json<CheckEmailOutput> {
    let email = req;
    let input = CheckEmailInput::new(email);
    let result = check_email(&input).await;
    Json(result)
}