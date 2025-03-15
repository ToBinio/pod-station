use std::{sync::Arc, time::Duration};

use axum::{
    Json, Router,
    extract::{
        State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
    routing::{any, get},
};
use futures::{sink::SinkExt, stream::StreamExt};
use podman::PodmanServiceTrait;
use serde::Serialize;
use tower_http::trace::{self, TraceLayer};
use tracing::{Level, warn};

pub mod podman;

pub fn app<PODMAN: PodmanServiceTrait + 'static>(podman_service: Arc<PODMAN>) -> Router {
    Router::new()
        .route("/containers", get(get_containers))
        .route("/ws", any(ws_handler))
        .with_state(podman_service)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
}

#[derive(Serialize)]
pub struct Container {
    id: String,
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
            id: container.id.clone(),
            name: container.names.first().unwrap().to_string(),
            started_at: container.started_at,
        })
        .collect();

    Json(containers)
}

async fn ws_handler<PODMAN: PodmanServiceTrait + 'static>(
    ws: WebSocketUpgrade,
    State(podman): State<Arc<PODMAN>>,
) -> impl IntoResponse {
    let podman = podman.clone();
    ws.on_upgrade(move |socket| handle_socket(socket, podman))
}

#[derive(Serialize)]
pub struct ContainerStats {
    id: String,
    cpu_percent: f32,
    memory_percent: f32,
    memory_usage: String,
}

async fn handle_socket<PODMAN: PodmanServiceTrait + 'static>(
    socket: WebSocket,
    podman: Arc<PODMAN>,
) {
    let (mut sender, mut receiver) = socket.split();

    let send_task = tokio::spawn(async move {
        loop {
            let message = serde_json::to_string(
                &podman
                    .running_containers_stats()
                    .iter()
                    .map(|stats| ContainerStats {
                        id: stats.id.clone(),
                        cpu_percent: stats
                            .cpu_percent
                            .trim_end_matches('%')
                            .parse()
                            .unwrap_or(0.0),
                        memory_percent: stats
                            .mem_percent
                            .trim_end_matches('%')
                            .parse()
                            .unwrap_or(0.0),
                        memory_usage: stats.mem_usage.clone(),
                    })
                    .collect::<Vec<_>>(),
            )
            .unwrap();

            match sender.send(Message::Text(message.into())).await {
                Ok(_) => {}
                Err(err) => {
                    warn!("{}", err);
                }
            }

            tokio::time::sleep(Duration::from_millis(1000)).await;
        }
    });

    while let Some(Ok(msg)) = receiver.next().await {
        if let Message::Close(_) = msg {
            break;
        }
    }

    send_task.abort();
}
