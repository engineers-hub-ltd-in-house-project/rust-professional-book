use axum::{
    routing::{get, post},
    http::StatusCode,
    response::Json,
    extract::{Path, State},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    // アプリケーションの状態: 共有され、スレッドセーフなユーザーのベクタ
    let db = Arc::new(Mutex::new(Vec::new()));

    // アプリケーションのルートを定義
    let app = Router::new()
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", get(get_user_by_id))
        .with_state(db);

    // サーバーを実行
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

// 共有状態の型エイリアス
type Db = Arc<Mutex<Vec<User>>>;

#[derive(Debug, Serialize, Clone)]
struct User {
    id: u64,
    name: String,
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    name: String,
}

// 全ユーザーを取得するハンドラ
async fn get_users(State(db): State<Db>) -> Json<Vec<User>> {
    let users = db.lock().unwrap().clone();
    Json(users)
}

// IDで単一ユーザーを取得するハンドラ
async fn get_user_by_id(
    State(db): State<Db>,
    Path(id): Path<u64>,
) -> Result<Json<User>, StatusCode> {
    let users = db.lock().unwrap();
    users
        .iter()
        .find(|user| user.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

// 新規ユーザーを作成するハンドラ
async fn create_user(
    State(db): State<Db>,
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let mut users = db.lock().unwrap();
    let new_user = User {
        id: users.len() as u64, // シンプルなID生成
        name: payload.name,
    };
    users.push(new_user.clone());
    (StatusCode::CREATED, Json(new_user))
}
