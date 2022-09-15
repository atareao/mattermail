use async_native_tls::TlsConnector;
//use futures_util::
use futures::TryStreamExt;
use async_imap;
use imap_proto::types::Address;
use alloc::borrow::Cow;
use std::str::from_utf8;

pub async fn get_unread_mails(server: &str, port: u16, user: &str, password: &str) -> Vec<String>{
    let mut result:Vec<String> = Vec::new();
    let tls = TlsConnector::new();
    let client = async_imap::connect( (server, port), server, tls).await.unwrap();
    let mut imap_session = client
        .login(user, password)
        .await
        .map_err(|e| e.0).unwrap();
    imap_session.select("INBOX").await.unwrap();
    let new_items = imap_session.search("NOT SEEN").await.unwrap();
    println!("===============");
    for item in new_items {
        println!("{}", item);
        let messages_stream = imap_session.fetch(item.to_string(), "RFC822").await.unwrap();
        let messages: Vec<_> = messages_stream.try_collect().await.unwrap();
        for message in messages {
            let envelope = message.envelope().unwrap();
            let subject = unsafe {std::str::from_utf8_unchecked(&envelope.subject.as_ref().unwrap())};
            let from = &envelope.from;
            let body = message.body().unwrap();
            result.push(std::str::from_utf8(body).unwrap().to_string());
        }
    }
    println!("===============");
    result
}

fn get_senders(addresses: Vec<Address>) -> String{
    let mut result: Vec<String> = Vec::new();
    for address in addresses{
        let name = get_from_cow(c).name

    }

}

fn get_from_cow(data: Option<Cow<[u8]>>) -> String{
    from_utf8(data.as_ref().unwrap())
}
