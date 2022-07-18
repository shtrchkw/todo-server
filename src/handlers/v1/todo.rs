use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::handlers::v1::auth::LoggedUser;
use crate::resources::Pool;

#[derive(Debug, Deserialize)]
pub struct TodoData {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoData {
    pub todo_status_id: i32,
}

pub async fn get(
    pool: web::Data<Pool>,
    logged_user: LoggedUser
) -> Result<HttpResponse, actix_web::Error> {

    let todo = web::block(move ||
        crate::models::todo::get(
            &pool,
            logged_user.id
        )
    ).await??;

    Ok(HttpResponse::Ok().json(todo))
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

pub async fn patch(
    todo_id: web::Path<i32>,
    update_todo_data: web::Json<UpdateTodoData>,
    pool: web::Data<Pool>,
    logged_user: LoggedUser
) -> Result<HttpResponse, actix_web::Error> {
    let todo = web::block(move ||
        crate::models::todo::update(
            &pool,
            todo_id.into_inner(),
            logged_user.id,
            update_todo_data.into_inner().todo_status_id
        )
    ).await??;

    Ok(HttpResponse::Ok().json(todo))
}

pub async fn delete(
    todo_id: web::Path<i32>,
    pool: web::Data<Pool>,
    logged_user: LoggedUser
) -> Result<HttpResponse, actix_web::Error> {

    let todo = web::block(move ||
        crate::models::todo::delete(
            &pool,
            todo_id.into_inner(),
            logged_user.id
        )
    ).await??;

    Ok(HttpResponse::Ok().json(todo))
}