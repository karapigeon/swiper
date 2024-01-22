use std::{env, process};
use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // TODO: Replace 'Hello World' route with real implementation.
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let axum_port = match env::var("BACKPACK_AXUM_PORT") {
        Ok(unwrapped_axum_port) => {
            println!("Binding Axum to port assigned in the .env: {}", unwrapped_axum_port);
            unwrapped_axum_port
        },
        Err(error) => {
            println!("Axum port was not assigned. Run source .env from root. Error: {}", error);
            process::abort();
        },
    };
    let formatted_address = format!("0.0.0.0:{}", axum_port);
    let listener = tokio::net::TcpListener::bind(formatted_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}