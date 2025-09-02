# 第8章: パフォーマンス向上の理論と実践

## 学習目標
本章を修了すると、以下が可能になります：
- [ ] 「推測するな、計測せよ」という原則に基づき、パフォーマンス改善に取り組める。
- [ ] `perf`とFlameGraphを使い、アプリケーションのCPUボトルネックを特定できる。
- [ ] `criterion`を使い、信頼性の高いマイクロベンチマークを作成できる。
- [ ] データレイアウトを最適化（AoS vs SoA）し、キャッシュ効率を改善できる。
- [ ] SIMDを使い、データ並列な計算を高速化できる。

---

## 8.1 導入：パフォーマンスは科学である

第1章と第2章で、現代のハードウェア性能がCPUのクロック周波数だけでなく、メモリ階層やデータアクセスパターンに大きく依存することを学びました。パフォーマンスチューニングとは、プログラマの直感や「速そうなコード」を書くことではありません。それは、仮説を立て、計測を行い、ボトルネックを特定し、改善策を施し、そして再度計測するという、科学的なプロセスです。

このプロセスにおける絶対的な黄金律は**「推測するな、計測せよ (Measure, don't guess)」**です。人間の直感は、複雑なコンパイラの最適化やCPUの内部動作の前ではしばしば間違っています。ある変更が本当にパフォーマンスを改善するかどうかを知る唯一の方法は、信頼できるツールで計測することです。

本章では、まずそのためのツール（プロファイラとベンチマークライブラリ）を紹介し、その後で、特定されたボトルネックを解消するための具体的なコーディングテクニックを探求します。

---

## 8.2 計測：プロファイリングとベンチマーキング

### 8.2.1 マクロプロファイリング：`perf` と FlameGraph

アプリケーション全体のどこでCPU時間が最も消費されているか、という全体像を把握するためには、**プロファイラ**を使います。Linux環境では、`perf`が標準的で強力なツールです。

`perf`は、プログラムの実行中にCPUのパフォーマンスカウンタをサンプリングし、どの関数が最も頻繁に実行されていたかを記録します。そして、その結果を**FlameGraph**というツールで可視化することで、ボトルネックを直感的に特定できます。

**実践手順 (Linux):**

1.  **デバッグ情報付きでビルド:** `Cargo.toml`に以下を追加して、リリースビルドでもデバッグシンボルが含まれるようにします。
    ```toml
    [profile.release]
    debug = true
    ```

2.  **`perf`でデータを記録:**
    ```bash
    # `perf`をインストール: sudo apt-get install linux-tools-common linux-tools-generic
    perf record --call-graph dwarf target/release/your_program
    ```

3.  **FlameGraphを生成:**
    ```bash
    # FlameGraphツールをクローン
    # git clone https://github.com/brendangregg/FlameGraph.git
    perf script | FlameGraph/flamegraph.pl > profile.svg
    ```

生成された`profile.svg`をブラウザで開くと、インタラクティブなグラフが表示されます。グラフの横幅が広い関数ほど、CPU時間を多く消費していることを意味します。これにより、最適化すべき「ホットスポット」が一目瞭然となります。

### 8.2.2 マイクロベンチマーキング：`criterion`

特定の関数のパフォーマンスを精密に測定し、改善の効果を比較するためには、**ベンチマークライブラリ**を使います。Rustのデファクトスタンダードは`criterion`です。

`criterion`は、統計的な手法を用いて、OSのスケジューリングやキャッシュの状態といった外部ノイズの影響を排し、信頼性の高い測定結果を提供します。

```rust
// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    // `fibonacci(20)`のパフォーマンスを計測
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
```
`cargo bench`で実行すると、詳細な分析レポートが生成されます。`black_box`は、コンパイラが計算を最適化しすぎてベンチマーク自体を削除してしまうのを防ぐために重要です。

---

## 8.3 最適化①：CPUキャッシュ効率の改善

プロファイリングによってデータ処理部分がボトルネックだと特定された場合、まず疑うべきはCPUキャッシュの効率です。第2章で学んだように、データがメモリ上でどのように配置されているか（データレイアウト）が、キャッシュヒット率に絶大な影響を与えます。

### 8.3.1 AoS vs SoA (Array of Structures vs Structure of Arrays)

- **AoS (Array of Structures):** 構造体の配列。オブジェクト指向プログラミングでは自然な形。
  ```rust
  struct Point { x: f64, y: f64, z: f64 }
  let points: Vec<Point> = vec![...];
  ```
- **SoA (Structure of Arrays):** 構造体の各フィールドを、それぞれ別の配列として持つ。
  ```rust
  struct Points {
      xs: Vec<f64>,
      ys: Vec<f64>,
      zs: Vec<f64>,
  }
  ```

**どちらを選ぶべきか？** それは、データへのアクセスパターンによります。

**シナリオ:** 全ての点の`x`座標の合計値を計算する。

- **AoSの場合:** `points`ベクタをイテレートすると、メモリから`Point`構造体全体（24バイト）がキャッシュに読み込まれます。しかし、実際に必要なのは`x`（8バイト）だけで、`y`と`z`（16バイト）はキャッシュの無駄遣いです。これを**キャッシュ汚染**と呼びます。
- **SoAの場合:** `xs`ベクタをイテレートすると、必要な`x`座標のデータだけがメモリ上で連続して並んでいるため、キャッシュラインは有用なデータで満たされます。キャッシュ効率は最大化されます。

```rust
// ベンチマークで比較
fn sum_x_aos(points: &[Point]) -> f64 {
    points.iter().map(|p| p.x).sum()
}

fn sum_x_soa(points: &Points) -> f64 {
    points.xs.iter().sum()
}
```
このベンチマークを実行すれば、`sum_x_soa`が`sum_x_aos`よりも大幅に高速であることが確認できるはずです。データ指向設計の基本は、処理内容に合わせたデータレイアウトを選択することにあります。

---

## 8.4 最適化②：データ並列性の活用 (SIMD)

もう一つの強力な最適化手法が**SIMD (Single Instruction, Multiple Data)**です。これは、CPUが持つ特殊なレジスタ（128〜512ビット）を使い、一つの命令で複数のデータ（例：4つの`f32`値）に対して同じ演算（例：加算）を同時に行う技術です。

### 8.4.1 自動ベクトル化

幸いなことに、多くの場合、コンパイラ（LLVM）がループ処理を自動的にSIMD命令に変換（**自動ベクトル化**）してくれます。コンパイラが最適化しやすいように、単純なループ構造を保つことが重要です。

### 8.4.2 明示的なSIMD

より確実にSIMDの恩恵を受けたい場合、`std::simd`モジュール（現在nightly）や`packed_simd_2`のようなクレートを使い、明示的にSIMD演算を記述できます。

```rust
// nightly toolchainが必要
use std::simd::{f32x4, Simd};

fn simd_add(a: &[f32], b: &[f32], c: &mut [f32]) {
    assert_eq!(a.len(), b.len());
    assert_eq!(a.len(), c.len());

    let chunks_a = a.chunks_exact(4);
    let chunks_b = b.chunks_exact(4);
    let chunks_c = c.chunks_exact_mut(4);

    for ((a, b), c) in chunks_a.zip(chunks_b).zip(chunks_c) {
        // 4つのf32を一度にロード
        let simd_a = f32x4::from_slice(a);
        let simd_b = f32x4::from_slice(b);
        // 4つのf32を一度に加算
        let simd_c = simd_a + simd_b;
        // 4つのf32を一度にストア
        simd_c.write_to_slice(c);
    }
    // 端数処理は省略
}
```
グラフィックス処理や科学技術計算など、大量のデータを一括で処理するような場面では、SIMDは数倍のパフォーマンス向上をもたらす可能性があります。

---

## 8.5 まとめ

本章では、パフォーマンス改善のための科学的なアプローチを学びました。

1.  **計測から始める:** `perf`とFlameGraphでマクロなボトルネックを、`criterion`でミクロな関数性能を特定します。推測で最適化を始めてはいけません。
2.  **データレイアウトを最適化する:** プロファイリングの結果、メモリアクセスがボトルネックだと判明したら、AoSからSoAへの変換など、キャッシュ効率を意識したデータレイアウトの変更を検討します。
3.  **計算の並列度を上げる:** 計算処理自体がボトルネックなら、SIMDによるデータレベルの並列化が有効な手段となり得ます。

パフォーマンスエンジニアリングは、銀の弾丸を探す作業ではありません。地道な計測と、ハードウェアの動作原理に基づいた論理的な改善を繰り返す、科学的な探求なのです。