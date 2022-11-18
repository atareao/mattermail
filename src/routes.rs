use std::ptr::null;

use actix_web::{get, post, web, Error, HttpResponse, http::StatusCode, http::header::{ContentType, Accept}, Responder};
use serde::{Serialize, Deserialize};
use log::info;
use serde_json::json;

use crate::mail::read_mail;


#[derive(Serialize)]
struct Respuesta{
    code: i32,
    status: String,
    message: String,
}

#[derive(Deserialize)]
struct MatterCommandHook{
    channel_id: String,
    channel_name: String,
    command: String,
    response_url: String,
    team_domain: String,
    team_id: String,
    text: String,
    token: String,
    trigger_id: String,
    user_id: String,
    user_name: String,
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
pub async fn hook(form: web::Form<MatterCommandHook>) -> impl Responder{
    info!("received command: {} and text: {}", form.command, form.text);
    let content = if form.text.is_empty(){
        read_mail(&form.text).await
    }else{
        "".to_string()
    };
    info!("Content: {}", content);
    HttpResponse::Ok()
        .body(format!("Content: {}", content))
}
