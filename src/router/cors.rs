use axum::http::{HeaderValue, Method};
use tower_http::cors::CorsLayer;

fn get_allowed_origin() -> Result<Vec<HeaderValue>, dotenv::Error> {
    let allow_origin = dotenv::var("ALLOW_ORIGIN")?;

    Ok(allow_origin
        .split(',')
        .map(|origin| {
            origin
                .to_string()
                .parse::<HeaderValue>()
                .expect("Failed to parse header value")
        })
        .collect::<Vec<HeaderValue>>())
}

pub fn get_cors_layers() -> CorsLayer {
    let allowed_origins: Vec<HeaderValue> =
        get_allowed_origin().expect("Faield to get allowed origin");
    let allowed_methods: Vec<Method> = vec![Method::GET, Method::POST];

    CorsLayer::new()
        .allow_methods(allowed_methods)
        .allow_origin(allowed_origins)
}
