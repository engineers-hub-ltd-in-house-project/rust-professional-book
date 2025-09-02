# プロジェクトステータス
# Project Status

本ドキュメントは、「プロフェッショナルのためのRust実践理論」プロジェクトの現在の進捗状況をまとめたものです。
This document summarizes the current progress of "The Rust Book for Professionals" project.

## 実装完了項目 (2025-09-02)
## Completed Items (2025-09-02)

### 書籍構成
### Book Structure
- 詳細な全体構成（18章構成）を追加
- 全18章の詳細原稿とハンズオン課題を配置
- 学習目標と対象読者の明確化

### コード例・ハンズオン
### Code Examples & Hands-on Exercises
- 全18章のコード例が実装済み
- Code examples for all 18 chapters have been implemented.

### ツール・スクリプト
### Tools & Scripts
- Zenn記事変換ツール
- 進捗追跡ツール
- GitHub Actions CI/CD設定

---

## 実行方法
## Execution Methods

### ベンチマーク実行
### Benchmark Execution
```bash
cd code-examples/chapter-01/benchmark-comparison
./benchmark_all.sh
```

### 判断フレームワーク実行
### Decision Framework Execution
```bash
cd code-examples/chapter-01/decision-framework
cargo run
```

### 進捗確認
### Progress Tracking
```bash
python3 tools/analytics/progress-tracker.py .
```



---

## ディレクトリ構造
## Directory Structure

本プロジェクトの主要なディレクトリとその役割は以下の通りです。
The main directories of this project and their roles are as follows:

*   `.github/`: GitHub ActionsのワークフローやIssueテンプレートなど、GitHub関連の設定を格納します。
    *   `ISSUE_TEMPLATE/`: GitHubのIssueテンプレート。
    *   `workflows/`: GitHub Actionsのワークフロー定義（例: `build.yml`）。
*   `assets/`: 書籍で使用される画像、フォントなどのアセットを格納します。
    *   `fonts/`: カスタムフォント。
    *   `images/`: 書籍で使用される画像（`covers/`, `diagrams/`, `screenshots/`に分類）。
    *   `templates/`: ドキュメント生成などに使用されるテンプレート。
*   `build/`: 書籍のビルドプロセスに関連する設定ファイルやスクリプトを格納します。
    *   `config/`: ビルド設定ファイル。
    *   `formats/`: 各出力形式（例: `oreilly/`, `qiita/`, `techbookfest/`, `web/`, `zenn/`）ごとの設定やテンプレート。
    *   `scripts/`: ビルド自動化スクリプト。
*   `code-examples/`: 書籍で参照される全てのコード例とハンズオン課題を章ごとに整理して格納します。
    *   `chapter-XX/`: 各章に対応するサブディレクトリ（例: `hands-on-01/`, `benchmark-comparison/`, `decision-framework/`）。
    *   `projects/`: 後半の章で言及される、より大規模な統合プロジェクト。
    *   `tests/`: コード例自体のテスト。
*   `docs/`: 書籍の内容とは別に、プロジェクト自体のドキュメントを格納します。
    *   `guidelines/`: プロジェクトのガイドライン（`CONTRIBUTING.md`を含む）。
    *   `project-proposal/`: 書籍の提案書関連ドキュメント（例: `oreilly-proposal.md`）。
    *   `strategy/`: プロジェクト戦略ドキュメント。
*   `manuscript/`: 書籍の主要な原稿コンテンツを格納します。
    *   `appendix/`: 付録。
    *   `bibliography/`: 参考文献。
    *   `chapters/`: 各章のファイル（例: `01-why-rust/`）。各章ディレクトリには、原稿ファイルとメタデータが含まれます。
    *   `outline/`: 書籍の構成案ドキュメント（例: `book-outline.md`）。
*   `marketing/`: 書籍のマーケティングおよびプロモーション関連資料を格納します。
    *   `blog-posts/`: ブログ記事の下書きやコンテンツ。
    *   `presentations/`: プレゼンテーション資料。
    *   `press-kit/`: プレス関連アセット。
    *   `social-media/`: ソーシャルメディア用コンテンツ。
*   `scripts/`: プロジェクト全体の汎用ユーティリティスクリプト（例: `format-all.sh`, `setup-tools.sh`）。
*   `tools/`: 書籍作成や分析を支援するために開発されたカスタムツールを格納します。
    *   `analytics/`: 進捗追跡やデータ分析ツール（例: `progress-tracker.py`）。
    *   `converters/`: コンテンツ形式変換ツール（例: `zenn-converter.py`）。
    *   `deployment/`: デプロイ関連ツール。
    *   `validators/`: コンテンツ検証ツール。

**トップレベルファイル:**

*   `.gitignore`: Gitが無視するファイルを指定。
*   `.textlintrc.json`: テキストリンターtextlintの設定。
*   `dprint.json`: コードフォーマッターdprintの設定。
*   `FORMATTING.md`: フォーマット規約を詳述したドキュメント。
*   `GETTING_STARTED.md`: プロジェクトの開始ガイド。
*   `lefthook.yml`: Gitフックマネージャーlefthookの設定。
*   `LICENSE`: プロジェクトのライセンス。
*   `package.json`: Node.jsプロジェクト設定（フロントエンドツールやビルドスクリプト用）。
*   `prh.yml`: テキスト置換ツールprhの設定。
*   `PROJECT_STATUS.md`: プロジェクトの現在のステータスを詳述したドキュメント。
*   `README.md`: プロジェクトのメインREADME。

この構造は、技術書の執筆と出版のための明確に組織化されたプロジェクトを示しており、原稿コンテンツ、コード例、ビルドプロセス、ドキュメント、マーケティングなど、各関心事が明確に分離されています。