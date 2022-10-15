use async_native_tls::TlsConnector;
//use futures_util::
use futures::TryStreamExt;
use async_imap;
use imap_proto::types::Address;
use std::str::from_utf8;
use std::borrow::Cow;
use html2md::parse_html;

pub async fn get_unread_mails(server: &str, port: u16, user: &str, password: &str) -> Vec<String>{
    let mut result:Vec<String> = Vec::new();
    let tls = TlsConnector::new();
    let client = async_imap::connect( (server, port), server, tls).await.unwrap();
    let mut imap_session = client
        .login(user, password)
        .await
        .map_err(|e| e.0).unwrap();
    imap_session.select("INBOX").await.unwrap();
    let mut new_items = imap_session.search("NOT SEEN").await.unwrap();
    println!("===============");
    let first = new_items.drain().next().unwrap().to_string();
    let messages_stream = imap_session.fetch(first, "RFC822").await.unwrap();
    let messages: Vec<_> = messages_stream.try_collect().await.unwrap();
    let mut counter = 0;
    for message in messages {
        //let envelope = message.envelope().unwrap();
        //let subject = get_from_cow(&envelope.subject);
        ////let subject = unsafe {std::str::from_utf8_unchecked(&envelope.subject.as_ref().unwrap())};
        //let from = &envelope.from;
        let body = message.body().unwrap();
        let content =parse_html(&std::str::from_utf8(body).unwrap().to_string());
        println!("{} --- {}", counter, content);
        result.push(content);
    }
    /*
    for item in new_items {
        println!("{}", item);
        let messages_stream = imap_session.fetch(item.to_string(), "RFC822").await.unwrap();
        let messages: Vec<_> = messages_stream.try_collect().await.unwrap();
        for message in messages {
            //let envelope = message.envelope().unwrap();
            //let subject = get_from_cow(&envelope.subject);
            ////let subject = unsafe {std::str::from_utf8_unchecked(&envelope.subject.as_ref().unwrap())};
            //let from = &envelope.from;
            let body = message.body().unwrap();
            result.push(parse_html(&std::str::from_utf8(body).unwrap().to_string()));
        }
    }
        */
    println!("===============");
    result
}

fn get_senders(addresses: Vec<Address>) -> Vec<String>{
    let mut result: Vec<String> = Vec::new();
    for address in addresses{
        let name = get_from_cow(&address.name);
        let mail = get_from_cow(&address.mailbox);
        result.push(format!("{} <{}>", name, mail))
    }
    result
}

fn get_from_cow(data: &Option<Cow<[u8]>>) -> String{
    from_utf8(data.as_ref().unwrap()).unwrap().to_string()
}
