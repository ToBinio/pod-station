use std::sync::Arc;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use pod_station::{
    app,
    podman::{PodmanContainer, PodmanServiceTrait},
};
use tower::ServiceExt;

pub struct MockPodmanService;

impl PodmanServiceTrait for MockPodmanService {
    fn running_containers(&self) -> Vec<PodmanContainer> {
        vec![
            PodmanContainer {
                id: "123".to_string(),
                names: vec!["test1".to_string()],
                image: "test".to_string(),
                started_at: 0,
                state: "running".to_string(),
            },
            PodmanContainer {
                id: "456".to_string(),
                names: vec!["test2".to_string()],
                image: "test2".to_string(),
                started_at: 0,
                state: "running".to_string(),
            },
            PodmanContainer {
                id: "789".to_string(),
                names: vec!["test3".to_string()],
                image: "test3".to_string(),
                started_at: 0,
                state: "running".to_string(),
            },
        ]
    }
}

#[tokio::test]
async fn get_containers() {
    let podman_service = Arc::new(MockPodmanService);
    let app = app(podman_service.clone());

    let response = app
        .oneshot(
            Request::builder()
                .uri("/containers")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = serde_json::from_slice::<Vec<String>>(&body[..]).unwrap();

}
