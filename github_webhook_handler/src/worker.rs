use std::collections::HashMap;
use std::io::Write;
use std::sync::Arc;

use anyhow::{bail, Context};
use bollard::auth::DockerCredentials;
use bollard::container::{
    Config, CreateContainerOptions, ListContainersOptions, StopContainerOptions,
};
use bollard::image::{BuildImageOptions, BuilderVersion};
use bollard::secret::{HostConfig, PortBinding};
use bollard::Docker;
use bytes::Bytes;
use futures_util::stream::StreamExt;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::Receiver;
use tokio::sync::Mutex;
use tracing::*;

use crate::github::{GitHubWebhook, RefType};
use crate::WorkerConfig;

fn compress_dockerfile(raw_dockerfile: &[u8]) -> anyhow::Result<Bytes> {
    let mut header = tar::Header::new_gnu();
    header.set_path("Dockerfile")?;
    header.set_size(raw_dockerfile.len() as u64);
    header.set_mode(0o755);
    header.set_cksum();
    let mut tar = tar::Builder::new(Vec::new());
    tar.append(&header, raw_dockerfile)?;

    let uncompressed = tar.into_inner()?;

    let mut c = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
    c.write_all(&uncompressed)?;

    let compressed = c.finish()?;

    Ok(compressed.into())
}

#[derive(Debug)]
pub struct ContainerConfig {
    port_mapping: Vec<(PortBinding, String)>,
}

pub struct ContainerConfigBuilder {
    inner: ContainerConfig,
}

impl ContainerConfig {
    pub fn builder() -> ContainerConfigBuilder {
        ContainerConfigBuilder {
            inner: ContainerConfig {
                port_mapping: Vec::new(),
            },
        }
    }
}

impl ContainerConfigBuilder {
    pub fn port_mapping(
        mut self,
        host: impl Into<String>,
        container_port: impl Into<String>,
    ) -> anyhow::Result<ContainerConfigBuilder> {
        fn check_if_valid_port(port: &str) -> anyhow::Result<()> {
            port.parse::<u16>()
                .with_context(|| format!("Couldn't parse string-port as port-number: {port:?}"))?;

            Ok(())
        }

        let host = host.into();
        // TODO implement something like '80/udp'
        let container_port = container_port.into();

        // TODO implement something like '0.0.0.0:80/udp'
        let host_splitted = host
            .split(":")
            .map(|item| item.to_string())
            .collect::<Vec<String>>();
        let (host_ip, host_port) = match host_splitted.len() {
            1 => (None, host_splitted[0].clone()),
            2 => (Some(host_splitted[0].clone()), host_splitted[1].clone()),
            _ => bail!("Unknown host formatting, {host:?}"),
        };

        check_if_valid_port(host_port.as_str())?;
        check_if_valid_port(&container_port)?;

        let host_port_binding = PortBinding {
            host_ip,
            host_port: Some(host_port),
        };

        self.inner
            .port_mapping
            .push((host_port_binding, container_port));

        Ok(self)
    }

    pub fn build(self) -> ContainerConfig {
        self.inner
    }
}

#[instrument(name = "worker", skip(receive_events, receive_shutdown, config))]
pub async fn start(
    mut receive_events: Receiver<GitHubWebhook>,
    mut receive_shutdown: Receiver<()>,
    config: Arc<Mutex<WorkerConfig>>,
    container_config: ContainerConfig,
) -> anyhow::Result<()> {
    let docker_connection = Docker::connect_with_defaults()?;

    if docker_connection.ping().await.is_err() {
        bail!("Couldn't ping to docker");
    }

    loop {
        info!("Waiting for new payload to come");
        let payload: GitHubWebhook = receive_events
            .recv()
            .await
            .context("Couldn't receive event from Receiver")?;

        let config = config.lock().await;

        if matches!(payload.ref_type, RefType::Tag) {
            info!("Received payload");
            debug!("payload = {:?}", payload);

            let mut filters = HashMap::new();
            filters.insert("name", vec![config.image_name.as_str()]);

            let options = Some(ListContainersOptions {
                all: true,
                filters,
                ..Default::default()
            });

            let containers = docker_connection.list_containers(options).await?;
            debug!("got {} containers from docker", containers.len());

            // stop container if running
            if let Some(container) = containers.first() {
                if container.id.is_none() {
                    bail!("Found container's id is not allowed to be 'None'");
                }
                if let Some(image) = container.image.as_ref() {
                    if !image.starts_with(&config.image_name) {
                        bail!(
                            "Found container's where the image doesn't match with the searched \
                             container"
                        );
                    }
                } else {
                    bail!("Found container's image is not allowed to be 'None'");
                }

                if let Some(state) = container.state.as_ref() {
                    debug!("container state: {state}");

                    match state.as_str() {
                        "running" => {
                            info!(
                                "Stopping container {}",
                                container
                                    .id
                                    .as_ref()
                                    .context("Container id is not allowed to be 'None'")?
                            );

                            docker_connection
                                .stop_container(
                                    "melcher_io_website",
                                    // ? wait for 10s before killing the container
                                    Some(StopContainerOptions { t: 10 }),
                                )
                                .await?;
                        }
                        _ => info!(
                            "Unknown container state {:?} for {}",
                            state,
                            container
                                .id
                                .as_ref()
                                .context("Container id is not allowed to be 'None'")?
                        ),
                    }
                }
            };

            info!("Build new image");

            debug!("Read Dockerfile from fs");
            let dockerfile = tokio::fs::read_to_string(config.dockerfile_path.as_os_str()).await?;

            debug!("Compress Dockerfile into tar");
            let compressed = compress_dockerfile(dockerfile.as_bytes())?;

            let mut buildargs = HashMap::new();
            buildargs.insert("WEBSITE_TAG", payload.ref_name.as_str());

            let tag = format!("{}:latest", config.image_name); // TODO maybe add the tag version

            let build_image_options = BuildImageOptions {
                t: tag.as_str(),
                dockerfile: "Dockerfile",
                buildargs,
                version: BuilderVersion::BuilderBuildKit,
                pull: false,
                session: Some(config.container_name.clone()), // TODO not quit sure if that is the container or image name
                // TODO labels: maybe add the label that the image has been build via github_webhook_handler
                ..Default::default()
            };

            debug!("Send build-image-command to docker");

            // TODO find out why a login is needed?
            let credentials = {
                // TODO create a macro to generate a empty HashMap or with a key+value
                let mut map = HashMap::new();

                map.insert(
                    "registry-1.docker.io".to_string(),
                    DockerCredentials {
                        username: Some(config.docker_username.clone()),
                        password: Some(config.docker_password.clone()),
                        ..Default::default()
                    },
                );

                map
            };

            let mut image_build_stream = docker_connection.build_image(
                build_image_options,
                Some(credentials),
                Some(compressed),
            );

            debug!("Building container");
            // TODO: refactor into separate function
            while let Some(msg) = image_build_stream.next().await {
                match msg {
                    Ok(info) => {
                        trace!("msg: {:?}", info);

                        if let Some(build_error) = info.error {
                            error!("Encountered error while building image: {build_error:?}");
                            panic!("{:?}", build_error);
                        }
                    }
                    Err(err) => {
                        error!("Encountered error: {err:?}");
                        panic!("{:?}", err);
                    }
                }
            }

            info!("Create container");
            // TODO remove hardcoded values
            let exposed_ports = Some({
                let mut map = HashMap::new();

                for (_, container_port) in &container_config.port_mapping {
                    map.insert(container_port.clone(), HashMap::new());
                }

                map
            });
            let host_config = Some(HostConfig {
                port_bindings: Some({
                    let mut map = HashMap::new();

                    for (host, container_port) in &container_config.port_mapping {
                        map.insert(container_port.clone(), Some(vec![host.clone()]));
                    }

                    map
                }),
                // restart_policy: Some(RestartPolicy {
                //     name: Some(RestartPolicyNameEnum::ON_FAILURE),
                //     maximum_retry_count: Some(5),
                // }),
                auto_remove: Some(true),
                ..Default::default()
            });

            let container = docker_connection
                .create_container(
                    Some(CreateContainerOptions {
                        name: config.container_name.as_str(),
                        ..Default::default()
                    }),
                    Config {
                        image: Some(tag),
                        exposed_ports,
                        host_config,
                        ..Default::default()
                    },
                )
                .await?;

            info!("Created docker container with id {}", container.id);

            info!("Start container");
            docker_connection
                .start_container::<String>(&config.container_name, None)
                .await?;
        }

        match receive_shutdown.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => break,
            Err(TryRecvError::Empty) => continue,
        }
    }

    Ok(())
}
