use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::mpsc::Sender;
use tokio::sync::{mpsc, Mutex};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

use crate::worker::ContainerConfig;

mod docker;
mod github;
mod router;
mod verify;
mod worker;

pub struct WorkerConfig {
    pub docker_username: String,
    pub docker_password: String,
    pub container_name: String,
    pub image_name: String,
    pub dockerfile_path: PathBuf,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or("github_webhook_handler=debug,axum=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .unwrap();

    let worker_config = Arc::new(Mutex::new(WorkerConfig {
        docker_username: std::env::var("DOCKER_USERNAME")
            .expect("Expect the env variable DOCKER_USERNAME"),
        docker_password: std::env::var("DOCKER_PASSWORD")
            .expect("Expect the env variable DOCKER_PASSWORD"),
        container_name: std::env::var("CONTAINER_NAME")
            .expect("Expect the env variable CONTAINER_NAME"),
        image_name: std::env::var("IMAGE_NAME").expect("Expect the env variable IMAGE_NAME"),
        dockerfile_path: std::env::var("DOCKERFILE_PATH")
            .map(|item| item.into())
            .expect("Expect the env variable DOCKERFILE_PATH"),
    }));

    let token = std::env::var("GITHUB_TOKEN").expect("Expect the env variable GITHUB_TOKEN");

    // ? 16 shutdown/terminate signals should be way more than enough
    let (shutdown_tx, shutdown_rx) = mpsc::channel(0x10);

    // ? 255 messages should be way more than enough
    // TODO maybe only the newest message should be processed
    // So either the queue has the len 0 or 1.
    // If a new event is received and one is already on the receiver side than the stored one should be switched with the new event
    let (event_tx, event_rx) = mpsc::channel(0xFF);

    tracing::info!("Spawn worker");
    let handle_worker = tokio::spawn({
        let worker_config = worker_config.clone();
        let container_config = ContainerConfig::builder()
            .port_mapping("0.0.0.0:8080", "80")
            .unwrap()
            .port_mapping("0.0.0.0:80", "80")
            .unwrap()
            .port_mapping("0.0.0.0:443", "80")
            .unwrap()
            .build();

        async move { worker::start(event_rx, shutdown_rx, worker_config, container_config).await }
    });

    let handle_server = tokio::spawn(async move {
        tracing::info!("Starting server at port 3000");
        let app = crate::router::router(token, event_tx);
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

        axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal(shutdown_tx))
            .await
    });

    tokio::select! {
        status_server = handle_server => { status_server.unwrap().unwrap(); },
        status_worker = handle_worker => { status_worker.unwrap().unwrap(); },
    }
}

async fn shutdown_signal(shutdown_sender: Sender<()>) {
    // Modified copied code from https://github.com/tokio-rs/axum/blob/2ec68d6c4dab10b83b9195c3acd4ccc7c26d0e8a/examples/graceful-shutdown/src/main.rs

    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    shutdown_sender.send(()).await.unwrap();
}
