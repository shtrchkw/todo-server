use actix_web::web;
use diesel::prelude::*;
use serde::Serialize;

use crate::errors::ServiceError;
use crate::resources::Pool;
use crate::schema::todo;

#[derive(Debug, Serialize, Queryable, Insertable)]
#[table_name = "todo"]
pub struct Todo {
    id: i32,
    title: String,
    description: String,
    created_at: chrono::NaiveDateTime,
    todo_status_id: i32,
    user_id: i32,
}

#[derive(Debug, Serialize, Queryable, Insertable)]
#[table_name = "todo"]
pub struct NewTodo {
    title: String,
    description: String,
    created_at: chrono::NaiveDateTime,
    todo_status_id: i32,
    user_id: i32,
}

impl NewTodo {
    fn from_details<S: Into<String>, T: Into<i32>>(
        title: S,
        description: S,
        user_id: T
    ) -> Self {
        NewTodo {
            title: title.into(),
            description: description.into(),
            created_at: chrono::Local::now().naive_local(),
            todo_status_id: 1, // Pending
            user_id: user_id.into(),
        }
    }
}

pub fn create(
    pool: &web::Data<Pool>,
    title: &str,
    description: &str,
    user_id: i32
) -> Result<Todo, ServiceError> {
    use crate::schema::todo::dsl::todo;

    let new_todo = NewTodo::from_details(title, description, user_id);
    let conn = &pool.get()?;
    let inserted_todo: Todo = diesel::insert_into(todo)
        .values(&new_todo)
        .get_result(conn)?;

    Ok(inserted_todo)
}

pub fn delete(
    pool: &web::Data<Pool>,
    todo_id: i32,
    user_id: i32,
) -> Result<Todo, ServiceError> {
    use crate::schema::todo::dsl::{todo, id, user_id as todo_user_id};

    let conn = &pool.get()?;
    let deleted_todo = diesel::delete(
        todo
            .filter(id.eq(todo_id))
            .filter(todo_user_id.eq(user_id))
    )
    .get_result(conn)?;

    Ok(deleted_todo)  
}