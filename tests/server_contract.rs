use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
};
use serde_json::{json, Value};
use tower::ServiceExt;

use rlvr_verifiers::server;

#[tokio::test]
async fn health_endpoint_reports_ready_status() {
    let response = server::app()
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response_json(response).await;
    assert_eq!(body["status"], "ok");
    assert!(body["verifiers"].as_u64().unwrap() >= 13);
}

#[tokio::test]
async fn verify_endpoint_scores_single_request() {
    let payload = json!({
        "domain": "gsm8k",
        "verifier": "math_numerical",
        "task": {"gold": "#### 42"},
        "response": "The answer is 42",
        "task_id": "gsm8k-1"
    });

    let response = server::app()
        .oneshot(post_json("/verify", payload))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response_json(response).await;
    assert_eq!(body["task_id"], "gsm8k-1");
    assert_eq!(body["domain"], "gsm8k");
    assert_eq!(body["score"], 1.0);
}

#[tokio::test]
async fn verify_batch_endpoint_preserves_order() {
    let payload = json!([
        {
            "domain": "gsm8k",
            "verifier": "math_numerical",
            "task": {"gold": "#### 42"},
            "response": "#### 42",
            "task_id": "correct"
        },
        {
            "domain": "gsm8k",
            "verifier": "math_numerical",
            "task": {"gold": "#### 42"},
            "response": "#### 41",
            "task_id": "wrong"
        }
    ]);

    let response = server::app()
        .oneshot(post_json("/verify/batch", payload))
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response_json(response).await;
    assert_eq!(body.as_array().unwrap().len(), 2);
    assert_eq!(body[0]["task_id"], "correct");
    assert_eq!(body[0]["score"], 1.0);
    assert_eq!(body[1]["task_id"], "wrong");
    assert_eq!(body[1]["score"], 0.0);
}

fn post_json(uri: &str, payload: Value) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(payload.to_string()))
        .unwrap()
}

async fn response_json(response: axum::response::Response) -> Value {
    let body = to_bytes(response.into_body(), 1024 * 1024).await.unwrap();
    serde_json::from_slice(&body).unwrap()
}
