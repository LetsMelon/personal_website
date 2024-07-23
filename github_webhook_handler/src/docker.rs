use std::collections::HashMap;
use std::fmt::Debug;
use std::io::Write;

use anyhow::bail;
use bollard::auth::DockerCredentials;
use bollard::container::{ListContainersOptions, StopContainerOptions};
use bollard::errors::Error;
use bollard::image::{BuildImageOptions, BuilderVersion};
use bollard::secret::ContainerSummary;
use bollard::Docker;
use bytes::Bytes;
use futures_util::StreamExt;
use tokio::io::AsyncReadExt;
use tracing::{debug, error, instrument, trace};

// Copied from https://stackoverflow.com/a/32428199
#[derive(Debug)]
pub enum ContainerState {
    /// A container that has been created (e.g. with `docker create`) but not started
    Created,
    /// A container that is in the process of being restarted
    Restarting,
    /// A currently running container
    Running,
    /// A container whose processes have been paused
    Paused,
    /// A container that ran and completed ("stopped" in other contexts, although a created container is technically also "stopped")
    Exited,
    /// A container that the daemon tried and failed to stop (usually due to a busy device or resource used by the container)
    Dead,
}

impl ContainerState {
    pub fn from_str(raw_status: &str) -> Result<ContainerState, &str> {
        match raw_status {
            "created" => Ok(ContainerState::Created),
            "restarting" => Ok(ContainerState::Restarting),
            "running" => Ok(ContainerState::Running),
            "paused" => Ok(ContainerState::Paused),
            "exited" => Ok(ContainerState::Exited),
            "dead" => Ok(ContainerState::Dead),
            _ => Err(raw_status),
        }
    }
}

#[instrument(skip(docker_connection))]
// TODO is the trait `Debug` really needed?
pub async fn get_container_by_name<S: Into<String> + Debug>(
    docker_connection: &Docker,
    name: S,
) -> Result<Option<ContainerSummary>, Error> {
    let name = name.into();

    let mut filters = HashMap::new();
    filters.insert("name", vec![name.as_str()]);
    trace!("filters: {:?}", filters);

    let options = Some(ListContainersOptions {
        all: true,
        filters,
        ..Default::default()
    });

    let containers = docker_connection.list_containers(options).await?;
    debug!("got {} containers", containers.len());

    // TODO maybe this function should return an error, if two containers have the same name
    Ok(containers.get(0).cloned())
}

#[instrument(skip(docker_connection))]
pub async fn stop_container<S: Into<String> + Debug>(
    docker_connection: &Docker,
    container_name: S,
    timeout_kill: Option<usize>,
) -> Result<(), Error> {
    let container_name = container_name.into();

    docker_connection
        .stop_container(
            &container_name,
            // ? wait for value-seconds before killing the container
            timeout_kill.map(|value| StopContainerOptions { t: value as i64 }),
        )
        .await
}

#[instrument(skip(docker_connection, docker_username, docker_password, dockerfile))]
pub async fn build_image<
    S1: Into<String> + Debug,
    S2: Into<String> + Debug,
    AsyncFile: AsyncReadExt,
>(
    docker_connection: &Docker,
    docker_registry: &Option<String>,
    docker_username: &str,
    docker_password: &str,
    tag: S1,
    container_name: S2,
    build_args: HashMap<&str, &str>,
    dockerfile: AsyncFile,
) -> anyhow::Result<()> {
    debug!("Send build-image-command to docker");

    let tag = tag.into();

    let build_options = BuildImageOptions {
        t: tag.as_str(),
        dockerfile: "Dockerfile",
        buildargs: build_args,
        version: BuilderVersion::BuilderBuildKit,
        pull: false,
        session: Some(container_name.into()), // TODO what is that?
        // TODO labels: maybe add the label that the image has been build via github_webhook_handler
        ..Default::default()
    };

    // TODO find out why a login is needed?
    let credentials = {
        let mut map = HashMap::new();

        map.insert(
            docker_registry
                .clone()
                .unwrap_or("registry-1.docker.io".to_string()), // default docker registry
            DockerCredentials {
                username: Some(docker_username.to_string()),
                password: Some(docker_password.to_string()),
                ..Default::default()
            },
        );

        map
    };

    let mut buffer = Vec::with_capacity(0x400);
    tokio::pin!(dockerfile);
    dockerfile.read_to_end(&mut buffer).await?;
    let dockerfile_compressed = compress_dockerfile(&buffer)?;

    let mut image_build_stream = docker_connection.build_image(
        build_options,
        Some(credentials),
        Some(dockerfile_compressed),
    );

    debug!("Building image");
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
                error!("Encountered error while building image: {err:?}");
                bail!("{:?}", err);
            }
        }
    }

    Ok(())
}

fn compress_dockerfile(raw_dockerfile: &[u8]) -> anyhow::Result<Bytes> {
    debug!("Compress Dockerfile");

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
