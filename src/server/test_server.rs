use serde_json::json;

use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use reqwest::StatusCode;
use std::{net::SocketAddr, time::Duration};
use tokio::{net::TcpListener, sync::oneshot, task::JoinHandle};

async fn ok() -> impl IntoResponse {
    StatusCode::OK
}

async fn not_found() -> impl IntoResponse {
    StatusCode::NOT_FOUND
}

async fn error() -> impl IntoResponse {
    StatusCode::INTERNAL_SERVER_ERROR
}

async fn slow() -> impl IntoResponse {
    tokio::time::sleep(Duration::from_millis(200)).await;
    StatusCode::OK
}

// configurable latency
async fn delay(Path(ms): Path<u64>) -> impl IntoResponse {
    tokio::time::sleep(Duration::from_millis(ms)).await;

    (StatusCode::OK, format!("delayed {}ms", ms))
}

async fn echo(body: String) -> impl IntoResponse {
    (StatusCode::OK, body)
}

async fn json_ok() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "message": "hello"
    }))
}

pub struct TestServer {
    pub addr: SocketAddr,
    shutdown_tx: oneshot::Sender<()>,
    handle: JoinHandle<()>,
}

impl TestServer {
    pub async fn start() -> Self {
        let app = Router::new()
            .route("/", get(ok))
            .route("/slow", get(slow))
            .route("/delay/{ms}", get(delay))
            .route("/error", get(error))
            .route("/404", get(not_found))
            .route("/echo", post(echo))
            .route("/json", get(json_ok));

        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("failed to bind test server");

        let addr = listener.local_addr().expect("failed to get local addr");

        let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();

        println!("🧪 Test server running at http://{}", addr);

        let handle = tokio::spawn(async move {
            axum::serve(listener, app)
                .with_graceful_shutdown(async {
                    shutdown_rx.await.ok();
                })
                .await
                .expect("test server crashed");
        });

        Self {
            addr,
            shutdown_tx,
            handle,
        }
    }

    pub fn url(&self, path: &str) -> String {
        format!("http://{}{}", self.addr, path)
    }

    pub async fn shutdown(self) {
        let _ = self.shutdown_tx.send(());

        let _ = self.handle.await;
    }
}
