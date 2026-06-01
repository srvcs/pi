use axum::{response::IntoResponse, Json};
use serde::Serialize;
use serde_json::json;
use utoipa::{OpenApi, ToSchema};

/// This service's identity. `srvcs-pi` is a zero-argument constant: it depends
/// on no other service and performs no input validation. It simply yields the
/// mathematical constant pi as an `f64`.
pub const SERVICE: &str = "srvcs-pi";
pub const CONCERN: &str = "constant: pi";
pub const DEPENDS_ON: &[&str] = &[];

#[derive(Serialize, ToSchema)]
pub struct Info {
    pub service: &'static str,
    pub concern: &'static str,
    pub depends_on: Vec<&'static str>,
}

/// `GET /` — service identity (srvcs service standard).
#[utoipa::path(get, path = "/", responses((status = 200, body = Info)))]
pub async fn index() -> Json<Info> {
    Json(Info {
        service: SERVICE,
        concern: CONCERN,
        depends_on: DEPENDS_ON.to_vec(),
    })
}

#[derive(Serialize, ToSchema)]
pub struct ConstantResponse {
    /// The value of pi.
    pub result: f64,
}

/// The single concern: the mathematical constant pi.
pub fn pi() -> f64 {
    std::f64::consts::PI
}

/// `POST /` — return the constant pi. The request body is ignored; it may be
/// empty, absent, or any JSON value.
#[utoipa::path(
    post,
    path = "/",
    responses((status = 200, body = ConstantResponse))
)]
pub async fn evaluate() -> impl IntoResponse {
    Json(json!({ "result": pi() }))
}

#[derive(OpenApi)]
#[openapi(paths(index, evaluate), components(schemas(Info, ConstantResponse)))]
pub struct ApiDoc;

/// Serve OpenAPI document
pub async fn openapi_json() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx(got: f64, expected: f64) -> bool {
        (got - expected).abs() < 1e-9
    }

    #[test]
    fn openapi_documents_routes() {
        let doc = ApiDoc::openapi();
        let root = doc.paths.paths.get("/").expect("path / present");
        assert!(root.get.is_some(), "GET / documented");
        assert!(root.post.is_some(), "POST / documented");
    }

    #[test]
    fn pi_is_correct() {
        // The spec-asserted value, parsed from its decimal text so the test
        // is independent of `std::f64::consts::PI` (and does not trip
        // `clippy::approx_constant`).
        let expected: f64 = "3.141592653589793".parse().unwrap();
        assert!(approx(pi(), expected));
    }

    #[test]
    fn pi_relations_hold() {
        // Sanity checks derived from the same constant.
        assert!(approx(pi() / 2.0, std::f64::consts::FRAC_PI_2));
        assert!(approx(2.0 * pi(), std::f64::consts::TAU));
        assert!(approx(pi().cos(), -1.0));
        assert!(approx(pi().sin(), 0.0));
    }

    #[tokio::test]
    async fn index_reports_identity() {
        let Json(info) = index().await;
        assert_eq!(info.service, "srvcs-pi");
        assert_eq!(info.concern, "constant: pi");
        assert!(info.depends_on.is_empty());
    }
}
