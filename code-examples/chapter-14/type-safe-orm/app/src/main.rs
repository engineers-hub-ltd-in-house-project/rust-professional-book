use anyhow::Result;
use query_macro::query_as_validated;

// この構造体は`migrations/001_create_users.sql`のスキーマと
// 以下のクエリに一致している必要があります。
// マクロはこれをコンパイル時に検証します。
#[derive(Debug)]
struct User {
    id: i64,
    name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Running application.");
    println!("Ensure you have run:");
    println!("1. `cargo install sqlx-cli`");
    println!("2. `export DATABASE_URL=sqlite:db.sqlite`");
    println!("3. `touch db.sqlite`");
    println!("4. `cargo xtask migrate`");
    println!("5. `cargo xtask prepare`");
    println!("---------------------------------");

    let db_url = std::env::var("DATABASE_URL").unwrap();
    let pool = sqlx::SqlitePool::connect(&db_url).await?;

    // ユーザーを作成
    let _ = sqlx::query("INSERT INTO users (id, name) VALUES (?, ?)")
        .bind(1i64)
        .bind("Alice".to_string())
        .execute(&pool)
        .await;

    // コンパイル時に検証されたマクロを使用！
    let user = query_as_validated!(
        User,
        "SELECT id, name FROM users WHERE id = ?",
        1i64
    )
    .fetch_one(&pool)
    .await?;

    println!("Fetched user: {:?}", user);

    // 以下の行のコメントを外すと、`cargo xtask prepare`実行後に
    // コンパイルエラーが発生します。`sqlx`がクエリ情報を保存し、
    // `sqlx`自身の`query!`マクロ（我々のマクロが内部で呼び出す）が
    // 不一致を検出するためです。

    // COMPILE ERROR: `email`カラムは存在しない
    // let _ = sqlx::query!("SELECT id, name, email FROM users");

    // COMPILE ERROR: パラメータの型が不一致
    // let _ = sqlx::query!("SELECT id, name FROM users WHERE id = ?", "a-string-not-an-int");

    Ok(())
}
