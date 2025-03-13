use std::sync::Arc;

use pod_station::{app, podman::PodmanService};

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app(Arc::new(PodmanService::new())))
        .await
        .unwrap();
}
