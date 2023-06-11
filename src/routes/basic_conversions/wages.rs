

#[get("/to-hourly/<req>")]
pub async fn hourly(req: u32) -> String {

    let hourly = req/40/50;
    String::from(format!("${hourly}/hr"))

}


#[get("/to-yearly/<req>")]
pub async fn yearly(req: u32) -> String {

    let yearly = req*40*50;
    String::from(format!("${yearly}/yr"))


}