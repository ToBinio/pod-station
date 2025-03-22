use std::sync::Arc;

use axum_test::TestServer;
use pod_station::{
    app,
    podman::{ContainerInfo, ContainerStats, PodmanServiceTrait},
};
use serde_json::json;

pub struct MockPodmanService;

impl PodmanServiceTrait for MockPodmanService {
    fn running_containers(&self) -> Result<Vec<ContainerInfo>, String> {
        Ok(vec![
            ContainerInfo {
                id: "123".to_string(),
                names: vec!["test1".to_string()],
                image: "test".to_string(),
                started_at: 894,
                state: "running".to_string(),
            },
            ContainerInfo {
                id: "456".to_string(),
                names: vec!["test2".to_string()],
                image: "test2".to_string(),
                started_at: 89583,
                state: "running".to_string(),
            },
            ContainerInfo {
                id: "789".to_string(),
                names: vec!["test3".to_string()],
                image: "test3".to_string(),
                started_at: 123123,
                state: "running".to_string(),
            },
        ])
    }

    fn running_containers_stats(&self) -> Result<Vec<ContainerStats>, String> {
        Ok(vec![
            ContainerStats {
                id: "123".to_string(),
                cpu_percent: "0.2%".to_string(),
                mem_percent: "0.1%".to_string(),
                mem_usage: "114.7kB / 33.44GB".to_string(),
            },
            ContainerStats {
                id: "456".to_string(),
                cpu_percent: "0.5%".to_string(),
                mem_percent: "0.8%".to_string(),
                mem_usage: "114.7kB / 33.44GB".to_string(),
            },
            ContainerStats {
                id: "789".to_string(),
                cpu_percent: "0.23%".to_string(),
                mem_percent: "0.62%".to_string(),
                mem_usage: "114.7kB / 33.44GB".to_string(),
            },
        ])
    }

    fn stop_container(&self, _id: &str) -> Result<(), String> {
        self.running_containers()
            .unwrap()
            .iter()
            .find(|c| c.id == _id)
            .map(|_| ())
            .ok_or("Container not found".to_string())
    }
}

fn test_server() -> TestServer {
    let podman_service = Arc::new(MockPodmanService);
    let app = app(podman_service.clone());

    TestServer::builder().http_transport().build(app).unwrap()
}

#[tokio::test]
async fn get_containers() {
    let server = test_server();

    let response = server.get("/containers").await;

    response.assert_status_ok();
    response.assert_json(&json!(       [
        {
            "id": "123",
            "name": "test1",
            "started_at": 894,
            "cpu_percent": 0.2,
            "memory_percent": 0.1,
            "memory_usage": "114.7kB / 33.44GB"
        },
        {
            "id": "456",
            "name": "test2",
            "started_at": 89583,
            "cpu_percent": 0.5,
            "memory_percent": 0.8,
            "memory_usage": "114.7kB / 33.44GB"
        },
        {
            "id": "789",
            "name": "test3",
            "started_at": 123123,
            "cpu_percent": 0.23,
            "memory_percent": 0.62,
            "memory_usage": "114.7kB / 33.44GB"
        }
    ]));
}

#[tokio::test]
async fn get_containers_ws() {
    let server = test_server();

    let mut connection = server.get_websocket("/ws").await.into_websocket().await;

    let mut assert_message = async || {
        connection
            .assert_receive_json(&json!([
                {
                    "id": "123",
                    "name": "test1",
                    "started_at": 894,
                    "cpu_percent": 0.2,
                    "memory_percent": 0.1,
                    "memory_usage": "114.7kB / 33.44GB"
                },
                {
                    "id": "456",
                    "name": "test2",
                    "started_at": 89583,
                    "cpu_percent": 0.5,
                    "memory_percent": 0.8,
                    "memory_usage": "114.7kB / 33.44GB"
                },
                {
                    "id": "789",
                    "name": "test3",
                    "started_at": 123123,
                    "cpu_percent": 0.23,
                    "memory_percent": 0.62,
                    "memory_usage": "114.7kB / 33.44GB"
                }
            ]))
            .await;
    };

    assert_message().await;
    assert_message().await;
    assert_message().await;
}

#[tokio::test]
async fn stop_containers() {
    let server = test_server();

    let response = server.post("/containers/stop/123").await;

    response.assert_status_ok();
}

#[tokio::test]
async fn stop_containers_unknown() {
    let server = test_server();

    let response = server.post("/containers/stop/404").await;

    response.assert_status_not_found();
}
