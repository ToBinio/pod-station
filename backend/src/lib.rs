use std::{sync::Arc, time::Duration};

use axum::{
    Json, Router,
    extract::{
        Path, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{any, get, post},
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::Serialize;
use services::{ContainerInfo, ContainerServiceTrait};
use tower_http::trace::{self, TraceLayer};
use tracing::{Level, error, warn};

pub mod services;

pub fn app<PODMAN: ContainerServiceTrait + 'static>(podman_service: Arc<PODMAN>) -> Router {
    Router::new()
        .route("/containers", get(get_all_containers))
        .route("/containers/stop/{id}", post(stop_container))
        .route("/containers/ws", any(ws_handler))
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
    cpu_percent: f32,
    memory_percent: f32,
    memory_usage: String,
}

impl From<ContainerInfo> for Container {
    fn from(value: ContainerInfo) -> Self {
        Container {
            id: value.id.clone(),
            name: value.names.first().unwrap().to_string(),
            started_at: value.started_at,
            cpu_percent: value
                .cpu_percent
                .trim_end_matches('%')
                .parse()
                .unwrap_or(0.0),
            memory_percent: value
                .mem_percent
                .trim_end_matches('%')
                .parse()
                .unwrap_or(0.0),
            memory_usage: value.mem_usage.clone(),
        }
    }
}

async fn get_all_containers<PODMAN: ContainerServiceTrait>(
    State(podman): State<Arc<PODMAN>>,
) -> Response {
    let containers = podman.get_running_containers();

    match containers {
        Ok(ok) => {
            Json(ok.into_iter().map(|x| x.into()).collect::<Vec<Container>>()).into_response()
        }
        Err(err) => {
            error!("Error getting container stats: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn ws_handler<PODMAN: ContainerServiceTrait + 'static>(
    ws: WebSocketUpgrade,
    State(podman): State<Arc<PODMAN>>,
) -> impl IntoResponse {
    let podman = podman.clone();
    ws.on_upgrade(move |socket| handle_socket(socket, podman))
}

async fn handle_socket<PODMAN: ContainerServiceTrait + 'static>(
    socket: WebSocket,
    podman: Arc<PODMAN>,
) {
    let (mut sender, mut receiver) = socket.split();

    let send_task = tokio::spawn(async move {
        loop {
            let containers = match podman.get_running_containers() {
                Ok(ok) => ok.into_iter().map(|x| x.into()).collect::<Vec<Container>>(),
                Err(err) => {
                    error!("Error getting container stats: {}", err);
                    vec![]
                }
            };

            let message = serde_json::to_string(&containers).unwrap();

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

async fn stop_container<PODMAN: ContainerServiceTrait>(
    Path(id): Path<String>,
    State(podman): State<Arc<PODMAN>>,
) -> StatusCode {
    match podman.is_container_running(&id) {
        Ok(ok) => {
            if !ok {
                return StatusCode::NOT_FOUND;
            }
        }
        Err(err) => {
            warn!("{}", err);
            return StatusCode::NOT_FOUND;
        }
    }

    let result = podman.stop_container(&id);

    match result {
        Ok(_) => StatusCode::OK,
        Err(err) => {
            warn!("{}", err);
            StatusCode::NOT_FOUND
        }
    }
}
