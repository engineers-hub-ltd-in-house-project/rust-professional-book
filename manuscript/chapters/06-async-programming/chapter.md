# 第6章: 非同期プログラミング - Future理論の実践

## 学習目標
本章を修了すると、以下が可能になります：
- [ ] 同期、並行、非同期の違いを理解し、非同期プログラミングが解決する問題を説明できる。
- [ ] `async/await`が、コンパイラによってどのように状態機械に変換されるかを概念的に理解できる。
- [ ] `Future`トレイトが非同期処理の基本単位であることを理解できる。
- [ ] `tokio`ランタイムを使い、実践的な非同期アプリケーション（ネットワークサービスなど）を構築できる。

---

## 6.1 導入：なぜ「非同期」なのか？

第5章では、CPUバウンドなタスクを複数のコアで並列実行することで高速化する方法を学びました。しかし、現代のアプリケーション、特にネットワークサービスでは、CPUが計算している時間よりも、**I/O（ディスクやネットワーク）の応答を待っている時間**の方がはるかに長いことがよくあります。

伝統的な「スレッドプール」モデルでは、リクエストごとにOSのスレッドを一つ割り当てます。しかし、1万のクライアントが同時に接続する状況（C10k問題）を考えると、1万のスレッドを作成・管理するのは、OSにとって非常に大きなオーバーヘッドとなります。ほとんどのスレッドは、ただI/Oを待っているだけでCPUを消費しないにも関わらず、メモリやコンテキストスイッチのコストを発生させます。

**非同期プログラミング**は、この問題を解決します。単一のスレッド（あるいは少数のスレッド）で、何千ものI/Oバウンドなタスクを効率的に管理する仕組みです。タスクがI/Oでブロックされる（待機状態になる）場合、そのタスクを一時停止し、スレッドは別の実行可能なタスクの処理に移ります。これにより、スレッドは常に「仕事をしている」状態を保ち、システムリソースを最大限に活用できるのです。

---

## 6.2 Rustの非同期構文：`async` と `.await`

Rustは、`async`と`.await`というキーワードを通じて、非同期プログラミングを言語レベルでサポートします。

```rust
// `async`キーワードは、この関数が非同期であることを示す
async fn say_hello() {
    println!("Hello");
    // `tokio::time::sleep`は、指定時間待機する非同期関数
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    println!("World!");
}

// 非同期関数を実行するには、ランタイムが必要
#[tokio::main]
async fn main() {
    say_hello().await;
}
```

- **`async fn`:** この関数を呼び出しても、すぐには実行されません。代わりに、その処理の全体を表す**`Future`**という値を返します。
- **`.await`:** `Future`の処理が完了するまで、現在の関数の実行を**非ブロッキング**で待機します。もし`Future`がまだ完了していなければ、現在のタスクは一時停止され、スレッドは他のタスクを実行できます。

---

## 6.3 非同期の仕組み：Future、状態機械、そしてランタイム

`async/await`は魔法ではありません。コンパイラと**非同期ランタイム**（`tokio`など）の協調によって実現されています。

### 6.3.1 `Future`トレイト

全ての非同期処理の基本単位が`Future`トレイトです。これは、将来のある時点で完了するかもしれない値を表現します。その定義は本質的に以下のようになっています。

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T), // Futureが完了し、値Tを生成した
    Pending,  // Futureはまだ完了していない
}
```

### 6.3.2 非同期ランタイム（Executor）

**非同期ランタイム**（あるいはExecutor）の仕事は、たくさんの`Future`を管理し、それらを完了まで駆動することです。ランタイムは、以下のようなループを実行します。

1.  管理している`Future`の一つを取り出し、`poll`メソッドを呼び出す。
2.  `poll`が`Poll::Ready(value)`を返せば、その`Future`は完了。結果を次の処理に渡す。
3.  `poll`が`Poll::Pending`を返せば、その`Future`はまだ完了していない。ランタイムはそれを一旦脇に置き、別の`Future`の`poll`に移る。
4.  I/O（ネットワーク、タイマーなど）の準備ができたというOSからの通知を受け取ると、そのI/Oを待っていた`Future`を「実行可能」な状態に戻し、再び`poll`の対象にする。

`#[tokio::main]`は、このランタイムをセットアップし、`main`関数の`Future`を実行するための便利なマクロです。

### 6.3.3 コンパイラの変換：`async/await`から状態機械へ

コンパイラは、`async fn`を、`Future`トレイトを実装した**状態機械（State Machine）**に変換します。各`.await`呼び出しが、状態遷移のポイントとなります。

```rust
async fn say_hello() {
    println!("Hello"); // State 0
    tokio::time::sleep(Duration::from_secs(1)).await; // State 0 -> State 1
    println!("World!"); // State 1
}
```

この関数は、以下のような状態を持つenumに変換されるとイメージできます。

```rust,ignore
enum SayHelloState {
    Start,
    WaitingForSleep(Pin<Box<dyn Future<Output = ()>>>),
    Done,
}
```

`poll`が呼ばれるたびに、現在の状態に応じて処理が進められます。`sleep`を`poll`して`Pending`が返ってくれば、状態を`WaitingForSleep`にして一旦処理を中断します。タイマーが完了してランタイムが再度この`Future`を`poll`すると、`sleep`が`Ready`を返し、次の状態`Done`へと遷移し、`println!("World!")`が実行されます。

---

## 6.4 実践的な非同期パターン with `tokio`

`tokio`は、Rustにおけるデファクトスタンダードな非同期ランタイムであり、本番環境で必要とされる多くの機能を提供します。

### 6.4.1 タスクの並行実行

複数の非同期処理を同時に実行したい場合、`tokio::spawn`を使って新しい非同期タスクを生成します。これはOSのスレッドを生成するよりもはるかに軽量です。

```rust
async fn learn_song() -> String {
    tokio::time::sleep(Duration::from_millis(500)).await;
    "Song learned".to_string()
}

async fn sing_song(song: String) {
    println!("Singing: {}", song);
}

async fn dance() {
    println!("Dancing...");
}

#[tokio::main]
async fn main() {
    // `tokio::join!`は複数のFutureを同時に実行し、全てが完了するのを待つ
    let (song, _) = tokio::join!(
        learn_song(),
        dance()
    );
    
    sing_song(song).await;
}
```
`learn_song`と`dance`は並行して実行されます。`dance`はすぐに完了し、`learn_song`は500ms待機しますが、その間スレッドはブロックされません。

### 6.4.2 タイムアウトとキャンセル

非同期処理は、完了しない可能性があります。`tokio::time::timeout`を使うことで、処理に時間制限を設けることができます。

```rust
use tokio::time::{timeout, Duration};

async fn long_running_task() {
    tokio::time::sleep(Duration::from_secs(5)).await;
}

#[tokio::main]
async fn main() {
    if timeout(Duration::from_secs(2), long_running_task()).await.is_err() {
        println!("Task timed out!");
    }
}
```
Rustでは、`Future`を`drop`（破棄）することが、その処理のキャンセル操作に相当します。`timeout`は、指定時間を超えた場合に内部の`Future`を`drop`することでキャンセルを実現しています。

### 6.4.3 状態の共有

非同期タスク間で状態を共有したい場合、第5章で学んだ`Arc<Mutex<T>>`が使えます。ただし、一つ注意点があります。標準ライブラリの`std::sync::Mutex`は、ロックを待っている間、スレッド全体をブロックしてしまいます。これは非同期の世界では致命的です。

そのため、`tokio`は独自の非同期版`Mutex`を提供しています。

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = tokio::spawn(async move {
            // .await を使って非ブロッキングでロックを獲得
            let mut num = counter_clone.lock().await;
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    println!("Result: {}", *counter.lock().await);
}
```
`lock()`を`.await`することに注意してください。これにより、ロックが利用できない場合、現在のタスクはスレッドをブロックせずに待機状態に入ります。

---

## 6.5 まとめ

本章では、Rustの非同期プログラミングの核心を学びました。

- **非同期はI/Oバウンドなタスクのため:** ネットワークやファイルアクセスなど、CPUが「待ち」状態になる処理を効率化します。
- **`async/await`は状態機械:** コンパイラが`Future`トレイトを実装した状態機械に変換することで、非ブロッキングな待機を実現します。
- **ランタイムが全てを駆動:** `tokio`のような非同期ランタイムが、`Future`のスケジューリングとI/Oイベントの監視を一手に引き受けます。
- **エコシステムが重要:** `tokio`が提供する非同期版の`Mutex`やタイマーなど、非同期対応のツールを使う必要があります。

これで、あなたはRustの提供する主要な並行・非同期モデル（スレッド、メッセージパッシング、データ並列、そして非同期タスク）を全て学びました。これらを適切に使い分けることで、あらゆる種類のタスクに対して、安全で高性能なソリューションを構築できるようになったはずです。