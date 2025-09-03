---
part: "第V部: 実践プロジェクト編"
page_count: 15
title: "実践プロジェクト: 型安全データベースORM"
---

# 第14章: プロジェクト2 - 型安全データベースORM

## 学習目標
本プロジェクトを完了すると、以下が可能になります：
- [ ] 従来のORMが抱える、実行時エラーの問題点を説明できる。
- [ ] Rustの手続きマクロを使い、コードを解析・生成する基本的な方法を理解できる。
- [ ] `sqlx`のオフラインコンパイルの仕組みを参考に、SQLクエリをコンパイル時に検証するアプローチを実装できる。
- [ ] `xtask`パターンを使い、プロジェクト固有のビルドタスク（マイグレーションなど）を管理できる。

---

## 14.1 プロジェクト概要：実行時エラーを撲滅せよ

多くのWebアプリケーションにとって、データベースとの連携は核心的な機能です。しかし、ここには大きな落とし穴が潜んでいます。`"SELECT user_name FROM users WHERE user_id = ?"`のようなSQLクエリを文字列として扱う場合、コンパイラはその中身を理解できません。

-   `user_name`を`username`にタイポしても、コンパイルは通ってしまう。
-   データベーススキーマが変更され、`user_name`カラムが削除されても、コードは古いまま。
-   `user_id`に文字列を渡そうとしても、実行するまでエラーにならない。

これらの問題は、アプリケーションを実行し、該当コードが呼び出された瞬間に初めて発覚します。これは、テストカバレッジの低い、発見が困難なバグの温床です。

このプロジェクトでは、Rustの最も強力な機能の一つである**手続きマクロ**を使い、この問題を解決する「型安全ORM」のプロトタイプを構築します。目標は、SQLの正しさを**実行時ではなくコンパイル時に検証**し、上記のようなエラーをコンパイルエラーとして開発者にフィードバックすることです。

この挑戦は、`sqlx`という先進的なRustライブラリのオフラインコンパイル機能から強いインスピレーションを受けています。我々はその仕組みを紐解き、再実装することで、Rustのメタプログラミング能力の深淵を探求します。

### 14.1.1 アーキテクチャ設計

このプロジェクトは、モノレポ（一つのリポジトリに複数のクレートを配置）として構成します。

1.  **`query-macro`クレート:** `query_as!`という手続きマクロを定義するクレート。
2.  **`app`クレート:** `query-macro`を利用して、実際のデータベース操作を行うアプリケーションクレート。
3.  **`xtask`クレート:** `cargo xtask migrate`や`cargo xtask prepare`といった、プロジェクト固有のタスク（マイグレーション実行やスキーマ情報生成）を実装するクレート。
4.  **`migrations`ディレクトリ:** `001_create_users.sql`のような、バージョン管理されたSQLファイルを格納する場所。

**コンパイル時の検証フロー:**
1.  開発者は`cargo xtask prepare`を実行する。これは内部で`sqlx-cli`を呼び出し、データベースに接続してスキーマ情報を`.sqlx`ディレクトリにJSONとして保存する。
2.  開発者が`app`クレートをコンパイルする。
3.  `query_as!`マクロが展開される。
4.  マクロは、引数として与えられたSQLクエリと、`.sqlx`に保存されたスキーマ情報を読み込む。
5.  SQLを解析し、テーブル名、カラム名、型、引数の数と型が、スキーマ情報およびRustのコード（例：`User`構造体）と一致しているかを検証する。
6.  検証に失敗すれば、`compile_error!`マクロを使ってコンパイルエラーを発生させる。
7.  検証に成功すれば、`sqlx`のランタイムコードを生成する。

---

## 14.2 ハンズオン：実装ステップ・バイ・ステップ

それでは、この野心的なプロジェクトを構築していきましょう。完成版のコードは`code-examples/chapter-14/type-safe-orm/`にあります。

### 14.2.1 `xtask`によるマイグレーション管理

まず、プロジェクトの土台となるデータベースのスキーマを管理する仕組みを作ります。`xtask`は、`Makefile`や`justfile`の代わりに、Rust自身でビルドスクリプトを記述する、Cargoで推奨されているパターンです。

**1. `migrations`ディレクトリの作成**

プロジェクトのルートに、最初のマイグレーションファイルを作成します。

```sql
-- migrations/001_create_users.sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

**2. `xtask`クレートの実装**

`xtask`クレートは、`sqlx-cli`をコマンドとして呼び出すラッパーとして機能します。

```rust
// xtask/src/main.rs

use std::env;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let task = env::args().nth(1);
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    match task.as_deref() {
        Some("migrate") => {
            println!("Running migrations...");
            Command::new("sqlx")
                .args(&["migrate", "run", "--database-url", &db_url])
                .status()?;
        }
        Some("prepare") => {
            println!("Preparing query data...");
            Command::new("sqlx")
                .args(&["prepare", "--database-url", &db_url])
                .current_dir("../app") // appクレート内で実行
                .status()?;
        }
        _ => {
            eprintln!("Usage: cargo xtask <migrate|prepare>");
        }
    }
    Ok(())
}
```

これで、`cargo xtask migrate`を実行すれば、データベースに`users`テーブルが作成されます。

### 14.2.2 `query-macro`手続きマクロの実装

次がこのプロジェクトの核心、手続きマクロです。手続きマクロは、コンパイラからコードのトークンストリームを受け取り、新しいトークンストリームを生成して返す、特殊なクレートです。

**1. `Cargo.toml`のセットアップ**

`proc-macro = true`を指定し、コードの解析・生成のために`syn`と`quote`を導入します。

```toml
# query-macro/Cargo.toml
[lib]
proc-macro = true

[dependencies]
syn = { version = "1.0", features = ["full"] }
quote = "1.0"
# ... sqlx-core, etc ...
```

**2. マクロの実装**

マクロの内部では、`sqlx-macros`が内部で行っているのと同様の処理を、簡略化して実装します。すなわち、コンパイル時に`.sqlx`ディレクトリからスキーマ情報を読み込み、SQLクエリを検証します。

```rust
// query-macro/src/lib.rs
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn query_as_validated(input: TokenStream) -> TokenStream {
    // ... マクロの引数をパース ...
    // let sql: LitStr = ...

    // コンパイル時にスキーマ情報を読み込む
    // let offline_data = sqlx_core::offline::load_offline_data(...).unwrap();

    // SQLをパースし、スキーマと照合して検証
    // let validation_result = sqlx_core::query::validate(sql.value(), &offline_data);

    // match validation_result {
    //     Ok(_) => { ... }
    //     Err(e) => {
    //         // 検証失敗ならコンパイルエラーを生成
    //         return syn::Error::new(sql.span(), e.to_string())
    //             .to_compile_error()
    //             .into();
    //     }
    // }

    // 検証成功なら、sqlxのランタイムコードを生成
    let expanded = quote! {
        // sqlx::query_as!(...)
    };

    expanded.into()
}
```
（注：上記は概念を示すための擬似コードです。実際の`sqlx`の内部実装はより複雑です。）

### 14.2.3 `app`クレートでの利用

最後に、作成したマクロをアプリケーションで利用します。

```rust
// app/src/main.rs
use query_macro::query_as_validated;

#[derive(Debug)]
struct User {
    id: i64,
    name: String,
}

#[tokio::main]
async fn main() {
    // 1. 準備
    // export DATABASE_URL=sqlite:db.sqlite
    // cargo xtask migrate
    // cargo xtask prepare

    // 2. コンパイル
    // このマクロはコンパイル時に検証される
    let user = query_as_validated!(
        User, 
        "SELECT id, name FROM users WHERE id = ?", 
        1i64
    ).fetch_one(...).await.unwrap();

    println!("{:?}", user);

    // 3. エラーを試す
    // 以下はコンパイルエラーになるはず！
    // let _ = query_as_validated!(User, "SELECT id, name, email FROM users"); // emailカラムは存在しない
    // let _ = query_as_validated!(User, "SELECT id, name FROM users WHERE id = ?", "hello"); // 型が違う
}
```

開発者が`email`カラムを追加するなどのスキーマ変更を行った場合、`cargo xtask prepare`を再実行してスキーマ情報を更新しない限り、古いクエリはコンパイルエラーになります。これにより、コードとデータベースの間の不整合が、実行時まで持ち越されることを防ぎます。

---

## 14.3 まとめ

このプロジェクトでは、Rustの最もユニークで強力な機能である「手続きマクロ」を使い、データベースアクセスの安全性を劇的に向上させる方法を探求しました。

-   **コンパイル時検証:** Rustのマクロは、単なるテキスト置換ではなく、コンパイルプロセスに深く関与し、外部リソース（データベーススキーマ）を参照してコードの正しさを検証できます。
-   **`xtask`パターン:** プロジェクト固有のビルド・管理タスクを、使い慣れたRust言語で記述し、`cargo`とシームレスに統合できます。
-   **安全性とパフォーマンスの両立:** このアプローチは、`sqlx`が示すように、実行時にはプリペアードステートメントとして効率的に動作し、最高のパフォーマンスを維持したまま、コンパイル時の安全性を提供します。

もちろん、全てのプロジェクトでこのような複雑なビルドプロセスが必要なわけではありません。しかし、システムの信頼性が最重要視される場面において、Rustのメタプログラミング能力が、いかにして他の言語では不可能なレベルの堅牢性を実現するか、その一端を垣間見ることができたはずです。
