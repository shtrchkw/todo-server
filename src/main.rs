#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpServer};
use diesel::PgConnection;
use diesel::r2d2::{self, ConnectionManager};

mod handlers;
mod models;
mod errors;
mod resources;
mod schema;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let environment = match std::env::var("ENVIRONMENT") {
        Ok(val) => val,
        Err(_) => "local".to_string(),
    };
    dotenv::from_filename(".env.".to_string() + &environment).ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: resources::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(handlers::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}