use std::process::Command;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PodmanContainer {
    pub image: String,
    pub id: String,
    pub names: Vec<String>,
    pub started_at: u64,
    pub state: String,
}

pub trait PodmanServiceTrait: Send + Sync {
    fn running_containers(&self) -> Vec<PodmanContainer>;
}

pub struct PodmanService;

impl PodmanService {
    pub fn new() -> Self {
        Self
    }
}

impl PodmanServiceTrait for PodmanService {
    fn running_containers(&self) -> Vec<PodmanContainer> {
        let output = Command::new("podman")
            .args(["ps", "--format", "json"])
            .output()
            .unwrap();

        let stdout = String::from_utf8_lossy(&output.stdout);

        serde_json::from_str(&stdout).unwrap()
    }
}
