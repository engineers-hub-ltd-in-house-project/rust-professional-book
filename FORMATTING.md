# 自動整形ツール設定ガイド

## 概要

本プロジェクトでは **lefthook + dprint + 専門ツール** の組み合わせで高速かつ高品質な自動整形を実現します。

### ツール構成

| ツール | 役割 | 対象 | 特徴 |
|--------|------|------|------|
| **lefthook** | Git hooks 管理 | 全体 | husky より高速 |
| **dprint** | 汎用フォーマッター | Markdown/JSON/TOML/YAML | Rust 製で超高速 |
| **textlint** | 日本語校正 | 原稿 | 技術文書向け設定 |
| **rustfmt** | Rust フォーマッター | *.rs | 公式ツール |
| **black** | Python フォーマッター | *.py | 設定不要 |
| **clang-format** | C/C++ フォーマッター | *.c/*.cpp | 標準的 |

## セットアップ

### 自動セットアップ（推奨）
```bash
./scripts/setup-tools.sh
```

### 手動セットアップ
```bash
# 1. lefthook インストール
curl -sSf https://raw.githubusercontent.com/evilmartians/lefthook/master/install.sh | sh

# 2. dprint インストール
cargo install dprint

# 3. Node.js 依存関係
npm install

# 4. Rust フォーマッター
rustup component add rustfmt

# 5. Python フォーマッター
pip3 install black

# 6. Git hooks 有効化
lefthook install
```

## 使用方法

### 全ファイル整形
```bash
npm run format:all
# または
./scripts/format-all.sh
```

### 個別整形
```bash
# Markdown/設定ファイル（高速）
dprint fmt

# 日本語文章チェック
npm run textlint
npm run textlint:fix  # 自動修正

# Rust コード
cargo fmt --all

# 手動でプレコミットチェック
lefthook run pre-commit
```

## Git フック

### pre-commit（コミット時）
- **並列実行**で高速処理
- dprint で Markdown/JSON/TOML を整形
- rustfmt で Rust コードを整形
- textlint で日本語文章をチェック

### pre-push（プッシュ時）
- Rust コードのビルドチェック
- Markdown のリンク切れチェック

### commit-msg（コミットメッセージ）
- Conventional Commits 形式を強制
- 例: `feat(chapter-01): Add ownership examples`

## 設定ファイル

| ファイル | 説明 |
|----------|------|
| `lefthook.yml` | Git hooks 設定 |
| `dprint.json` | dprint 設定（高速整形） |
| `.textlintrc.json` | 日本語校正ルール |
| `prh.yml` | 用語統一ルール |

## トラブルシューティング

### lefthook が動かない
```bash
# 再インストール
lefthook uninstall
lefthook install
```

### dprint が見つからない
```bash
cargo install dprint
# PATH に追加
export PATH="$HOME/.cargo/bin:$PATH"
```

### textlint エラー
```bash
# 依存関係再インストール
rm -rf node_modules package-lock.json
npm install
```

## パフォーマンス比較

| ツール | 1000 ファイル処理時間 |
|--------|----------------------|
| prettier | ~10秒 |
| **dprint** | **~0.1秒** |

## 推奨ワークフロー

1. **執筆**: 気にせず書く
2. **保存時**: エディタの自動整形（VSCode/Neovim）
3. **コミット時**: lefthook が自動整形
4. **プッシュ時**: ビルド・リンクチェック

これにより執筆に集中でき、品質は自動的に保たれます。