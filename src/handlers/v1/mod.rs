use actix_web::web;

mod auth;
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
    );
}