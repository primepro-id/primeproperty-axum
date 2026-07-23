use lettre::message::header::MessageId;
use lettre::message::SinglePart;
use lettre::message::{header::ContentType, Mailbox, MultiPart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Address, Message, SmtpTransport, Transport};

use crate::envs::Envs;

pub struct Mail {}

impl Mail {
    fn build_transport() -> Result<SmtpTransport, std::io::Error> {
        let smtp_host = Envs::smtp_host();
        let username = Envs::smtp_user();
        let password = Envs::smtp_pass();
        let creds = Credentials::new(username, password);

        match SmtpTransport::relay(&smtp_host) {
            Ok(transport) => Ok(transport.port(465).credentials(creds).build()),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
        }
    }

    fn build_message(
        to: &str,
        email: &str,
        subject: &str,
        body: &str,
    ) -> Result<Message, std::io::Error> {
        let from = Envs::smtp_from();
        let from_address = match from.parse::<Address>() {
            Ok(addr) => addr,
            Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
        };

        let to_address = match email.parse::<Address>() {
            Ok(addr) => addr,
            Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
        };
        let message_id = format!(
            "<{}.{}@primeproindonesia.com>",
            chrono::Utc::now().timestamp_nanos_opt().unwrap_or_default(),
            uuid::Uuid::new_v4()
        );
        let multipart = MultiPart::alternative()
            .singlepart(
                SinglePart::builder()
                    .header(ContentType::TEXT_PLAIN)
                    .body(format!(
                        "Please view this email in an HTML-compatible client.\n\n{}",
                        body
                    )),
            )
            .singlepart(
                SinglePart::builder()
                    .header(ContentType::TEXT_HTML)
                    .body(body.to_string()),
            );

        match Message::builder()
            .from(Mailbox::new(Some(from.clone()), from_address))
            .to(Mailbox::new(Some(to.to_owned()), to_address))
            .subject(subject)
            .header(MessageId::from(message_id))
            .multipart(multipart)
        {
            Ok(msg) => Ok(msg),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
        }
    }

    pub fn send(to: &str, email: &str, subject: &str, body: &str) -> Result<(), std::io::Error> {
        let transport = match Self::build_transport() {
            Ok(transport) => transport,
            Err(e) => return Err(e),
        };

        let message = match Self::build_message(to, email, subject, body) {
            Ok(msg) => msg,
            Err(e) => return Err(e),
        };

        match transport.send(&message) {
            Ok(_) => Ok(()),
            Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
        }
    }
}
