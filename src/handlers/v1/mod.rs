use actix_web::web;

mod user;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/v1")
            .route("/users", web::post().to(user::post))
    );
}