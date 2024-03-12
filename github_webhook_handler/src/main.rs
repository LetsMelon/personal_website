use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::{mpsc, Mutex};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

use crate::worker::ContainerConfig;

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

    // TODO handle ctl-c

    // ? 255 messages should be way more than enough
    let (tx, rx) = mpsc::channel(0xFF);

    tracing::info!("Spawn worker");
    tokio::spawn({
        let worker_config = worker_config.clone();
        let container_config = ContainerConfig::builder()
            .port_mapping("0.0.0.0:8080", "80")
            .unwrap()
            .build();

        async move {
            worker::start(rx, worker_config, container_config)
                .await
                .unwrap()
        }
    });

    tracing::info!("Starting server at port 3000");
    let app = crate::router::router(token, tx);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
