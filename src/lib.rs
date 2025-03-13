use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::get};
use podman::PodmanServiceTrait;

pub mod podman;

pub fn app<PODMAN: PodmanServiceTrait + 'static>(podman_service: Arc<PODMAN>) -> Router {
    Router::new()
        .route("/containers", get(get_containers))
        .with_state(podman_service)
}

async fn get_containers<PODMAN: PodmanServiceTrait>(
    State(podman): State<Arc<PODMAN>>,
) -> Json<Vec<String>> {
    let containers = podman.running_containers();
    let containers = containers
        .iter()
        .map(|container| container.names.first().unwrap().to_string())
        .collect();
    Json(containers)
}
