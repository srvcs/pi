use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use srvcs_pi::{health, router, telemetry};
use tower::ServiceExt;

async fn status_of(uri: &str) -> StatusCode {
    let app = router(telemetry::metrics_handle_for_tests());
    app.oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap()
        .status()
}

/// POST a raw body to `/` and return (status, parsed JSON response).
async fn post_raw(content_type: Option<&str>, body: Body) -> (StatusCode, serde_json::Value) {
    let app = router(telemetry::metrics_handle_for_tests());
    let mut builder = Request::builder().method("POST").uri("/");
    if let Some(ct) = content_type {
        builder = builder.header("content-type", ct);
    }
    let res = app.oneshot(builder.body(body).unwrap()).await.unwrap();
    let status = res.status();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let json = serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null);
    (status, json)
}

/// POST a JSON body to `/`.
async fn post_json(body: serde_json::Value) -> (StatusCode, serde_json::Value) {
    post_raw(Some("application/json"), Body::from(body.to_string())).await
}

fn approx(got: f64, expected: f64) -> bool {
    (got - expected).abs() < 1e-9
}

/// The spec-asserted value of pi, parsed from its decimal text so it is
/// independent of `std::f64::consts::PI` (and does not trip
/// `clippy::approx_constant`).
fn expected_pi() -> f64 {
    "3.141592653589793".parse().unwrap()
}

#[tokio::test]
async fn index_ok() {
    assert_eq!(status_of("/").await, StatusCode::OK);
}

#[tokio::test]
async fn index_reports_identity() {
    let app = router(telemetry::metrics_handle_for_tests());
    let res = app
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["service"], "srvcs-pi");
    assert_eq!(body["concern"], "constant: pi");
    assert_eq!(body["depends_on"], serde_json::json!([]));
}

#[tokio::test]
async fn healthz_ok() {
    assert_eq!(status_of("/healthz").await, StatusCode::OK);
}

#[tokio::test]
async fn readyz_reflects_state() {
    health::set_ready(true);
    assert_eq!(status_of("/readyz").await, StatusCode::OK);
}

#[tokio::test]
async fn metrics_ok() {
    assert_eq!(status_of("/metrics").await, StatusCode::OK);
}

#[tokio::test]
async fn openapi_ok() {
    assert_eq!(status_of("/openapi.json").await, StatusCode::OK);
}

#[tokio::test]
async fn post_empty_object_returns_pi() {
    let (status, body) = post_json(serde_json::json!({})).await;
    assert_eq!(status, StatusCode::OK);
    let result = body["result"].as_f64().expect("result is an f64");
    assert!(approx(result, expected_pi()), "expected pi, got {result}");
}

#[tokio::test]
async fn post_ignores_arbitrary_body() {
    // A constant service ignores any provided fields and still returns pi.
    let (status, body) = post_json(serde_json::json!({ "a": 1, "b": "noise", "c": [1, 2] })).await;
    assert_eq!(status, StatusCode::OK);
    let result = body["result"].as_f64().expect("result is an f64");
    assert!(approx(result, expected_pi()));
}

#[tokio::test]
async fn post_no_body_returns_pi() {
    // No Content-Type / no body at all: the body extractor is optional.
    let (status, body) = post_raw(None, Body::empty()).await;
    assert_eq!(status, StatusCode::OK);
    let result = body["result"].as_f64().expect("result is an f64");
    assert!(approx(result, expected_pi()));
}

#[tokio::test]
async fn generates_request_id_when_absent() {
    let app = router(telemetry::metrics_handle_for_tests());
    let res = app
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert!(
        res.headers().contains_key("x-request-id"),
        "response must carry a generated x-request-id"
    );
}
