use axum::{
    extract, response::IntoResponse,
    Json,
};
use tracing::info;

use crate::service;

pub async fn health(
) -> impl IntoResponse {
    info!("healthy");
    Json("healthy")
}

#[derive(serde::Deserialize, Debug)]
pub struct Add {
    a: i32,
    b: i32,
}

pub async fn sum(
    add: extract::Json<Add>,
) -> impl IntoResponse {
    info!("add: {:?}", add);

    let result =
        service::add(add.a, add.b);

    Json(format!(
        "The sum of {} and {} is {}",
        add.a, add.b, result
    ))
}
