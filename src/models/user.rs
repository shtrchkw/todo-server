use actix_web::web;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use crate::errors::ServiceError;
use crate::resources::Pool;
use crate::schema::users;

#[derive(Debug, Serialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    email: String,
    password_hash: String,
    created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    email: String,
    password_hash: String,
    created_at: chrono::NaiveDateTime,
}

impl NewUser {
    fn from_details<T: Into<String>>(
        email: T,
        password_hash: T,
    ) -> Self {
        NewUser {
            email: email.into(),
            password_hash: password_hash.into(),
            created_at: chrono::Local::now().naive_local(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser {
    pub id: i32,
    pub email: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser {
            id: user.id,
            email: user.email,
        }
    }
}

pub struct AuthQuery {
    pub email: String,
    pub password: String,
}

pub fn verify_auth_data(
    pool: &web::Data<Pool>,
    auth_query: AuthQuery
) -> Result<SlimUser, ServiceError> {
    use crate::schema::users::dsl::{users, email};

    let conn = &pool.get()?;
    let mut result = users
        .filter(email.eq(auth_query.email))
        .limit(1)
        .load::<User>(conn)?;

    if let Some(user) = result.pop() {
        if let Ok(matching) = crate::utils::verify(&user.password_hash, &auth_query.password) {
            if matching {
                return Ok(user.into());
            }
        }
    }

    Err(ServiceError::Unauthorized)
}

pub fn create(
    pool: &web::Data<Pool>,
    email: &str,
    password: &str,
) -> Result<SlimUser, ServiceError> {
    use crate::schema::users::dsl::users;

    let password_hash = crate::utils::hash_password(password)?;

    let new_user = NewUser::from_details(email, &password_hash);
    let conn = &pool.get()?;
    let inserted_user: User = diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)?;

    Ok(inserted_user.into())
}