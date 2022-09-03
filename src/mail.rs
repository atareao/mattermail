use async_native_tls::TlsConnector;
use async_imap;

pub struct ImapConfiguration{
    server: String,
    port: i32,
}

impl ImapConfiguration{
    pub fn new(server: &str, port: i32) -> Self{
        Self{
            server: server.to_string(),
            port,
        }
    }
}
pub struct Credentials{
    user: String,
    password: String,
}

impl Credentials{
    pub fn new(user: &str, password: &str) -> Self{
        Self{
            user: user.to_string(),
            password: password.to_string(),
        }
    }
}

pub async fn get_unread_mails(imap_configuration: ImapConfiguration, credentials: Credentials) -> imap::error::Result<Option<String>>{
    let tls = TlsConnector::new();
    let client = async_imap::connect((imap_configuration.server, imap_configuration.port), imap_configuration.server, tls).await?;
    let mut imap_session = client
        .login(credentials.user, credentials.password)
        .await
        .map_err(|e| e.0)?;
    imap_session.select("INBOX").await?;
    let messages_stream = imap_session.fetch(sequence_set, query)


}
