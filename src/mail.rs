use async_native_tls::TlsConnector;
//use futures_util::
use futures::TryStreamExt;
use async_std::net::ToSocketAddrs;
use async_imap;

pub async fn get_unread_mails(server: &str, port: u16, user: &str, password: &str) -> Vec<String>{
    let mut result:Vec<String> = Vec::new();
    let tls = TlsConnector::new();
    let client = async_imap::connect( (server, port), server, tls).await.unwrap();
    let mut imap_session = client
        .login(user, password)
        .await
        .map_err(|e| e.0).unwrap();
    imap_session.select("INBOX").await.unwrap();
    let messages_stream = imap_session.fetch("1", "RFC822").await.unwrap();
    let messages: Vec<_> = messages_stream.try_collect().await.unwrap();
    for message in messages {
        let body = message.body().unwrap();
        result.push(std::str::from_utf8(body).unwrap().to_string());
    }
    result
}
