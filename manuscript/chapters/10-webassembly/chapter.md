---
part: "第IV部: ドメイン特化応用編"
page_count: 20
title: "WebAssemblyとクロスプラットフォーム開発"
---

# 第10章: WebAssembly とクロスプラットフォーム開発

## 学習目標
本章を修了すると、以下が可能になります：
- [ ] WebAssembly（Wasm）が解決する問題と、RustがWasm開発に適している理由を説明できる。
- [ ] `wasm-pack`と`wasm-bindgen`を使い、Rustで記述したロジックをJavaScriptから利用できる。
- [ ] WASI（WebAssembly System Interface）の概念を理解し、RustプログラムをサーバーサイドWasmとして実行できる。

---

## 10.1 導入：ブラウザとサーバーのための第4の言語

Webの歴史は、HTML、CSS、そしてJavaScriptという3つの言語を中心に築かれてきました。しかし、Webアプリケーションが複雑化し、より高いパフォーマンスが求められるようになるにつれ、JavaScriptの限界も見えてきました。特に、動的型付けやガベージコレクションによる性能の予測不可能性は、ゲームや画像処理、大規模計算などのCPUバウンドなタスクには不向きな場合があります。

**WebAssembly（Wasm）**は、この問題を解決するために生まれました。Wasmは、ブラウザでネイティブに近い速度で実行できる、ポータブルなバイナリ形式です。これはJavaScriptを置き換えるものではなく、JavaScriptと協調して動作し、パフォーマンスが重要な部分を補強するためのものです。

では、なぜRustがWasm開発の「第一級言語」なのでしょうか？

- **パフォーマンス:** GCを持たず、ゼロコスト抽象化を特徴とするRustは、Wasmの目指すネイティブに近い性能を最大限に引き出せます。
- **軽量なバイナリ:** Rustはシステムランタイムに依存しないため、非常に小さな`.wasm`ファイルを生成でき、ネットワーク経由でのロード時間を短縮できます。
- **安全性:** Rustのメモリ安全性はWasmモジュールにも引き継がれ、サンドボックス化されたWasmのセキュリティモデルをさらに強化します。
- **優れたエコシステム:** `wasm-pack`や`wasm-bindgen`といった成熟したツールチェーンが、開発、ビルド、テスト、デプロイの全工程を強力にサポートします。

本章では、RustとWasmを使って、ブラウザとサーバーサイドの両方で動作するクロスプラットフォームなコンポーネントを構築する方法を学びます。

---

## 10.2 ブラウザで動かすRust：`wasm-bindgen`

Rustのコードをブラウザで動かすための核心的なツールが`wasm-bindgen`です。Wasmの仕様自体は、現状では数値型（整数と浮動小数点数）のやり取りしか定義していません。しかし、実際のアプリケーションでは文字列や構造体、オブジェクトといった、より複雑なデータをJavaScriptとRustの間で受け渡したいはずです。

`wasm-bindgen`は、このギャップを埋めるための「接着剤」の役割を果たします。Rustの型とJavaScriptの型の間で、シリアライズやAPI呼び出しなどの変換を自動的に行ってくれるコードを生成し、まるで魔法のように両者を繋ぎます。

### 10.2.1 ハンズオン：RustからJavaScriptの`alert`を呼び出す

最初のステップとして、RustのコードからブラウザのJavaScript関数（`alert`）を呼び出してみましょう。このハンズオンは`code-examples/chapter-10/browser-hello-world/`にあります。

**1. プロジェクトのセットアップ**

まず、`wasm-pack`をインストールし、新しいライブラリプロジェクトを作成します。
```bash
# wasm-packのインストール
cargo install wasm-pack
# プロジェクト作成
cargo new browser-hello-world --lib
cd browser-hello-world
```

**2. `Cargo.toml`の編集**

`wasm-bindgen`への依存関係と、ライブラリの種類をクレートトップとして指定します。
```toml
[package]
name = "browser-hello-world"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"
```

**3. Rustコードの記述 (`src/lib.rs`)**

`#[wasm_bindgen]`アトリビュートを使って、JavaScriptと連携する部分を記述します。

```rust
use wasm_bindgen::prelude::*;

// `window.alert`関数をインポート
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// JavaScriptに公開する`greet`関数を定義
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
```

**4. ビルド**

`wasm-pack`を使ってビルドします。これにより、`.wasm`ファイルと、それを読み込むためのJavaScriptのグルーコード（接着剤コード）が`pkg`ディレクトリに生成されます。

```bash
wasm-pack build --target web
```

**5. HTMLから利用**

最後に、生成されたJavaScriptモジュールを読み込むHTMLファイルを作成します。

```html
<!-- index.html -->
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Hello wasm-pack!</title>
</head>
<body>
    <script type="module">
        // pkg/browser_hello_world.js をインポート
        import init, { greet } from './pkg/browser_hello_world.js';

        async function run() {
            // Wasmモジュールの初期化
            await init();
            // Rustのgreet関数を呼び出し
            greet("WebAssembly");
        }

        run();
    </script>
</body>
</html>
```

この`index.html`をブラウザで開くと、「Hello, WebAssembly!」というアラートが表示されるはずです。これで、RustとJavaScriptの連携が成功しました。

---

## 10.3 ブラウザの外へ：WASI (WebAssembly System Interface)

Wasmのポテンシャルはブラウザに留まりません。もし、Wasmの持つ「ポータブル」で「安全なサンドボックス」という特性を、サーバーサイドやプラグインシステムで利用できたらどうでしょうか？そのための標準仕様が**WASI (WebAssembly System Interface)**です。

WASIは、ファイルシステム、環境変数、ソケット、時計といった、OSのシステム機能をWasmモジュールから呼び出すための標準的なインターフェースを定義します。これにより、Wasmバイナリは、特定のOSやCPUアーキテクチャに依存しない、真にポータブルな実行ファイルとなります。

重要なのは、WASIが**Capability-based Security（権能ベースのセキュリティ）**モデルを採用している点です。Wasmモジュールは、デフォルトでは外部の世界に一切アクセスできません。ファイルを読む、ネットワークに接続するといった権能は、Wasmを実行する**ランタイム**（`wasmtime`など）が、明示的に許可したものに限られます。これにより、信頼できないコードを安全に実行するための強力なサンドボックスが実現します。

### 10.3.1 ハンズオン：WASIでファイルI/Oを行う

標準ライブラリのファイルI/O機能を使った簡単なRustプログラムを、WASIターゲットにコンパイルして実行してみましょう。このハンズオンは`code-examples/chapter-10/wasi-file-io/`にあります。

**1. Rustコードの記述 (`src/main.rs`)**

ごく普通のRustプログラムです。コマンドライン引数でファイルパスを受け取り、その内容を標準出力に書き出します。

```rust
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <file>", args[0]);
        process::exit(1);
    }

    let content = fs::read_to_string(&args[1])
        .expect("could not read file");

    println!("File content:\n{}", content);
}
```

**2. WASIターゲットへのコンパイル**

Rustコンパイラは、WASIを第一級のターゲットとしてサポートしています。

```bash
# wasm32-wasiターゲットを追加
rustup target add wasm32-wasi
# コンパイル
cargo build --target wasm32-wasi
```

これにより、`target/wasm32-wasi/debug/wasi-file-io.wasm`が生成されます。

**3. Wasmランタイムでの実行**

`wasmtime`のようなWASI互換ランタイムを使って、生成された`.wasm`ファイルを実行します。

```bash
# wasmtimeをインストール
# curl https://wasmtime.dev/install.sh -sSf | bash

# テスト用のファイルを作成
echo "Hello from a text file!" > test.txt

# wasmtimeで実行。--mapdirでホストのディレクトリをゲストに公開
wasmtime run \
  --mapdir /local::. \
  target/wasm32-wasi/debug/wasi-file-io.wasm \
  /local/test.txt
```

`--mapdir /local::.`というオプションに注目してください。これは、ホストのカレントディレクトリ（`.`）を、Wasmゲスト内の`/local`というパスにマッピング（公開）するという意味です。Wasmモジュールは、この明示的に許可されたディレクトリ以外には一切アクセスできません。これがWASIのサンドボックス能力の核心です。

---

## 10.4 まとめ

本章では、RustとWebAssemblyが切り拓く、新しいクロスプラットフォーム開発の世界を探求しました。

-   **ブラウザ向け (`wasm-bindgen`):** パフォーマンスが重要な計算処理をRustで記述し、JavaScriptと協調させることで、高速でリッチなWebアプリケーションを構築できます。
-   **サーバーサイド向け (WASI):** Rustプログラムを、ポータブルで安全なサンドボックス化されたWasmモジュールとしてコンパイルし、サーバー、エッジ、プラグインシステムなど、様々な環境で実行できます。

Rustの持つ安全性、パフォーマンス、そして優れたツールチェーンは、WebAssemblyのポテンシャルを最大限に引き出すための理想的な組み合わせです。次章では、組み込みシステムという、さらにリソースが制約された環境でRustがどのように活躍するかを見ていきます。