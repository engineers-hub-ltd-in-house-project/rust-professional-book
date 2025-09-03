---
part: "第III部: システムプログラミング編"
page_count: 30
title: "OS概念とRustシステムプログラミング"
---

# 第7章: OS概念とRustシステムプログラミング

## 学習目標
この章を読み終えると、以下ができるようになります：
- [ ] Rustの標準ライブラリが、OS機能の安全な抽象化であることを理解できる。
- [ ] FFI（Foreign Function Interface）を使い、C言語で書かれたOSのAPIを呼び出せる。
- [ ] RAIIパターンを使い、`unsafe`なシステムコールを安全なRustのAPIにラップできる。
- [ ] プロセス間通信（IPC）やシグナルハンドリングといった、代表的なシステムプログラミングのタスクを、安全な方法で実現できる。

---

## 7.1 導入：標準ライブラリの裏側

これまで私たちは、`std::fs::File`や`std::thread`、`std::net::TcpListener`など、Rustの標準ライブラリが提供する便利な機能を使ってきました。これらは全て、OSが提供する低レベルな機能（システムコール）の、プラットフォームの違いを吸収した**安全な抽象化**です。

**CSのポイント:** Rustの標準ライブラリは、CSの**「オペレーティングシステム」**における**「システムコールインターフェース」**の概念を、高レベルかつ安全な形で提供しています。これにより、開発者はOSの複雑な詳細を意識することなく、ファイル操作やネットワーク通信といった基本的なシステム機能を利用できます。

しかし、時には標準ライブラリが提供しない、より低レベルなOSの機能を使いたい場合があります。例えば、特定のファイルディスクリプタを操作したり、プロセスにシグナルを送ったり、特殊なIPCメカニズムを使ったりする場合です。

このような時、Rustは**FFI (Foreign Function Interface)** という仕組みを通じて、C言語で書かれたライブラリ（OSのAPIは通常C言語形式で提供される）を直接呼び出す能力を提供します。しかし、CのAPIを呼び出すことは、Rustが保証してきたメモリ安全性を一時的に手放すことを意味します。なぜなら、Cのコードに対してはRustコンパイラの借用検査器は無力だからです。

本章のテーマは、これらの`unsafe`な操作を、いかにして局所化し、**安全なラッパー**に包み込むことで、アプリケーション全体としての安全性を維持するか、というRustのシステムプログラミングにおける中心的な設計思想を学ぶことです。

---

## 7.2 `unsafe`な世界への扉：FFIとシステムコール

RustからOSの機能にアクセスする最も直接的な方法は、`libc`クレートを通じてC言語の標準ライブラリ関数を呼び出すことです。

```rust
// Cargo.toml に以下を追加: libc = "0.2"

fn get_process_id() -> i32 {
    // Cのgetpid()関数を呼び出す。この呼び出しはunsafeブロックで囲む必要がある
    unsafe {
        libc::getpid()
    }
}

fn main() {
    println!("My process ID is {}", get_process_id());
}
```

`unsafe`キーワードは、コンパイラに対して「ここから先のコードは、コンパイラには安全性を検証できない。プログラマである私がその安全性を保証する」と宣言するものです。`getpid()`のような単純な関数は安全に呼び出せますが、ポインタやリソースを扱う関数は、細心の注意が必要になります。

**CSのポイント:** FFIは、CSの**「言語間連携」**や**「バイナリ互換性」**における重要なメカニズムです。異なるプログラミング言語で書かれたコードが、互いの関数やデータ構造を呼び出し、利用することを可能にします。しかし、Rustの`unsafe`ブロックが示すように、FFIの境界を越えることは、Rustの強力なメモリ安全保証を一時的に無効化することを意味します。これは、CSの**「メモリモデル」**や**「コンパイラ」**が持つ不変条件が、言語の境界を越えると適用されなくなるためです。

### 7.2.1 RAIIによる安全なリソース管理

システムプログラミングで最も一般的なパターンは、OSからリソース（ファイルディスクリプタ、ソケット、メモり領域など）を取得し、使い終わったら必ず解放することです。C言語では手動での解放が必須であり、解放忘れによるリソースリークは頻繁に起こるバグでした。

Rustでは、**RAII (Resource Acquisition Is Initialization)** パターンと`Drop`トレイトを使って、この問題を根絶します。`unsafe`なリソース取得処理を、`Drop`を実装した構造体（struct）でラップするのです。

例として、低レベルな`open`と`close`システムコールを安全な`File`構造体でラップしてみましょう。

```rust
use std::ffi::CString;
use std::io;

// 生のファイルディスクリプタを保持する構造体
pub struct File {
    fd: i32,
}

impl File {
    pub fn open(path: &str) -> io::Result<File> {
        let c_path = CString::new(path).unwrap();
        let fd = unsafe {
            // Cのopen関数を呼び出す
            libc::open(c_path.as_ptr(), libc::O_RDWR | libc::O_CREAT)
        };

        if fd < 0 {
            // エラーが発生した場合
            Err(io::Error::last_os_error())
        } else {
            Ok(File { fd })
        }
    }
}

// File構造体がスコープを抜けるときに、自動的にcloseが呼ばれる
impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            // Cのclose関数を呼び出す
            if libc::close(self.fd) < 0 {
                // drop処理中のエラーはpanicするしかない場合が多い
                eprintln!("Failed to close file descriptor: {}", self.fd);
            }
        }
    }
}
```
この`File`構造体を使う限り、開発者はファイルディスクリプタの解放を意識する必要は一切ありません。`unsafe`な処理は`File`の実装内部に完全にカプセル化され、外部には安全なAPIのみが公開されています。これこそが、Rustにおけるゼロコスト抽象化の神髄です。

**CSのポイント:** RAIIは、CSの**「リソース管理」**における強力な設計パターンです。リソースのライフサイクルをオブジェクトのライフサイクルに結びつけることで、リソースリークや二重解放といった問題を自動的に防ぎます。Rustの`Drop`トレイトは、このRAIIパターンを言語レベルで強制し、`unsafe`な低レベル操作を安全な高レベル抽象化に変換する鍵となります。

`[図：unsafeブロックがRustの安全なコードとC言語のAPIの境界をどのように形成するか。RAIIパターンでunsafeなリソース取得を安全なRustの型でラップし、Dropトレイトで自動的にリソースが解放される様子を図解する。]`

---

## 7.3 プロセス間通信（IPC）

次に、異なるプロセス間で通信するためのいくつかの一般的な方法を、Rustで安全に扱う方法を見ていきましょう。

**CSのポイント:** IPC（Inter-Process Communication）は、CSの**「オペレーティングシステム」**における**「プロセス管理」**の重要な側面です。独立したプロセス間でデータを交換し、協調動作を行うためのメカニズムを提供します。

### 7.3.1 パイプ

最も古典的でシンプルなIPCは、親プロセスと子プロセスの間で標準入出力を通じて通信するパイプです。Rustの`std::process::Command`は、これを非常に簡単かつ安全に扱うための高レベルなAPIを提供します。

```rust
use std::process::{Command, Stdio};
use std::io::{Write};

fn main() -> std::io::Result<()> {
    let mut child = Command::new("grep")
        .arg("Rust")
        .stdin(Stdio::piped()) // 子プロセスの標準入力をパイプに設定
        .stdout(Stdio::piped()) // 子プロセスの標準出力をパイプに設定
        .spawn()?;

    // 子プロセスの標準入力に書き込む
    let stdin = child.stdin.as_mut().unwrap();
    stdin.write_all(b"Hello, Rust!\nThis is a test.\nRust is awesome.")?;

    // 子プロセスの完了を待ち、出力を取得
    let output = child.wait_with_output()?;

    println!("Grep output:\n{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}
```
**CSのポイント:** パイプは、CSの**「ストリーム」**モデルに基づいた、単方向のデータフローを持つIPCメカニズムです。通常、親プロセスと子プロセスの間でデータをやり取りするために使用され、OSによって管理されるバッファを介して通信します。

### 7.3.2 Unixドメインソケット

ファイルシステムのパスを使って通信するUnixドメインソケットも、一般的なIPCメカニズムです。`std::os::unix::net`モジュール（Unix系OSでのみ利用可能）が、TCPソケットとほぼ同じAPIでこれを提供します。

```rust
// Unix系OSでのみ動作
#[cfg(unix)]
fn main() -> std::io::Result<()> {
    use std::os::unix::net::{UnixListener, UnixStream};
    use std::thread;
    use std::io::{Read, Write};

    let socket_path = "/tmp/rust-ipc-socket";
    let _ = std::fs::remove_file(socket_path); // 事前に古いソケットファイルを削除

    let listener = UnixListener::bind(socket_path)?;

    // クライアントからの接続を待つスレッド
    let server_handle = thread::spawn(move || -> std::io::Result<()> {
        let (mut stream, _) = listener.accept()?;
        let mut response = String::new();
        stream.read_to_string(&mut response)?;
        println!("Server received: {}", response);
        Ok(())
    });

    // サーバーに接続するクライアント
    let mut stream = UnixStream::connect(socket_path)?;
    stream.write_all(b"Hello from client")?;

    server_handle.join().unwrap()?;
    Ok(())
}

#[cfg(not(unix)]
fn main() {
    println!("This example only runs on Unix-like systems.");
}
```
**CSのポイント:** Unixドメインソケットは、CSの**「ネットワークプログラミング」**におけるソケットAPIを、同一ホスト内のプロセス間通信に特化させたものです。TCP/IPソケットと比較して、ネットワークスタックを介さないためオーバーヘッドが低く、ファイルシステム上のパスで識別されるため、アクセス制御が容易という特徴があります。

`[図：パイプやUnixドメインソケットを通じたプロセス間のデータフロー。プロセスAとプロセスBが、OSが提供するIPCメカニズムを介して通信する様子を図解する。]`

---

## 7.4 シグナルハンドリング

プロセスは、OSから非同期に通知（シグナル）を受け取ることがあります。例えば、ユーザーがCtrl-Cを押したときの`SIGINT`や、`kill`コマンドによる`SIGTERM`です。

シグナルハンドリングは、システムプログラミングの中でも特に危険で難しい領域です。シグナルハンドラ関数内で呼び出して良い関数（非同期シグナル安全な関数）は極めて限定されており、`malloc`や`println!`すら安全ではありません。

**CSのポイント:** シグナルは、CSの**「オペレーティングシステム」**における**「非同期イベント通知」**のメカニズムです。ハードウェア割り込みやソフトウェアイベント（例：ゼロ除算、セグメンテーション違反）によって発生し、プロセスの実行を中断して特定のハンドラ関数を実行させます。シグナルハンドラが**「非同期シグナル安全（Async-Signal-Safe）」**である必要があるのは、CSの**「再入可能性（Reentrancy）」**の原則に基づきます。つまり、ハンドラが実行中に、同じハンドラが再度呼び出されても問題なく動作しなければなりません。

このような危険な処理を自前で`unsafe`を使って実装するのは、専門家でも困難です。幸いなことに、Rustのエコシステムには、これを安全に扱うための優れたクレートが存在します。代表的なものが`signal-hook`です。

```rust
// Cargo.toml に以下を追加: signal-hook = "0.3"
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let term = Arc::new(AtomicBool::new(false));
    // `term`をクローンして、シグナルハンドラに渡す
    // Ctrl-Cを受け取ったら、`term`をtrueにセットする
    signal_hook::flag::register(SIGINT, Arc::clone(&term))?;

    while !term.load(Ordering::Relaxed) {
        println!("Working...");
        thread::sleep(Duration::from_secs(1));
    }

    println!("Received SIGINT, shutting down gracefully.");
    Ok(())
}
```
この例では、`unsafe`なコードを一切書くことなく、アトミックなフラグを使って安全にCtrl-C（`SIGINT`）を補足し、優雅なシャットダウンを実現しています。**複雑で危険な低レベル処理は、信頼できるクレートに任せる**。これもまた、Rustにおける重要な安全設計の思想です。

---

## 7.5 まとめ

本章では、RustがOSの低レベルな世界とどのように関わるかを学びました。

- [ ] **FFIは`unsafe`な扉:** C言語のAPIを呼び出すことでOSの機能に直接アクセスできますが、それはRustの安全神話の外側です。
- [ ] **RAIIは安全な鎧:** `unsafe`なリソース取得処理を、`Drop`トレイトを実装した構造体でラップすることが、安全な抽象化を構築する鍵です。
- [ ] **高レベルAPIを優先:** `std::process`のように、標準ライブラリが提供する安全な高レベルAPIがある場合は、常にそちらを使うべきです。
- [ ] **エコシステムを信頼する:** シグナルハンドリングのような特に危険な領域では、自前で実装するのではなく、コミュニティによって検証された安全なクレートを利用することが賢明です。

Rustは、あなたを低レベルの世界から遠ざける言語ではありません。むしろ、その危険な世界を安全に冒険するための、最高のツールキットを提供してくれる言語なのです。
