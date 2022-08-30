use actix_web::{get, post, Error, HttpResponse, http::StatusCode};


#[get("/")]
pub async fn root() -> Result<HttpResponse, Error>{
    Ok(HttpResponse::build(StatusCode::OK).body("Hola mundo"))
}
