pub mod podman;

pub struct ContainerInfo {
    pub image: String,
    pub id: String,
    pub names: Vec<String>,
    pub started_at: u64,
    pub state: String,
    pub cpu_percent: String,
    pub mem_percent: String,
    pub mem_usage: String,
}

pub trait ContainerServiceTrait: Send + Sync {
    fn get_running_containers(&self) -> Result<Vec<ContainerInfo>, String>;
    fn is_container_running(&self, id: &str) -> Result<bool, String>;
    fn stop_container(&self, id: &str) -> Result<(), String>;
}
