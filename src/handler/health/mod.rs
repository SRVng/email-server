use axum::response;
use dotenv::var;
use serde_json::{json, Value};

pub async fn health_handler() -> response::Json<Value> {
    let origin: String = var("ORIGIN").unwrap_or("".to_string());
    response::Json(json!({ "message": "Hello World!", "origin": origin }))
}
