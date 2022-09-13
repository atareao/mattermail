
use actix_web::{App, HttpServer};

mod routes;
mod mail;

use routes::{root, status, hook};
use crate::mail::get_unread_mails;
use std::{thread, time};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut stop: &'static bool = false;
    let duration = &time::Duration::from_secs_f32(300.0);
    let handle = thread::spawn(|| async {
        while !stop{
            let mails = get_unread_mails("imap.atareao.es", 995, "atareao@atareao.es", "secreto").await;
            for mail in mails{
                println!("{}", mail);
            }
            thread::sleep(*duration);
        }
    });

    stop = true;
    HttpServer::new(move ||{
        App::new()
            .service(root)
            .service(status)
            .service(hook)
    })
        .bind("0.0.0.0:8080")
        .unwrap()
        .run()
        .await

}
