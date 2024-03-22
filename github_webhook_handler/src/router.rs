use axum::body::Bytes;
use axum::http::{HeaderMap, Method, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde_json::json;
use tokio::sync::mpsc::Sender;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::github::GitHubWebhook;

pub fn router(secret_token: String, payload_sender: Sender<GitHubWebhook>) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/",
            post(|headers: HeaderMap, body: Bytes| async move {
                tracing::info!("Received webhook");

                let signature = headers
                    .get("x-hub-signature-256")
                    .expect("Missing secret key header")
                    .to_str()
                    .unwrap()
                    .strip_prefix("sha256=")
                    .unwrap();

                if crate::verify::call(
                    secret_token.as_bytes(),
                    &hex::decode(signature).unwrap(),
                    &body,
                )
                .is_err()
                {
                    tracing::error!("Couldn't verify that the message came from github.com");

                    return (StatusCode::UNAUTHORIZED, "Unauthorized");
                }

                let payload = if let Ok(payload) = serde_json::from_slice::<GitHubWebhook>(&body) {
                    payload
                } else {
                    tracing::error!("Couldn't parse request body as struct 'GitHubWebhook'");

                    return (StatusCode::BAD_REQUEST, "Bad Request");
                };

                payload_sender
                    .send(payload)
                    .await
                    .expect("Couldn't send the webhook body to the channel");

                tracing::info!("Return status code");
                (StatusCode::ACCEPTED, "Accepted")
            }),
        )
        .fallback(|| async {
            let status_code = StatusCode::NOT_FOUND;

            (
                status_code,
                Json(json!({
                    "status": status_code.as_u16(), "message": status_code.canonical_reason()
                })),
            )
        })
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(
                    CorsLayer::new()
                        .allow_methods([Method::GET, Method::POST])
                        .allow_origin(tower_http::cors::Any),
                )
                .layer(CompressionLayer::new()),
        )
}
