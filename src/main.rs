use dotenv::dotenv;
use helper::get_address::get_address;
use lambda_web::{is_running_on_lambda, run_hyper_on_lambda, LambdaError};

mod handler;
mod helper;
mod router;

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    match is_running_on_lambda() {
        true => run_hyper_on_lambda(router::app_router()).await?,
        false => {
            if let Err(error) = axum::Server::bind(&get_address())
                .serve(router::app_router().into_make_service())
                .await
            {
                eprintln!("{}", error)
            }
        }
    }

    Ok(())
}
