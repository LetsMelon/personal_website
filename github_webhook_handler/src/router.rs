use axum::body::Bytes;
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
use axum::Router;
use tokio::sync::mpsc::Sender;

use crate::github::GitHubWebhook;

pub fn router(secret_token: String, payload_sender: Sender<GitHubWebhook>) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route(
            "/webhook",
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
}
