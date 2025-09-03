---
part: "第V部: 実践プロジェクト編"
page_count: 15
title: "実践プロジェクト: メモリプロファイラー"
---

# 第13章: プロジェクト1 - メモリプロファイラーの実装

## 学習目標
本プロジェクトを完了すると、以下が可能になります：
- [ ] これまでに学んだ複数の技術（システムプログラミング、非同期処理、Webサービス）を組み合わせて、一つの実用的なアプリケーションを構築できる。
- [ ] `sysinfo`クレートを使い、クロスプラットフォームな方法でシステム情報を取得できる。
- [ ] `axum`を使い、WebSocketによるリアルタイム通信を行うサーバーを実装できる。
- [ ] RustバックエンドとJavaScriptフロントエンドが連携する、完全なアプリケーションのアーキテクチャを設計・実装できる。

---

## 13.1 プロジェクト概要：何を構築するのか？

このプロジェクトでは、指定したプロセスID（PID）のメモリ使用量を監視し、その推移をWebブラウザ上でリアルタイムにグラフ表示する、シンプルなメモリプロファイリングツールを構築します。これは、長時間実行されるアプリケーションのメモリリーク調査などに役立つ、実用的なツールです。

このプロジェクトを通じて、Rustがいかにして異なるドメインのタスク（低レベルなシステム情報取得と、高レベルなWebサービス提供）を一つのアプリケーションに統合できるか、その強力さ と柔軟性を体験します。

### 13.1.1 機能要件

1.  **ターゲット指定:** ツール起動時に、監視対象のプロセスID（PID）をコマンドライン引数で受け取る。
2.  **データ収集:** 1秒ごとに、対象プロセスのメモリ使用量（Resident Set Size）をKB単位で取得する。
3.  **リアルタイム配信:** 収集したデータを、WebSocketを通じて接続している全てのWebクライアントにブロードキャストする。
4.  **Web UI:**
    *   ツールは、グラフ表示用のシンプルなWebページを配信する。
    *   Webページは、WebSocketに接続し、受信したデータを使ってメモリ使用量の推移をリアルタイムで折れ線グラフに描画する。

### 13.1.2 アーキテクチャ設計

この要件を満たすため、アプリケーションを以下の2つの主要コンポーネントで構成します。

1.  **コレクター（Rustバックエンド）:**
    *   コマンドライン引数を解析する。
    *   `sysinfo`クレートを使い、OSに依存しない方法でプロセス情報を取得する。
    *   `axum`フレームワークを使い、Webサーバーを起動する。
        *   一つのルートは、フロントエンドのHTML/JSファイルを配信する。
        *   もう一つのルートは、WebSocket接続を待ち受ける。
    *   WebSocket接続が確立されると、バックグラウンドで定期的にメモリ使用量をサンプリングし、接続クライアントにデータを送信する。

2.  **ビューワー（Webフロントエンド）:**
    *   単一のHTMLファイルで構成される。
    *   `Chart.js`のようなJavaScriptのグラフ描画ライブラリを利用する。
    *   ページのロード時に、コレクターのWebSocketエンドポイントに接続する。
    *   データを受信するたびに、グラフを動的に更新する。

![Architecture Diagram](assets/images/diagrams/project1-architecture.png) <!-- 図は後で作成 -->

---

## 13.2 ハンズオン：実装ステップ・バイ・ステップ

それでは、実際にアプリケーションを構築していきましょう。完成版のコードは`code-examples/chapter-13/memory-profiler/`にあります。

### 13.2.1 バックエンド：コレクターの実装

**1. プロジェクトのセットアップ**

`axum`、`tokio`、`sysinfo`、`serde`など、必要なクレートを`Cargo.toml`に追加します。

```toml
[package]
name = "memory-profiler"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1", features = ["full"] }
axum = { version = "0.6", features = ["ws"] }
sysinfo = "0.29"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tower-http = { version = "0.4", features = ["fs"] }
```

**2. メインの処理とルーティング (`src/main.rs`)**

アプリケーションのエントリーポイントです。コマンドライン引数をパースし、`axum`サーバーを起動します。

```rust
use axum::{
    routing::get,
    Router,
    extract::ws::{WebSocketUpgrade, WebSocket},
    extract::State,
    response::Html,
};
use std::sync::Arc;
use sysinfo::{System, SystemExt, ProcessExt};

struct AppState {
    pid: u32,
}

#[tokio::main]
async fn main() {
    let pid = std::env::args().nth(1)
        .expect("Usage: memory-profiler <PID>")
        .parse::<u32>()
        .expect("PID must be a number");

    let app_state = Arc::new(AppState { pid });

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/ws", get(websocket_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000 for PID {}", pid);
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> Html<String> {
    let html_content = tokio::fs::read_to_string("src/index.html").await.unwrap();
    Html(html_content)
}

// ... websocket_handler は次に実装 ...
```

**3. WebSocketハンドラの実装**

`websocket_handler`は、クライアントからのWebSocket接続要求をアップグレードし、接続が確立されたらデータ送信ループを開始します。

```rust
// main.rs に追記
use axum::extract::ws::Message;
use tokio::time::{interval, Duration};

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl axum::response::IntoResponse {
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
        sys.refresh_process(state.pid.into());

        let mem_kb = if let Some(process) = sys.process(state.pid.into()) {
            process.memory()
        } else {
            // プロセスが存在しない場合は0を送信し、ループを抜ける
            0
        };

        let data_point = MemDataPoint {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap().as_secs(),
            memory_kb: mem_kb,
        };

        let json_payload = serde_json::to_string(&data_point).unwrap();

        if socket.send(Message::Text(json_payload)).await.is_err() {
            // クライアントが切断した
            println!("Client disconnected.");
            break;
        }

        if mem_kb == 0 {
            println!("Target process not found. Closing connection.");
            break;
        }
    }
}
```

### 13.2.2 フロントエンド：ビューワーの実装

**1. HTMLファイルの作成 (`src/index.html`)**

グラフを描画するための`canvas`要素と、`Chart.js`および我々のカスタムスクリプトを読み込むための`script`タグを配置します。

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Memory Profiler</title>
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/chartjs-adapter-date-fns"></script>
</head>
<body>
    <h1>Real-time Memory Usage</h1>
    <canvas id="memoryChart" width="400" height="200"></canvas>
    <script src="/static/app.js"></script> <!-- このファイルはまだない -->
</body>
</html>
```

**2. JavaScriptの実装 (`static/app.js`)**

`axum`から配信されるように、`static`ディレクトリを作成し、その中に`app.js`を配置します。このスクリプトが、WebSocket通信とグラフ描画のロジックを担います。

```javascript
// static/app.js
window.onload = function () {
    const ctx = document.getElementById('memoryChart').getContext('2d');
    const chart = new Chart(ctx, {
        type: 'line',
        data: {
            datasets: [{
                label: 'Memory Usage (KB)',
                borderColor: 'rgb(75, 192, 192)',
                data: [],
            }]
        },
        options: {
            scales: {
                x: {
                    type: 'time',
                    time: { unit: 'second' }
                },
                y: {
                    beginAtZero: true
                }
            }
        }
    });

    const ws = new WebSocket(`ws://${window.location.host}/ws`);

    ws.onmessage = function (event) {
        const dataPoint = JSON.parse(event.data);
        const chartData = chart.data.datasets[0].data;
        
        chartData.push({
            x: dataPoint.timestamp * 1000, // Chart.jsはミリ秒を期待
            y: dataPoint.memory_kb
        });

        // 古いデータを削除してグラフが無限に大きくならないようにする
        if (chartData.length > 60) {
            chartData.shift();
        }

        chart.update();
    };

    ws.onopen = () => console.log("Connected to profiler server.");
    ws.onclose = () => console.log("Disconnected from profiler server.");
    ws.onerror = (error) => console.error("WebSocket Error:", error);
};
```

### 13.2.3 実行

1.  何かメモリを消費するプロセス（例えばブラウザなど）のPIDを調べます。
2.  `cargo run -- <PID>`でコレクターを起動します。
3.  ブラウザで`http://localhost:3000`を開きます。

指定したプロセスのメモリ使用量が、リアルタイムでグラフに描画されるはずです。

---

## 13.3 まとめ

このプロジェクトを通じて、私たちはRustの様々な側面を統合しました。

-   `sysinfo`による低レベルなシステム情報アクセス。
-   `tokio`と`axum`による高レベルな非同期WebサービスとWebSocket通信。
-   `serde`によるRustのデータ構造とJSONの相互変換。
-   `Arc<Mutex<T>>`による非同期タスク間での状態共有。

このように、Rustは一つの言語とエコシステムの中で、低レベルなシステムプログラミングから高レベルなWebアプリケーションまで、シームレスに開発することを可能にします。この強力な統合能力こそが、複雑で要求の厳しい現代のソフトウェアを構築する上で、Rustが選ばれる大きな理由の一つなのです。
