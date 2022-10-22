use actix_web::body::MessageBody;
use async_native_tls::TlsConnector;
//use futures_util::
use futures::TryStreamExt;
use async_imap;
use std::fmt::Display;
use html2md::parse_html;
use std::fmt;
use email_parser::{email::Email, address::Mailbox};

pub struct Mail{
    from: String,
    subject: String,
    content: String,
}

impl Mail {
    pub fn new(message: &Email) -> Self{
        let from = get_addresses(&message.from);
        let subject = message.subject.as_ref().unwrap().to_string();
        let content = parse_html(&message.body.as_ref().unwrap().to_string());
        Self{
            from,
            subject,
            content,
        }

    }
}

impl Display for Mail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "From: {}\nSubject: {}\nContent: {}",
            self.from, self.subject, self.content)
    }
}

pub async fn get_unread_mails(server: &str, port: u16, user: &str, password: &str) -> Vec<Mail>{
    let mut result:Vec<Mail> = Vec::new();
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
        let body = message.body().unwrap();
        match Email::parse(body) {
            Ok(content) => result.push(Mail::new(&content)),
            Err(e) => println!("{:?}", e),
        }
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

fn get_addresses(mailboxes: &Vec<Mailbox>) -> String{
    let mut addresses = Vec::new();
    for mailbox in mailboxes{
        addresses.push(get_address(mailbox));
    }
    addresses.join(", ")
}

fn get_address(mailbox: &Mailbox) -> String{
    let user_name = match &mailbox.name{
        Some(names) => names.join(" "),
        None => "".to_string(),
    };
    format!("{} <{}@{}>", user_name, mailbox.address.local_part,
        mailbox.address.domain)
}
