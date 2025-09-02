use tokio::time::{sleep, Duration};

// `async`キーワードは、この関数が非同期であることを示す。
// この関数を呼び出しても、すぐには実行されず、`Future`を返す。
async fn say_hello() {
    println!("Hello");
    // `sleep`は非同期関数なので、`.await`で完了を待つ。
    // この間、スレッドはブロックされず、他のタスクを実行できる。
    sleep(Duration::from_secs(1)).await;
    println!("World!");
}

// `#[tokio::main]`マクロは、非同期関数を実行するための`tokio`ランタイムをセットアップする。
// `main`関数自体も`async`である必要がある。
#[tokio::main]
async fn main() {
    println!("基本的なasync/awaitの例を開始します...");
    say_hello().await; // `say_hello`の`Future`が完了するまで待つ。
    println!("例が終了しました。");
}
