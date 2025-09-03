---
part: "第IV部: ドメイン特化応用編"
page_count: 20
title: "ネットワークプログラミングと分散システム"
---

# 第12章: ネットワークプログラミングと分散システム

## 学習目標
本章を修了すると、以下が可能になります：
- [ ] Rustの非同期エコシステムが、高性能なネットワークサービス構築にどのように貢献するかを説明できる。
- [ ] `axum`フレームワークを使い、実践的なWeb APIサーバーを構築できる。
- [ ] 分散システムにおける合意形成問題の難しさを理解できる。
- [ ] Raftアルゴリズムの基本概念を理解し、Rustの型システムがその実装をいかに安全にするかを説明できる。

---

## 12.1 導入：信頼性の高いサービスを支えるRust

ネットワークサービスの開発は、これまでに学んだ多くの概念が交差する、応用的な領域です。第5章の並行性、第6章の非同期プログラミング、そして第7章のOSとの連携。これら全てが、一つのリクエストを処理する裏側で動いています。

Rustは、この領域で圧倒的な強みを発揮します。

- **パフォーマンス:** GCによる停止がなく、ゼロコスト抽象化により、C++に匹敵する性能でリクエストを処理できます。
- **安全性:** `Send`と`Sync`トレイトに裏打ちされた「恐れることのない並行性」により、データ競合の心配なく、マルチコアを最大限に活用できます。
- **信頼性:** 豊富な型システムとエラー処理パターン（`Result`）により、複雑なビジネスロジックや分散システムのプロトコルを、堅牢に実装できます。

本章では、まず`axum`という現代的なWebフレームワークを使って、実践的なWebサービスを構築します。次に、より複雑な分散システムの領域に踏み込み、その核心的な課題である「合意形成」について学びます。

---

## 12.2 高性能HTTPサービス with `axum`

Rustには多くのWebフレームワークが存在しますが、近年人気を博しているのが`axum`です。`axum`は、非同期ランタイムのデファクトスタンダードである`tokio`とシームレスに統合されており、モジュール性が高く、型を最大限に活用した設計が特徴です。

### 12.2.1 ハンズオン：`axum`によるJSON APIサーバー

ユーザー情報を管理する、基本的なCRUD（作成・読み取り・更新・削除）操作の一部を模したJSON APIサーバーを構築します。このハンズオンは`code-examples/chapter-12/axum-web-server/`にあります。

**1. プロジェクトのセットアップ**

`tokio`、`axum`、そしてJSONを扱うための`serde`を`Cargo.toml`に追加します。

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
axum = "0.6"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

**2. データ構造とアプリケーションの状態定義**

まず、APIで扱うデータ（`User`）と、アプリケーション全体で共有する状態（ここではインメモリのDB代わり）を定義します。

```rust
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Clone)]
struct User {
    id: u64,
    name: String,
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
}

// アプリケーションの状態。Arc<Mutex<T>>でスレッドセーフに共有する
type AppState = Arc<Mutex<Vec<User>>>;
```

**3. ハンドラの作成とルーティング**

`axum`では、リクエストを処理する関数を**ハンドラ**と呼びます。ハンドラ関数の引数に`axum`が提供する**エクストラクタ**（`Path`, `Json`, `State`など）を指定することで、リクエストの各要素を簡単かつ型安全に受け取ることができます。

```rust
use axum::{
    routing::{get, post},
    http::StatusCode,
    response::Json,
    extract::{Path, State},
    Router,
};

// GET /users : 全ユーザーを一覧表示
async fn get_users(State(state): State<AppState>) -> Json<Vec<User>> {
    let users = state.lock().unwrap().clone();
    Json(users)
}

// POST /users : 新しいユーザーを作成
async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let mut users = state.lock().unwrap();
    let new_user = User {
        id: users.len() as u64 + 1,
        name: payload.name,
    };
    users.push(new_user.clone());
    (StatusCode::CREATED, Json(new_user))
}

#[tokio::main]
async fn main() {
    let app_state = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/users", get(get_users).post(create_user))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

このサーバーを起動し、`curl`などでアクセスすれば、JSON APIが動作することが確認できます。

```bash
# ユーザー作成
curl -X POST -H "Content-Type: application/json" -d '{"name":"Alice"}' http://localhost:3000/users

# ユーザー一覧
curl http://localhost:3000/users
```

`axum`の強力な点は、エクストラクタやレスポンスの型がコンパイル時に検証されることです。これにより、実行時エラーの多くを未然に防ぎ、堅牢なAPIを効率的に開発できます。

---

## 12.3 分散システムの核心：合意形成とRaft

Webサーバーのようなステートレスなアプリケーションから一歩進み、複数のマシン（ノード）が協調して一つのデータベースやサービスを構成する**分散システム**を考えると、問題の難易度は飛躍的に上がります。ネットワークは分断されるかもしれず、ノードはクラッシュするかもしれません。そのような状況でも、システム全体として一貫した状態を保つにはどうすればよいでしょうか？

この問題を解決するのが**合意形成（コンセンサス）アルゴリズム**です。

### 12.3.1 Raftアルゴリズムの概念

**Raft**は、理解しやすさを目標に設計された合意形成アルゴリズムです。全てのノードは、常に**フォロワー**、**候補者**、**リーダー**のいずれかの状態にあります。

1.  **リーダー選出:** システムは常にただ一人のリーダーを持つことを目指します。リーダーに障害が発生すると、残りのフォロワーの中から新しい候補者が現れ、選挙によって次のリーダーが選ばれます。
2.  **ログ複製:** 全てのデータの変更は、まずリーダーによって受信されます。リーダーはその変更を「ログエントリ」として自身のログに追加し、それを全てのフォロワーに複製するよう要求します。フォロワーの大多数がログの複製に成功した時点で、そのログエントリは「コミット」され、安全なものと見なされます。

この仕組みにより、一部のノードが故障しても、大多数が生き残っている限り、システム全体としてログ（＝状態）の一貫性が保たれます。

### 12.3.2 Rustの型システムによる安全な実装

Raftのような複雑な状態機械を持つプロトコルを実装する際、Rustの型システムは絶大な力を発揮します。ゼロからRaftを実装するのは本章の範囲を超えますが、その状態やメッセージをRustの型でどう表現できるかを見てみましょう。これは`code-examples/chapter-12/raft-conceptual-types/`にあります。

```rust
// ノードの状態をenumで表現。不正な状態遷移を防ぐ。
enum NodeState {
    Follower { term: u64, voted_for: Option<ServerId> },
    Candidate { term: u64, votes_received: u64 },
    Leader { term: u64, next_index: HashMap<ServerId, LogIndex> },
}

// ノード間でやり取りされるメッセージをenumで表現
enum Message {
    RequestVote(RequestVoteArgs),
    RequestVoteResponse(RequestVoteResponseArgs),
    AppendEntries(AppendEntriesArgs),
    AppendEntriesResponse(AppendEntriesResponseArgs),
}

// メッセージの引数も構造体で明確に定義
struct RequestVoteArgs {
    term: u64,
    candidate_id: ServerId,
    last_log_index: LogIndex,
    last_log_term: u64,
}

// ... 他の構造体定義 ...
```

このように、プロトコルの仕様を`enum`と`struct`で厳密に型定義することで、

-   ありえない状態の組み合わせをコンパイル時に排除できる。
-   メッセージのフィールドを間違えたり、忘れたりすることがない。
-   `match`文による網羅的なパターンマッチングで、全ての状態とメッセージの組み合わせを処理せざるを得なくなり、考慮漏れを防げる。

といった利点が得られます。これは、C言語などでありがちな、整数フラグや`void*`ポインタを使った実装に比べて、はるかに安全で保守性の高いコードに繋がります。

**実践では、`raft-rs`（TiKVプロジェクトで使われている）のような、実績のあるクレートを利用することが賢明です。** しかし、その内部でRustの型システムがどのように貢献しているかを理解することは、分散システムを扱う上で非常に重要です。

---

## 12.4 まとめ

本章では、Rustが単なる高速な言語ではなく、信頼性の高いネットワークサービスを構築するための優れたツールであることを学びました。

-   `axum`のような現代的なWebフレームワークは、Rustの型システムを活用し、安全で効率的なAPI開発を可能にします。
-   分散システムの領域では、Raftのような複雑なプロトコルを、Rustの厳格な型（特に`enum`）を使って明確かつ安全にモデル化できます。

Rustの真価は、アプリケーションレベルの便利な抽象化から、分散合意形成のような低レベルでクリティカルな実装まで、一貫した安全性とパフォーマンスを提供できる点にあります。次章からは、これまでに学んだ全ての知識を総動員し、3つの実践的なプロジェクトに取り組んでいきます。
