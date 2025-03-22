use std::process::Command;

use serde::Deserialize;
use tracing::error;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContainerInfo {
    pub image: String,
    pub id: String,
    pub names: Vec<String>,
    pub started_at: u64,
    pub state: String,
}

#[derive(Deserialize)]
pub struct ContainerStats {
    pub id: String,
    pub cpu_percent: String,
    pub mem_percent: String,
    pub mem_usage: String,
}

pub trait PodmanServiceTrait: Send + Sync {
    fn running_containers(&self) -> Result<Vec<ContainerInfo>, String>;
    fn running_containers_stats(&self) -> Result<Vec<ContainerStats>, String>;
    fn stop_container(&self, id: &str) -> Result<(), String>;
}

#[derive(Default)]
pub struct PodmanService;

impl PodmanService {
    pub fn new() -> Self {
        Self
    }
}

impl PodmanServiceTrait for PodmanService {
    fn running_containers(&self) -> Result<Vec<ContainerInfo>, String> {
        let output = Command::new("podman")
            .args(["--remote", "ps", "--format", "json"])
            .output()
            .map_err(|err| {
                error!("Failed to get running containers: {}", err);
                err.to_string()
            })?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        serde_json::from_str(&stdout).map_err(|err| {
            error!("Failed to parse running containers: {}", err);
            err.to_string()
        })
    }

    fn running_containers_stats(&self) -> Result<Vec<ContainerStats>, String> {
        let output = Command::new("podman")
            .args([
                "--remote",
                "stats",
                "--format",
                "json",
                "--no-stream",
                "--no-reset",
            ])
            .output()
            .map_err(|err| {
                error!("Failed to get running containers stats: {}", err);
                err.to_string()
            })?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        let serde = serde_json::from_str(&stdout);

        serde.map_err(|err| {
            error!("Failed to parse running containers stats: {}", err);
            err.to_string()
        })
    }

    fn stop_container(&self, id: &str) -> Result<(), String> {
        let output = Command::new("podman").args(["stop", id]).output().unwrap();

        if output.status.success() {
            Ok(())
        } else {
            Err(format!("Failed to stop container: {}", output.status))
        }
    }
}
