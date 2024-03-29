use actix_identity::Identity;
use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::resources::Pool;

#[derive(Debug, Deserialize)]
pub struct UserData {
    pub email: String,
    pub password: String,
}

pub async fn post(
    user_data: web::Json<UserData>,
    pool: web::Data<Pool>,
    id: Identity
) -> Result<HttpResponse, actix_web::Error> {
    let user_data = user_data.into_inner();

    let user = web::block(move || crate::models::user::create(
        &pool,
        &user_data.email,
        &user_data.password
    )).await??;

    let user_string = serde_json::to_string(&user)?;
    id.remember(user_string);

    Ok(HttpResponse::Ok().json(user))
}