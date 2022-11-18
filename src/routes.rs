use actix_web::{get, post, Error, HttpResponse, http::StatusCode, http::header::ContentType, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct Respuesta{
    code: i32,
    status: String,
    message: String,
}


#[get("/")]
pub async fn root() -> impl Responder{
    HttpResponse::Ok().body("Rust is the best!")
}

#[get("/status")]
pub async fn status() -> impl Responder{
    HttpResponse::Ok()
        .body(serde_json::json!({
            "code": 200,
            "status": "Ok",
            "message": "Up and running"
        }).to_string())
}

#[post("/hook")]
pub async fn hook(post: String) -> impl Responder{
    HttpResponse::Ok().body(format!("Message recievend {}", post))
}
