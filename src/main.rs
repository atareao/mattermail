mod routes;
mod mail;

use crate::mail::get_unread_mails;
use std::{thread, time};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let imap_host = env::var("HOST").expect("HOST not set");
    let imap_port: u16 = env::var("PORT").expect("PORT not set")
        .parse()
        .unwrap();
    let imap_login = env::var("LOGIN").expect("LOGIN not set");
    let imap_passwd = env::var("PASSWD").expect("PASSWD not set");
    let mut stop = false;
    let duration = time::Duration::from_secs_f32(300.0); // 5 minutes
    while !stop{
        let mails = get_unread_mails(&imap_host, imap_port, &imap_login, &imap_passwd).await;
        for mail in mails{
            println!("{}", mail);
        }
        thread::sleep(duration);
    }
    stop = true;
    Ok(())
}
