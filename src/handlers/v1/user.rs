use actix_web::HttpResponse;

pub async fn post() -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok().body("user::post"))
}