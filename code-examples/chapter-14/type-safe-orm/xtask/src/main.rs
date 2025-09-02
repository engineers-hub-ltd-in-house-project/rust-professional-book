use std::env;
use std::process::Command;
use anyhow::Result;

fn main() -> Result<()> {
    let task = env::args().nth(1);
    // DATABASE_URL環境変数が設定されている必要がある
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set. Try `export DATABASE_URL=sqlite:db.sqlite`");

    match task.as_deref() {
        Some("migrate") => {
            println!("Running migrations...");
            // `sqlx-cli` を使ってマイグレーションを実行
            let status = Command::new("sqlx")
                .args(&["migrate", "run", "--database-url", &db_url])
                .status()?;
            if !status.success() {
                anyhow::bail!("sqlx migrate failed");
            }
        }
        Some("prepare") => {
            println!("Preparing query data for app...");
            // `sqlx-cli` を使ってコンパイル時検証用のデータを生成
            let status = Command::new("sqlx")
                .args(&["prepare", "--database-url", &db_url, "--workspace"])
                .current_dir("../") // ワークスペースのルートで実行
                .status()?;
             if !status.success() {
                anyhow::bail!("sqlx prepare failed");
            }
        }
        _ => {
            eprintln!("Usage: cargo xtask <migrate|prepare>");
        }
    }
    Ok(())
}
