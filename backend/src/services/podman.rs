use std::{iter::zip, process::Command};

use itertools::Itertools;
use serde::Deserialize;
use tracing::error;

use super::{ContainerInfo, ContainerServiceTrait};

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PodmanContainerInfo {
    pub image: String,
    pub id: String,
    pub names: Vec<String>,
    pub started_at: u64,
    pub state: String,
}

#[derive(Deserialize)]
pub struct PodmanContainerStats {
    pub id: String,
    pub cpu_percent: String,
    pub mem_percent: String,
    pub mem_usage: String,
}

#[derive(Default)]
pub struct PodmanService;

impl PodmanService {
    pub fn new() -> Self {
        Self
    }

    fn running_containers(&self) -> Result<Vec<PodmanContainerInfo>, String> {
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

    fn running_containers_stats(&self) -> Result<Vec<PodmanContainerStats>, String> {
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
}

impl ContainerServiceTrait for PodmanService {
    fn stop_container(&self, id: &str) -> Result<(), String> {
        let output = Command::new("podman").args(["stop", id]).output().unwrap();

        if output.status.success() {
            Ok(())
        } else {
            Err(format!("Failed to stop container: {}", output.status))
        }
    }

    fn get_running_containers(&self) -> Result<Vec<ContainerInfo>, String> {
        let container_infos = self.running_containers()?;

        let container_stats = self.running_containers_stats()?;

        let containers = zip(
            container_infos.iter().sorted_by(|a, b| a.id.cmp(&b.id)),
            container_stats.iter().sorted_by(|a, b| a.id.cmp(&b.id)),
        )
        .map(|(info, stats)| {
            // assert via starts with since stats sometimes have a shortened version
            assert!(info.id.starts_with(&stats.id));
            ContainerInfo {
                image: info.image.clone(),
                id: info.id.clone(),
                names: info.names.clone(),
                started_at: info.started_at.clone(),
                state: info.state.clone(),
                cpu_percent: stats.cpu_percent.clone(),
                mem_percent: stats.mem_percent.clone(),
                mem_usage: stats.mem_usage.clone(),
            }
        })
        .collect();

        Ok(containers)
    }

    fn is_container_running(&self, id: &str) -> Result<bool, String> {
        Ok(self.running_containers()?.iter().any(|c| c.id == id))
    }
}
