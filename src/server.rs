//! HTTP server for verifier requests.

use std::{net::SocketAddr, time::Instant};

use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;

use crate::registry::{verify_one, VerifyRequest, VerifyResponse, SUPPORTED_VERIFIERS};

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub verifiers: usize,
}

#[derive(Debug, Serialize)]
pub struct StatsResponse {
    pub supported_verifiers: &'static [&'static str],
}

pub fn app() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/stats", get(stats))
        .route("/verify", post(verify))
        .route("/verify/batch", post(verify_batch))
}

pub async fn serve(addr: SocketAddr) -> std::io::Result<()> {
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app()).await
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        verifiers: SUPPORTED_VERIFIERS.len(),
    })
}

async fn stats() -> Json<StatsResponse> {
    Json(StatsResponse {
        supported_verifiers: SUPPORTED_VERIFIERS,
    })
}

async fn verify(Json(request): Json<VerifyRequest>) -> Json<VerifyResponse> {
    Json(verify_one(request))
}

async fn verify_batch(Json(requests): Json<Vec<VerifyRequest>>) -> Json<Vec<VerifyResponse>> {
    let started = Instant::now();
    let responses = requests.into_iter().map(verify_one).collect::<Vec<_>>();
    tracing::debug!(
        count = responses.len(),
        elapsed_ms = started.elapsed().as_millis(),
        "verified batch"
    );
    Json(responses)
}
