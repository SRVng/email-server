use axum::extract::Json;
use axum::http::StatusCode;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use serde_json::{json, Value};

use crate::helper::email::{get_creds, get_message, IEmailBody};

pub async fn send_email_handler(Json(body): Json<IEmailBody>) -> (StatusCode, Json<Value>) {
    let email: Message = get_message(body);

    let creds: Credentials = get_creds();

    // Open a remote connection to gmail
    let mailer: SmtpTransport = SmtpTransport::relay("smtp.gmail.com")
        .expect("Failed to establish TLS")
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => (StatusCode::OK, Json(json!(None::<String>))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        ),
    }
}
