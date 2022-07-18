use actix_identity::Identity;
use actix_web::{dev::Payload, web, Error, FromRequest, HttpResponse, HttpRequest};
use futures::future::{err, ok, Ready};
use serde::Deserialize;

use crate::models::user;
use crate::errors::ServiceError;
use crate::resources::Pool;

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

pub type LoggedUser = user::SlimUser;

impl FromRequest for LoggedUser {
    type Error = Error;
    type Future = Ready<Result<LoggedUser, Error>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        if let Ok(identity) = Identity::from_request(req, pl).into_inner() {
            if let Some(user_json) = identity.identity() {
                if let Ok(user) = serde_json::from_str(&user_json) {
                    return ok(user);
                }
            }
        }
        err(ServiceError::Unauthorized.into())
    }
}

impl From<AuthData> for user::AuthQuery {
    fn from(auth_data: AuthData) -> Self {
        user::AuthQuery {
            email: auth_data.email,
            password: auth_data.password,
        }
    }
}

pub async fn get(
    logged_user: LoggedUser
) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().json(logged_user))
}

pub async fn post(
    auth_data: web::Json<AuthData>,
    pool: web::Data<Pool>,
    id: Identity
) -> Result<HttpResponse, actix_web::Error> {
    let auth_data = auth_data.into_inner();
    let user = web::block(move || user::verify_auth_data(&pool, auth_data.into())).await??;

    let user_string = serde_json::to_string(&user)?;
    id.remember(user_string);

    Ok(HttpResponse::Ok().json(user))
}

pub async fn delete(
    id: Identity
) -> Result<HttpResponse, actix_web::Error> {
    id.forget();
    Ok(HttpResponse::Ok().finish())
}