use tokio::time::{sleep, Duration};

async fn learn_song() -> String {
    println!("歌を覚えています...");
    sleep(Duration::from_millis(500)).await;
    println!("歌を覚えました！");
    "歌を覚えました".to_string()
}

async fn sing_song(song: String) {
    println!("歌っています: {}", song);
    sleep(Duration::from_millis(200)).await; // 歌う時間をシミュレート
    println!("歌い終わりました！");
}

async fn dance() {
    println!("踊っています...");
    sleep(Duration::from_millis(300)).await; // 踊る時間をシミュレート
    println!("踊り終わりました！");
}

#[tokio::main]
async fn main() {
    println!("並行タスクの例を開始します...");

    // `tokio::join!`は複数のFutureを並行して実行し、全てが完了するのを待つ。
    // タスクは実行をインターリーブする。
    let (song_learned, _) = tokio::join!(
        learn_song(), // このタスクは500msかかる
        dance()       // このタスクは300msかかる
    );

    // `song_learned`は`learn_song()`の結果。
    sing_song(song_learned).await;

    println!("全ての並行タスクが終了しました。");
}
