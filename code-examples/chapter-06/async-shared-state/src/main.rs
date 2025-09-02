use std::sync::Arc;
use tokio::sync::Mutex; // tokioの非同期Mutexを使用
use tokio::time::{sleep, Duration};

async fn increment_counter(counter: Arc<Mutex<u32>>, task_id: u32) {
    println!("タスク {} 開始中...", task_id);
    // ミューテックスを非同期でロックする。
    // もしロックが他のタスクによって保持されている場合、このタスクは
    // スレッドをブロックせずに、他のタスクが実行されるのを待つ。
    let mut num = counter.lock().await;
    
    // ロックを保持しながら非同期作業をシミュレート
    sleep(Duration::from_millis(10)).await; 
    
    *num += 1;
    println!("タスク {} がカウンターを {} にインクリメントしました", task_id, *num);
    
    // `num`がスコープを抜けるときに、ロックは自動的に解放される。
}

#[tokio::main]
async fn main() {
    println!("非同期共有状態の例を開始します...");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = tokio::spawn(increment_counter(counter_clone, i));
        handles.push(handle);
    }

    // 全てのスレッド化されたタスクが完了するのを待つ。
    for handle in handles {
        handle.await.unwrap();
    }

    // 最終的な値にアクセスする。
    // 最終的な値を読み取るために、ミューテックスをもう一度ロックする必要がある。
    println!("最終カウント: {}", *counter.lock().await); // 期待値: 10
}
