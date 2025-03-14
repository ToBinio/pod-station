use std::sync::Arc;

use axum::{Json, Router, extract::State, routing::get};
use podman::PodmanServiceTrait;
use serde::Serialize;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

pub mod podman;

pub fn app<PODMAN: PodmanServiceTrait + 'static>(podman_service: Arc<PODMAN>) -> Router {
    Router::new()
        .route("/containers", get(get_containers))
        .with_state(podman_service)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}

#[derive(Serialize)]
pub struct Container {
    name: String,
    started_at: u64,
}

async fn get_containers<PODMAN: PodmanServiceTrait>(
    State(podman): State<Arc<PODMAN>>,
) -> Json<Vec<Container>> {
    let containers = podman.running_containers();
    let containers = containers
        .iter()
        .map(|container| Container {
            name: container.names.first().unwrap().to_string(),
            started_at: container.started_at,
        })
        .collect();
    Json(containers)
}
