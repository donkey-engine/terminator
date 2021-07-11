use shiplift::{Docker, Container, LogsOptions};
use futures_util::stream::Stream;

pub struct DockerClient {
    api: Docker,
}

impl DockerClient {
    pub fn new() -> DockerClient {
        DockerClient {
            api: Docker::new(),
        }
    }

    pub fn read_logs(&self, container_id: &String) {
        self.api
            .containers()
            .get(container_id)
            .logs(
                &LogsOptions::builder().stdout(true).stderr(true).build()
            )
    }
}
