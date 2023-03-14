use axum::{routing, Router, Server};
use dotenv::dotenv;
use helper::get_address::get_address;

mod handler;
mod helper;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let public_router: Router =
        Router::new().route("/health", routing::get(handler::health::health_handler));

    let app_router: Router = Router::new().nest("/api", public_router);

    if let Err(error) = Server::bind(&get_address())
        .serve(app_router.into_make_service())
        .await
    {
        eprintln!("{}", error);
    };
}
