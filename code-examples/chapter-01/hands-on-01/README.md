# ハンズオン 01: 言語間パフォーマンス比較

## 概要

同一アルゴリズム（フィボナッチ数列）を複数言語で実装し、パフォーマンスを比較します。

## 実装言語

- Rust
- Java
- Python
- C

## 実行方法

### Rust
```bash
cd rust
cargo build --release
cargo run --release
```

### Java
```bash
cd java
javac Benchmark.java
java Benchmark
```

### Python
```bash
cd python
python3 benchmark.py
```

### C
```bash
cd c
gcc -O3 -o benchmark benchmark.c
./benchmark
```

## 比較項目

1. **実行時間**: 各アルゴリズムの実行速度
2. **メモリ使用量**: プロセスが使用するメモリ
3. **コンパイル時間**: （該当する場合）
4. **コードの簡潔性**: 実装の行数と可読性

## 学習目標

- 各言語の性能特性を理解する
- Rustのゼロコスト抽象化を実感する
- 適切な言語選択の判断基準を確立する