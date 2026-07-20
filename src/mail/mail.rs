// use lettre::message::{Mailbox, header::ContentType};
// use lettre::transport::smtp::authentication::Credentials;
// use lettre::{Message, SmtpTransport, Transport};

// fn main() {
//     let email = Message::builder()
//         .from(Mailbox::new(Some("NoBody".to_owned()), "nobody@domain.tld".parse().unwrap()))
//         .reply_to(Mailbox::new(Some("Yuin".to_owned()), "yuin@domain.tld".parse().unwrap()))
//         .to(Mailbox::new(Some("Hei".to_owned()), "hei@domain.tld".parse().unwrap()))
//         .subject("Happy new year")
//         .header(ContentType::TEXT_PLAIN)
//         .body(String::from("Be happy!"))
//         .unwrap();

//     let creds = Credentials::new("smtp_username".to_owned(), "smtp_password".to_owned());

//     // Open a remote connection to gmail
//     let mailer = SmtpTransport::relay("smtp.gmail.com")
//         .unwrap()
//         .credentials(creds)
//         .build();

//     // Send the email
//     match mailer.send(&email) {
//         Ok(_) => println!("Email sent successfully!"),
//         Err(e) => panic!("Could not send email: {e:?}"),
//     }
// }

use lettre::message::{header::ContentType, Mailbox};
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
            Ok(transport) => Ok(transport.credentials(creds).build()),
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

        match Message::builder()
            .from(Mailbox::new(Some(from.clone()), from_address))
            .to(Mailbox::new(Some(to.to_owned()), to_address))
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(String::from(body))
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
