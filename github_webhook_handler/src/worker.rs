use std::collections::HashMap;
use std::sync::Arc;

use anyhow::{bail, Context};
use bollard::container::{Config, CreateContainerOptions, RemoveContainerOptions};
use bollard::errors::Error;
use bollard::secret::{HostConfig, PortBinding};
use bollard::Docker;
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::Receiver;
use tokio::sync::Mutex;
use tracing::*;

use crate::docker::{build_image, get_container_by_name, stop_container, ContainerState};
use crate::github::{GitHubWebhook, RefType};
use crate::WorkerConfig;

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

#[instrument(name = "worker", skip_all)]
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
        let payload = receive_events
            .recv()
            .await
            .context("Couldn't receive event from Receiver")?;

        let config = config.lock().await;

        if matches!(payload.ref_type, RefType::Tag) {
            info!("Received payload");
            debug!("payload = {:?}", payload);

            info!(
                "Check if a container from the image {:?} is on the system",
                &config.image_name
            );
            // TODO should this be `config.image_name`?
            if let Some(container) =
                get_container_by_name(&docker_connection, &config.image_name).await?
            {
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

                debug!("Container has name(s): {:?}", container.names);
                let container_name = container
                    .names
                    .map(|names| names.get(0).cloned())
                    .flatten()
                    .with_context(|| {
                        format!(
                            "The container {:?} doesn't have a specific name",
                            container
                                .id
                                .as_ref()
                                .unwrap_or(&"UNKNOWN_CONTAINER_ID".to_string())
                        )
                    })?;

                if let Some(state) = container.state.as_ref() {
                    debug!("container state: {state}");

                    let container_id = container
                        .id
                        .as_ref()
                        .context("Container id is not allowed to be 'None'")?;

                    let container_state = ContainerState::from_str(&state);
                    debug!("Container state: {:?}", container_state);

                    match container_state {
                        Ok(
                            ContainerState::Running
                            | ContainerState::Paused
                            | ContainerState::Restarting,
                        ) => stop_container(&docker_connection, &container_name, Some(10)).await?,
                        Ok(_) => (),
                        Err(unknown_state) => {
                            bail!(
                                "Unknown container state {:?} for {}",
                                unknown_state,
                                container_id
                            )
                        }
                    };
                }

                // TODO delete container if it is still on the system
                debug!("Try to delete the container from the system");
                match docker_connection
                    .remove_container(
                        &container_name,
                        Some(RemoveContainerOptions {
                            force: true,
                            ..Default::default()
                        }),
                    )
                    .await
                {
                    Ok(_) => debug!("deleted container successfully"),
                    Err(err) => {
                        error!(
                            "encountered an error while deleting the container, err = {:?}",
                            err
                        );

                        // TODO find out which error is being thrown if there is no container to delete
                        match err {
                            Error::NoHomePathError => todo!("Error::NoHomePathError"),
                            Error::CertPathError { path } => todo!("Error::CertPathError"),
                            Error::CertMultipleKeys { count, path } => {
                                todo!("Error::CertMultipleKeys")
                            }
                            Error::CertParseError { path } => todo!("Error::CertParseError"),
                            Error::NoNativeCertsError { err } => todo!("Error::NoNativeCertsError"),
                            Error::DockerResponseServerError {
                                status_code,
                                message,
                            } => todo!("Error::DockerResponseServerError"),
                            Error::JsonDataError { message, column } => {
                                todo!("Error::JsonDataError")
                            }
                            Error::APIVersionParseError {} => todo!("Error::APIVersionParseError"),
                            Error::RequestTimeoutError => todo!("Error::RequestTimeoutError"),
                            Error::DockerStreamError { error } => todo!("Error::DockerStreamError"),
                            Error::DockerContainerWaitError { error, code } => {
                                todo!("Error::DockerContainerWaitError")
                            }
                            Error::MissingSessionBuildkitError {} => {
                                todo!("Error::MissingSessionBuildkitError")
                            }
                            Error::MissingVersionBuildkitError {} => {
                                todo!("Error::MissingVersionBuildkitError")
                            }
                            Error::JsonSerdeError { err } => todo!("Error::JsonSerdeError"),
                            Error::StrParseError { err } => todo!("Error::StrParseError"),
                            Error::IOError { err } => todo!("Error::IOError"),
                            Error::StrFmtError { err } => todo!("Error::StrFmtError"),
                            Error::HttpClientError { err } => todo!("Error::HttpClientError"),
                            Error::HyperResponseError { err } => todo!("Error::HyperResponseError"),
                            Error::URLEncodedError { err } => todo!("Error::URLEncodedError"),
                            Error::URLParseError { err } => todo!("Error::URLParseError"),
                            Error::InvalidURIError { err } => todo!("Error::InvalidURIError"),
                            Error::HyperLegacyError { err } => todo!("Error::HyperLegacyError"),
                            Error::UnsupportedURISchemeError { uri } => {
                                todo!("Error::UnsupportedURISchemeError")
                            }
                        }
                    }
                }
            };

            info!("Build new image");

            debug!(
                "Read Dockerfile from fs at path {:?}",
                config.dockerfile_path
            );
            let dockerfile = tokio::fs::File::open(config.dockerfile_path.as_path()).await?;

            let mut buildargs = HashMap::new();
            buildargs.insert("WEBSITE_TAG", payload.ref_name.as_str());

            let tag = format!("{}:latest", config.image_name); // TODO maybe add the tag version

            build_image(
                &docker_connection,
                &config.docker_username,
                &config.docker_password,
                &tag,
                &config.container_name,
                buildargs,
                dockerfile,
            )
            .await?;

            info!("Create container");
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
