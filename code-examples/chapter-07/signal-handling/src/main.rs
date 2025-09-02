use signal_hook::{consts::SIGINT, flag::register};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- シグナルハンドリングの例 ---");
    println!("Ctrl+C を押して、優雅にシャットダウンしてください。");

    // SIGINTを受信したときにtrueになるAtomicBoolを作成する。
    let term = Arc::new(AtomicBool::new(false));

    // SIGINT (Ctrl+C) のシグナルハンドラを登録する。
    // SIGINTを受信すると、`term`がtrueに設定される。
    register(SIGINT, Arc::clone(&term))?;

    let mut counter = 0;
    while !term.load(Ordering::Relaxed) {
        println!("作業中... カウンター: {}", counter);
        counter += 1;
        thread::sleep(Duration::from_secs(1));
    }

    println!("\nSIGINTを受信しました。優雅にシャットダウンします。");
    println!("最終カウンター値: {}", counter);

    Ok(())
}
