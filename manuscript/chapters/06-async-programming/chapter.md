# 第6章: 非同期プログラミング - Future理論の実践

## 学習目標
この章を読み終えると、以下ができるようになります：
- [ ] 同期、並行、非同期の違いを理解し、非同期プログラミングが解決する問題を説明できる。
- [ ] `async/await`が、コンパイラによってどのように状態機械に変換されるかを概念的に理解できる。
- [ ] `Future`トレイトが非同期処理の基本単位であることを理解できる。
- [ ] `tokio`ランタイムを使い、実践的な非同期アプリケーション（ネットワークサービスなど）を構築できる。

---

## 6.1 導入：なぜ「非同期」なのか？

第5章では、CPUバウンドなタスクを複数のコアで並列実行することで高速化する方法を学びました。しかし、現代のアプリケーション、特にネットワークサービスでは、CPUが計算している時間よりも、**I/O（ディスクやネットワーク）の応答を待っている時間**の方がはるかに長いことがよくあります。

伝統的な「スレッドプール」モデルでは、リクエストごとにOSのスレッドを一つ割り当てます。しかし、1万のクライアントが同時に接続する状況（C10k問題）を考えると、1万のスレッドを作成・管理するのは、OSにとって非常に大きなオーバーヘッドとなります。ほとんどのスレッドは、ただI/Oを待っているだけでCPUを消費しないにも関わらず、メモリやコンテキストスイッチのコストを発生させます。

**CSのポイント:** C10k問題は、CSの**「スケーラビリティ」**における古典的な課題です。多数の同時接続を効率的に処理するためには、OSスレッドのような重いリソースに依存せず、I/Oの完了を待つ間に他の処理を進めるメカニズムが必要です。

**非同期プログラミング**は、この問題を解決します。単一のスレッド（あるいは少数のスレッド）で、何千ものI/Oバウンドなタスクを効率的に管理する仕組みです。タスクがI/Oでブロックされる（待機状態になる）場合、そのタスクを一時停止し、スレッドは別の実行可能なタスクの処理に移ります。これにより、スレッドは常に「仕事をしている」状態を保ち、システムリソースを最大限に活用できるのです。

`[図：同期I/Oと非同期I/Oの比較。同期I/OではスレッドがI/O完了までブロックされる様子、非同期I/OではスレッドがI/O完了を待たずに他のタスクに切り替える様子をタイムラインで示す。]`

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

**CSのポイント:** `async/await`は、CSの**「コルーチン（Coroutine）」**や**「協調的マルチタスク（Cooperative Multitasking）」**の概念を言語レベルでサポートするものです。OSのスケジューラに依存するプリエンプティブなスレッドとは異なり、コルーチンは明示的な`.await`ポイントで実行を中断し、制御をランタイムに返します。これにより、コンテキストスイッチのオーバーヘッドが低減され、多数のタスクを単一スレッドで効率的に処理できます。

---

## 6.3 非同期の仕組み：Future、状態機械、そしてランタイム

`async/await`は魔法ではありません。コンパイラと**非同期ランタイム**（`tokio`など）の協調によって実現されています。

### 6.3.1 `Future`トレイト

全ての非同期処理の基本単位が`Future`トレイトです。これは、将来のある時点で完了するかもしれない値を表現します。その定義は本質的に以下のようになっています。

```rust
pub trait Future {
    type Output;
    // `poll`メソッドは、Futureの進行を試みる
    // `cx`は、FutureがPendingを返した場合に、完了時に通知を受け取るためのWakerを含む
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T), // Futureが完了し、値Tを生成した
    Pending,  // Futureはまだ完了していない
}
```
**CSのポイント:** `Future`トレイトの`poll`メソッドは、CSの**「継続（Continuation）」**の概念と密接に関連しています。`poll`が`Pending`を返すことは、現在の計算が一時停止され、後で再開されるべき「継続」がランタイムに渡されたことを意味します。

### 6.3.2 非同期ランタイム（Executor）

**非同期ランタイム**（あるいはExecutor）の仕事は、たくさんの`Future`を管理し、それらを完了まで駆動することです。ランタイムは、以下のようなループを実行します。

1.  管理している`Future`の一つを取り出し、`poll`メソッドを呼び出す。
2.  `poll`が`Poll::Ready(value)`を返せば、その`Future`は完了。結果を次の処理に渡す。
3.  `poll`が`Poll::Pending`を返せば、その`Future`はまだ完了していない。ランタイムはそれを一旦脇に置き、別の`Future`の`poll`に移る。この際、`Waker`を使って、I/Oが完了した際にランタイムに通知が来るように登録する。
4.  I/O（ネットワーク、タイマーなど）の準備ができたというOSからの通知を受け取ると、そのI/Oを待っていた`Future`を「実行可能」な状態に戻し、再び`poll`の対象にする。

**CSのポイント:** 非同期ランタイムは、CSの**「イベントループ（Event Loop）」**や**「スケジューラ（Scheduler）」**として機能します。OSのI/O多重化API（Linuxの`epoll`、macOS/BSDの`kqueue`、WindowsのIOCPなど）を利用して、多数のI/Oイベントを効率的に監視し、完了したI/Oに対応するタスクを再開します。これにより、単一のスレッドで多数の非同期タスクを効率的に処理する、CSの**「ノンブロッキングI/O」**を実現します。

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
    Start, // 初期状態
    WaitingForSleep(Pin<Box<dyn Future<Output = ()>>>), // sleepの完了を待っている状態
    Done, // 完了状態
}

impl Future for SayHelloState {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // selfを可変参照としてピン留めし、内部の状態にアクセス
        let mut this = self.project(); 

        match this.state {
            SayHelloState::Start => {
                println!("Hello");
                // sleep Futureを生成し、状態を更新
                let sleep_future = Box::pin(tokio::time::sleep(Duration::from_secs(1)));
                *this.state = SayHelloState::WaitingForSleep(sleep_future);
                // 再度pollを呼び出す（またはランタイムに登録）
                cx.waker().wake_by_ref(); // すぐに再pollを要求
                Poll::Pending
            }
            SayHelloState::WaitingForSleep(sleep_future) => {
                // sleep Futureをpollし、完了を待つ
                match sleep_future.as_mut().poll(cx) {
                    Poll::Ready(_) => {
                        println!("World!");
                        *this.state = SayHelloState::Done;
                        Poll::Ready(())
                    }
                    Poll::Pending => Poll::Pending,
                }
            }
            SayHelloState::Done => {
                // 既に完了しているので、何もせずReadyを返す
                Poll::Ready(())
            }
        }
    }
}
```
`poll`が呼ばれるたびに、現在の状態に応じて処理が進められます。`sleep`を`poll`して`Pending`が返ってくれば、状態を`WaitingForSleep`にして一旦処理を中断します。タイマーが完了してランタイムが再度この`Future`を`poll`すると、`sleep`が`Ready`を返し、次の状態`Done`へと遷移し、`println!("World!")`が実行されます。

`[図：async/await関数の状態機械への変換。`async fn`内の各`.await`ポイントが、状態機械の異なる状態に対応し、`poll`が呼ばれるたびに状態が遷移する様子をフローチャートまたは状態遷移図で示す。]`

---

## 6.4 実践的な非同期パターン with `tokio`

`tokio`は、Rustにおけるデファクトスタンダードな非同期ランタイムであり、本番環境で必要とされる多くの機能を提供します。

### 6.4.1 タスクの並行実行

複数の非同期処理を同時に実行したい場合、`tokio::spawn`を使って新しい非同期タスクを生成します。これはOSのスレッドを生成するよりもはるかに軽量です。

```rust
use tokio::time::Duration;

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

**CSのポイント:** `tokio::spawn`で生成されるタスクは, CSの**「軽量プロセス（Lightweight Process）」**や**「グリーンタスク（Green Task）」**に相当します。これらはOSのスケジューラではなく、ランタイムのスケジューラによって管理されるため、OSスレッドに比べてコンテキストスイッチのコストがはるかに低く、数万〜数十万のタスクを単一のスレッド上で効率的に実行できます。

### 6.4.2 タイムアウトとキャンセル

非同期処理は、完了しない可能性があります。`tokio::time::timeout`を使うことで、処理に時間制限を設けることができます。

```rust
use tokio::time::{timeout, Duration};

async fn long_running_task() {
    tokio::time::sleep(Duration::from_secs(5)).await;
    println!("Task completed!"); // タイムアウトすると表示されない
}

#[tokio::main]
async fn main() {
    println!("Starting task with 2-second timeout...");
    if timeout(Duration::from_secs(2), long_running_task()).await.is_err() {
        println!("Task timed out!");
    }
    println!("Main function finished.");
}
```
Rustでは、`Future`を`drop`（破棄）することが、その処理のキャンセル操作に相当します。`timeout`は、指定時間を超えた場合に内部の`Future`を`drop`することでキャンセルを実現しています。

**CSのポイント:** 非同期タスクのキャンセルは、CSの**「並行性制御」**における重要な課題です。Rustの`Future`の`drop`によるキャンセルは、CSの**「協調的キャンセル」**の一形態であり、リソースリークを防ぎつつ安全にタスクを終了させるためのメカニズムです。

### 6.4.3 状態の共有

非同期タスク間で状態を共有したい場合、第5章で学んだ`Arc<Mutex<T>>`が使えます。ただし、一つ注意点があります。標準ライブラリの`std::sync::Mutex`は、ロックを待っている間、スレッド全体をブロックしてしまいます。これは非同期の世界では致命的です。

そのため、`tokio`は独自の非同期版`Mutex`を提供しています。

```rust
use std::sync::Arc;
use tokio::sync::Mutex; // tokio::sync::Mutex を使う

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

**CSのポイント:** `tokio::sync::Mutex`のような非同期版の同期プリミティブは、CSの**「ノンブロッキング同期」**の概念を実装しています。標準ライブラリの`Mutex`がOSのスケジューラにスレッドをブロックさせるのに対し、非同期版`Mutex`は、ロックが利用可能になるまで現在のタスクを一時停止させ、ランタイムがその間に他のタスクを実行できるようにします。これにより、単一スレッド上での並行性を最大限に高めます。

---

## 6.5 まとめ

本章では、Rustの非同期プログラミングの核心を学びました。

- [ ] **非同期はI/Oバウンドなタスクのため:** ネットワークやファイルアクセスなど、CPUが「待ち」状態になる処理を効率化します。CSのI/O多重化やイベント駆動型プログラミングに基づきます。
- [ ] **`async/await`は状態機械:** コンパイラが`Future`トレイトを実装した状態機械に変換することで、非ブロッキングな待機を実現します。CSのコルーチンや協調的マルチタスクに基づきます。
- [ ] **ランタイムが全てを駆動:** `tokio`のような非同期ランタイムが、`Future`のスケジューリングとI/Oイベントの監視を一手に引き受けます。CSのイベントループやノンブロッキングI/Oに基づきます。
- [ ] **エコシステムが重要:** `tokio`が提供する非同期版の`Mutex`やタイマーなど、非同期対応のツールを使う必要があります。CSのノンブロッキング同期に基づきます。

これで、あなたはRustの提供する主要な並行・非同期モデル（スレッド、メッセージパッシング、データ並列、そして非同期タスク）を全て学びました。これらを適切に使い分けることで、あらゆる種類のタスクに対して、安全で高性能なソリューションを構築できるようになったはずです.
