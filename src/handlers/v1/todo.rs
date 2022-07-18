use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::handlers::v1::auth::LoggedUser;
use crate::resources::Pool;

#[derive(Debug, Deserialize)]
pub struct TodoData {
    pub title: String,
    pub description: String,
}

pub async fn post(
    todo_data: web::Json<TodoData>,
    pool: web::Data<Pool>,
    logged_user: LoggedUser
) -> Result<HttpResponse, actix_web::Error> {
    let todo_data = todo_data.into_inner();

    let todo = web::block(move || 
        crate::models::todo::create(
            &pool,
            &todo_data.title,
            &todo_data.description,
            logged_user.id
        )
    ).await??;

    Ok(HttpResponse::Ok().json(todo))
}