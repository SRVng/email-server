use dotenv::dotenv;

mod handler;
mod helper;
mod router;

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    dotenv().ok();

    lambda_http::run(router::app_router()).await
}
