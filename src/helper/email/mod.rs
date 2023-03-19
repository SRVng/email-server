use dotenv::var;
use lettre::{message::header::ContentType, transport::smtp::authentication::Credentials, Message};
use uuid::Uuid;

// * Use unwrap directly, env already checked at initialize
pub fn get_message(body: String) -> Message {
    let from = var("EMAIL_FROM").unwrap();
    let to = var("EMAIL_TO").unwrap();
    let fixed_subject = var("EMAIL_FIXED_SUBJECT").unwrap();
    let subject = format!("{fixed_subject} {}", Uuid::new_v4());
    let header = ContentType::TEXT_HTML;

    Message::builder()
        .from(from.parse().expect("Failed to parse Message::from"))
        .to(to.parse().expect("Failed to parse Message::to"))
        .subject(subject)
        .header(header)
        .body(body)
        .expect("Failed to create email")
}

pub fn get_creds() -> Credentials {
    let username = var("SMTP_USERNAME").unwrap();
    let password = var("SMTP_PASSWORD").unwrap();

    Credentials::new(username, password)
}
