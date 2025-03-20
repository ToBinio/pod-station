use std::process::Command;

use serde::Deserialize;

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
    fn running_containers(&self) -> Vec<ContainerInfo>;
    fn running_containers_stats(&self) -> Vec<ContainerStats>;
}

#[derive(Default)]
pub struct PodmanService;

impl PodmanService {
    pub fn new() -> Self {
        Self
    }
}

impl PodmanServiceTrait for PodmanService {
    fn running_containers(&self) -> Vec<ContainerInfo> {
        let output = Command::new("podman")
            .args(["--remote", "ps", "--format", "json"])
            .output()
            .unwrap();

        let stdout = String::from_utf8_lossy(&output.stdout);

        serde_json::from_str(&stdout).unwrap()
    }

    fn running_containers_stats(&self) -> Vec<ContainerStats> {
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
            .unwrap();

        let stdout = String::from_utf8_lossy(&output.stdout);

        serde_json::from_str(&stdout).unwrap()
    }
}
