use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserData {
    pub email: String,
    pub password: String,
}


pub async fn post(
    user_data: web::Json<UserData>
) -> Result<HttpResponse, actix_web::Error> {
    let _user_data = user_data.into_inner();

    Ok(HttpResponse::Ok().json({}))
}