mod routes;
mod mail;
mod mattermost;

use crate::mail::get_unread_mails;
use std::{thread, time};
use dotenv::dotenv;
use mattermost::Mattermost;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pull_time: u64 = env::var("PULL_TIME").unwrap_or("300".to_string())
        .parse()
        .unwrap();
    let imap_host = env::var("HOST").expect("HOST not set");
    let imap_port: u16 = env::var("PORT").expect("PORT not set")
        .parse()
        .unwrap();
    let imap_login = env::var("LOGIN").expect("LOGIN not set");
    let imap_passwd = env::var("PASSWD").expect("PASSWD not set");
    let base_uri = env::var("MATTERMOST_BASE_URI").expect("MATTERMOST_BASE_URI not set");
    let token = env::var("MATTERMOST_ACCESS_TOKEN").expect("MATTERMOST_ACCESS_TOKEN not set");
    let duration = time::Duration::from_secs(pull_time); // 5 minutes
    let mm = Mattermost::new(&base_uri, &token);
    let channel_id = mm.get_channel_by_name("correo").await.unwrap();
    loop {
        let mails = get_unread_mails(&imap_host, imap_port, &imap_login, &imap_passwd).await;
        for mail in mails{
            println!("{}", mail);
            mm.post_message(&channel_id, &mail.to_string(), None).await.unwrap();
        }
        thread::sleep(duration);
    }
}
