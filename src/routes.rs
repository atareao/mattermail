use actix_web::{get, post, Error, HttpResponse, http::StatusCode, http::header::ContentType};
use serde::Serialize;

#[derive(Serialize)]
struct Respuesta{
    code: i32,
    status: String,
    message: String,
}


#[get("/")]
pub async fn root() -> Result<HttpResponse, Error>{
    Ok(HttpResponse::build(StatusCode::OK).body("Rust is the best!"))
}

#[get("/status")]
pub async fn status() -> Result<HttpResponse, Error>{
    let respuesta = Respuesta{
        code: 200,
        status: "Ok".to_string(),
        message: "Up and running!".to_string(),
    };
    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(serde_json::to_string(&respuesta)?))
}

#[post("/hook")]
pub async fn hook(post: String) -> Result<HttpResponse, Error>{
    println!("{}", post);
    Ok(HttpResponse::build(StatusCode::OK).body(format!("Message recieved {}", post)))
}
