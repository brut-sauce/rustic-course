mod error;
mod model;
mod utils;
mod dao;
mod controller;

use controller::router;
use lambda_http::{http::{self}, service_fn, Body,IntoResponse, Request, Response};

#[tokio::main]
async fn main() -> Result<(), lambda_http::Error> {
    //cloudwatch logging configuration
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    lambda_http::run(service_fn(handle)).await?;
    Ok(())
}

// Handle incoming requests
async fn handle(req: Request) -> Result<impl IntoResponse, lambda_http::Error>{
    // Route the request to the appropriate controller
    let response = router(req).await.unwrap_or_else(|e |e.into());

    let res = Response::builder()
    .status(http::StatusCode::from_u16(response.status_code).expect("Status Code is not valid"))
    .body(Body::from(response.body.to_string())).expect("Response Body is not valid");

    Ok(res)
}