use axum::{routing, Router};
use tracing::info;

use crate::{
    controller::{health, sum},
    service::add,
};

mod controller;
mod service;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .init();

    let a = 10;
    let b = 20;
    let result = add(a, b);

    info!(
        "The result of {} + {} = {}",
        a, b, result
    );

    let router = Router::new()
        .route(
            "/add/health",
            routing::get(health),
        )
        .route(
            "/add/sum",
            routing::get(sum),
        );

    let server = axum::Server::bind(
        &std::net::SocketAddr::from((
            [0, 0, 0, 0],
            8080,
        )),
    )
    .serve(router.into_make_service());

    tokio::select! {
        _ = server => info!("axum server exited"),
    }
}
