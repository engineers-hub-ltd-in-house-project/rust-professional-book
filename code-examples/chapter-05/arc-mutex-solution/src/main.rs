use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Arc<Mutex<T>> は、スレッド間で可変な状態を共有する慣用的な方法。
    // Arc: 同じデータの複数の所有者を許可する。
    // Mutex: 一度に一つのスレッドだけがデータにアクセスできるようにする（データ競合を防ぐ）。
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    println!("Arc<Mutex<T>>を使って、10個のスレッドでカウンターを10,000までインクリメントしています...");

    for i in 0..10 {
        // Arcをクローンして、各スレッドに新しい所有者を作成する。
        // これにより参照カウントがインクリメントされる。
        let counter_clone = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            // ミューテックスをロックして、カウンターへの排他的アクセスを得る。
            // ここでは簡潔さのために`unwrap()`を使用しているが、実際のコードではエラーを処理する。
            let mut num = counter_clone.lock().unwrap();
            
            for _ in 0..1000 {
                *num += 1;
            }
            // `num`がスコープを抜けるときに、ロックは自動的に解放される。
        });
        handles.push(handle);
    }

    // 全てのスレッドが完了するのを待つ。
    for handle in handles {
        handle.join().unwrap();
    }

    // 最終的な値にアクセスする。
    // 最終的な値を読み取るために、ミューテックスをもう一度ロックする必要がある。
    println!("最終カウント: {}", *counter.lock().unwrap()); // 期待値: 10000
}
