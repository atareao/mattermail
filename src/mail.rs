use async_native_tls::TlsConnector;
//use futures_util::
use futures::TryStreamExt;
use async_imap;
use std::fmt::Display;
use html2md::parse_html;
use std::fmt;
use mail_parser::{Message, HeaderValue, Addr, Group};

pub struct Mail{
    id: u32,
    reg: String,
    from: String,
    subject: String,
}

impl Mail {
    pub fn new(id: u32, content: &Message) -> Self{
        let reg = content.get_message_id().unwrap().to_string();
        let from = get_address(&content.get_from());
        let subject = content.get_subject().unwrap_or("").to_string();
        Self{
            id,
            reg,
            from,
            subject,
        }
    }
}

impl Display for Mail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Id: {}\nReg: {}\nFrom: {}\nSubject: {}",
            self.id, self.reg, self.from, self.subject)
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
    match new_items.drain().next(){
        Some(identificador) => {
            let messages_stream = imap_session.fetch(identificador.to_string(), "RFC822").await.unwrap();
            let messages: Vec<_> = messages_stream.try_collect().await.unwrap();
            for message in messages {
                let id = message.message;
                let body = message.body().unwrap();
                match Message::parse(body){
                    Some(content) => result.push(Mail::new(id, &content)),
                    None => println!("{:?}", body),
                }
            }
        },
        None => {}
    }
    result
}

fn get_from_groups(groups: &Vec<Group>) -> String{
    let mut result = Vec::new();
    for group in groups{
        result.push(get_from_addresses(&group.addresses));
    }
    result.join(", ")
}

fn get_from_addresses(addresses: &Vec<Addr>) -> String{
    let mut result = Vec::new();
    for address in addresses{
        result.push(get_from_address(address))
    }
    result.join(", ")
}
fn get_from_address(address: &Addr) -> String{
    let name = match &address.name{
        Some(name) => name.to_string(),
        None => "".to_string(),
    };
    let mail = match &address.address{
        Some(mail) => mail.to_string(),
        None => "".to_string(),
    };
    format!("{} <{}>", name, mail)
}

fn get_address(header: &HeaderValue) -> String{
    match header {
        HeaderValue::Address(address) => get_from_address(address),
        HeaderValue::AddressList(addresses) => get_from_addresses(addresses),
        HeaderValue::Text(text) => text.to_string(),
        HeaderValue::TextList(textlist) => textlist.join(", "),
        HeaderValue::Group(group) => get_from_addresses(&group.addresses),
        HeaderValue::GroupList(groups) => get_from_groups(groups),
        _ => "".to_string(),
    }
}
