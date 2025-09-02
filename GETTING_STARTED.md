# 執筆開始ガイド

## 1. 環境構築

### 必要なツール
```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Python (ツール用)
python3 --version  # 3.8以上

# その他言語（比較用）
java -version      # Java 11以上
g++ --version      # C++17対応
```

## 2. 第1章のハンズオン実行

### Step 1: メモリ管理の歴史を体験
```bash
# C言語の古典的アプローチ
cd code-examples/chapter-01/hands-on-01/c
gcc -o classic_memory classic_memory.c
./classic_memory

# Java GCベンチマーク
cd ../java
javac GcBenchmark.java
java -XX:+PrintGC GcBenchmark

# Python参照カウント
cd ../python
python3 python_memory.py

# Rust所有権システム
cd ../rust
cargo run --release
```

### Step 2: 性能比較ベンチマーク
```bash
cd code-examples/chapter-01/benchmark-comparison
./benchmark_all.sh
```

### Step 3: 言語選択判断ツール
```bash
cd code-examples/chapter-01/decision-framework
cargo run
```

## 3. 執筆作業

### 章の執筆
```bash
# 第1章を編集
vim manuscript/chapters/01-why-rust/chapter-detailed.md

# メタデータ更新
vim manuscript/chapters/01-why-rust/metadata.yaml
```

### 進捗確認
```bash
python3 tools/analytics/progress-tracker.py .
```

### Zenn記事生成
```bash
python3 tools/converters/zenn-converter.py .
ls -la build/output/zenn/
```

## 4. Git管理

```bash
# リポジトリ初期化
git init
git add .
git commit -m "Initial project setup with Chapter 1 implementation"

# GitHubリポジトリ作成後
git remote add origin https://github.com/[username]/rust-professional-book.git
git branch -M main
git push -u origin main
```

## 5. CI/CD確認

GitHub Actionsが自動的に以下を実行：
- Rustコード例のビルド・テスト
- フォーマットチェック
- リンク切れチェック

## 6. 次章の執筆準備

```bash
# 第2章ディレクトリ作成
mkdir -p manuscript/chapters/02-ownership/exercises
mkdir -p code-examples/chapter-02/hands-on-01

# テンプレートからコピー
cp manuscript/chapters/01-why-rust/metadata.yaml manuscript/chapters/02-ownership/
# 編集して第2章用に調整
```

## 執筆のコツ

1. **ハンズオン重視**: 各概念に実行可能なコード例を
2. **定量的比較**: ベンチマークで主張を裏付け
3. **段階的説明**: 他言語経験者の視点から
4. **実践的内容**: 実務で使える知識を

## サポート

問題が発生した場合：
1. `PROJECT_STATUS.md` で現在の状態確認
2. GitHub Issues で課題管理
3. 定期的な進捗確認で執筆ペース維持

頑張って執筆を進めてください！🦀