use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // 新しいチャネルを作成する。`tx`は送信側、`rx`は受信側。
    let (tx, rx) = mpsc::channel();

    // 送信側をクローンして、もう一つのプロデューサーを作成する。
    let tx1 = mpsc::Sender::clone(&tx);

    // プロデューサースレッド 1
    thread::spawn(move || {
        let messages = vec![
            String::from("hi"),
            String::from("from"),
            String::from("thread 1"),
        ];

        for msg in messages {
            println!("スレッド 1 が送信中: {}", msg);
            tx.send(msg).unwrap(); // メッセージを送信する。所有権がムーブされる。
            thread::sleep(Duration::from_millis(100));
        }
    });

    // プロデューサースレッド 2
    thread::spawn(move || {
        let messages = vec![
            String::from("more"),
            String::from("messages"),
            String::from("from thread 2"),
        ];

        for msg in messages {
            println!("スレッド 2 が送信中: {}", msg);
            tx1.send(msg).unwrap(); // メッセージを送信する。所有権がムーブされる。
            thread::sleep(Duration::from_millis(150));
        }
    });

    // コンシューマー（メインスレッド）
    // `rx`はメッセージを受信するイテレータ。
    for received in rx {
        println!("メインスレッドが受信: {}", received);
    }

    println!("全てのメッセージが送信され、受信されました。");
}
