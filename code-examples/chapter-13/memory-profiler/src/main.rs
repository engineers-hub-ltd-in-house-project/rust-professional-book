use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use std::{net::SocketAddr, sync::Arc};
use sysinfo::{System, SystemExt, ProcessExt, Pid};
use tokio::time::{interval, Duration};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

struct AppState {
    pid: Pid,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "memory_profiler=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pid_str = std::env::args()
        .nth(1)
        .expect("Usage: memory-profiler <PID>");
    let pid = pid_str.parse::<usize>()
        .expect("PID must be a number")
        .into();

    let app_state = Arc::new(AppState { pid });

    // `static`ディレクトリ以下のファイルを静的に配信する
    let app = Router::new()
        .nest_service("/", ServeDir::new("static"))
        .route("/ws", get(websocket_handler))
        .with_state(app_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

#[derive(serde::Serialize)]
struct MemDataPoint {
    timestamp: u64,
    memory_kb: u64,
}

async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    let mut sys = System::new_all();
    let mut interval = interval(Duration::from_secs(1));

    loop {
        interval.tick().await;
        sys.refresh_process(state.pid);

        let mem_kb = if let Some(process) = sys.process(state.pid) {
            process.memory()
        } else {
            0
        };

        let data_point = MemDataPoint {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            memory_kb: mem_kb,
        };

        let json_payload = serde_json::to_string(&data_point).unwrap();

        if socket.send(Message::Text(json_payload)).await.is_err() {
            tracing::debug!("Client disconnected.");
            break;
        }

        if mem_kb == 0 {
            tracing::debug!("Target process not found. Closing connection.");
            let _ = socket.close().await;
            break;
        }
    }
}
