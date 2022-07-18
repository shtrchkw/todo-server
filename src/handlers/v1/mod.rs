use actix_web::web;

mod auth;
mod todo;
mod user;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .route("/users", web::post().to(user::post))
            .service(
                web::resource("/auth")
                    .route(web::get().to(auth::get))
                    .route(web::post().to(auth::post))
                    .route(web::delete().to(auth::delete))
            )
            .service(
                web::resource("/todo")
                    .route(web::post().to(todo::post))
            )
            .route("/todo/{todo_id}", web::patch().to(todo::patch))
            .route("/todo/{todo_id}", web::delete().to(todo::delete))
    );
}