mod routes;
mod mail;
mod mattermost;

use actix_web::{App, HttpServer, web::Data, middleware::Logger};
use std::time;
use dotenv::dotenv;
use mattermost::Mattermost;
use std::env;
use tokio;

use env_logger::Env;
use log::{debug, error, info};

use crate::mail::get_unread_mails;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let log_level = env::var("LOG_LEVEL").unwrap_or("info".to_string());
    env_logger::init_from_env(Env::default().default_filter_or(log_level));
    let port = env::var("PORT").expect("PORT not set");
    info!("Port: {}", port);
    let pull_time: u64 = env::var("PULL_TIME").unwrap_or("300".to_string())
        .parse()
        .unwrap();
    let imap_host = env::var("IMAP_HOST").expect("HOST not set");
    let imap_port: u16 = env::var("IMAP_PORT").expect("PORT not set")
        .parse()
        .unwrap();
    let imap_login = env::var("IMAP_LOGIN").expect("LOGIN not set");
    let imap_passwd = env::var("IMAP_PASSWD").expect("PASSWD not set");
    let base_uri = env::var("MATTERMOST_BASE_URI").expect("MATTERMOST_BASE_URI not set");
    let token = env::var("MATTERMOST_ACCESS_TOKEN").expect("MATTERMOST_ACCESS_TOKEN not set");

    tokio::spawn(async move {
        let duration = time::Duration::from_secs(pull_time); // 5 minutes
        let mm = Mattermost::new(&base_uri, &token);
        let channel_id = mm.get_channel_by_name("correo").await.unwrap();
        loop {
            let mails = get_unread_mails(&imap_host, imap_port, &imap_login, &imap_passwd).await;
            for mail in mails{
                info!("{}", mail);
                mm.post_message(&channel_id, &mail.to_string(), None).await.unwrap();
            }
            tokio::time::sleep(duration).await;
        }
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(routes::root)
            .service(routes::status)
            .service(routes::hook)
    })
    .workers(2)
    .bind(format!("0.0.0.0:{}", &port))
    .unwrap()
    .run()
    .await
}
