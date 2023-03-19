use dotenv::var;
use lettre::{
    message::{
        header::{ContentDisposition, ContentId, ContentType},
        MultiPart, SinglePart,
    },
    transport::smtp::authentication::Credentials,
    Message,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct IEmailBody {
    pub html: String,
    pub cid_body: Option<String>,
}

impl IEmailBody {
    // Html
    fn get_body(&self) -> SinglePart {
        SinglePart::builder()
            .header(ContentType::TEXT_HTML)
            .body(self.html.clone())
    }

    // Base64 Image
    fn attach_cid(&self) -> Option<SinglePart> {
        if let Some(img) = self.cid_body.clone() {
            let metadata = img.split(',').collect::<Vec<&str>>();

            let decoded_base64 =
                openssl::base64::decode_block(metadata.get(1).expect("No base64 body found"))
                    .expect("Failed to decode image");

            Some(
                SinglePart::builder()
                    .header(
                        ContentType::parse("image/png").expect("Failed to parse image/png header"),
                    )
                    .header(ContentDisposition::inline())
                    .header(ContentId::from(var("CID").unwrap()))
                    .body(decoded_base64),
            )
        } else {
            None
        }
    }

    pub fn get_multipart_body(&self) -> MultiPart {
        let multipart = MultiPart::related().singlepart(self.get_body());

        if let Some(attachment) = self.attach_cid() {
            multipart.singlepart(attachment)
        } else {
            multipart
        }
    }
}

// * Use unwrap directly, env already checked at initialize
pub fn get_message(params: IEmailBody) -> Message {
    let from = var("EMAIL_FROM").unwrap();
    let to = var("EMAIL_TO").unwrap();
    let fixed_subject = var("EMAIL_FIXED_SUBJECT").unwrap();
    let subject = format!("{fixed_subject} {}", Uuid::new_v4());

    Message::builder()
        .from(from.parse().expect("Failed to parse Message::from"))
        .to(to.parse().expect("Failed to parse Message::to"))
        .subject(subject)
        .multipart(params.get_multipart_body())
        .expect("Failed to create email")
}

pub fn get_creds() -> Credentials {
    let username = var("SMTP_USERNAME").unwrap();
    let password = var("SMTP_PASSWORD").unwrap();

    Credentials::new(username, password)
}
