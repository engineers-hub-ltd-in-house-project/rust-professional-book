#!/bin/bash
# 自動整形ツールのセットアップスクリプト

set -e

echo "🚀 Setting up formatting tools for rust-professional-book"
echo ""

# 1. Lefthook (Git hooks manager)
echo "📎 Installing lefthook..."
if ! command -v lefthook &> /dev/null; then
    if [[ "$OSTYPE" == "darwin"* ]]; then
        brew install lefthook
    else
        curl -sSf https://raw.githubusercontent.com/evilmartians/lefthook/master/install.sh | sh
    fi
else
    echo "✓ lefthook already installed"
fi

# 2. dprint (超高速フォーマッター)
echo "⚡ Installing dprint..."
if ! command -v dprint &> /dev/null; then
    cargo install dprint
else
    echo "✓ dprint already installed"
fi

# 3. Node.js dependencies (textlint)
echo "📦 Installing Node.js dependencies..."
if [ -f "package.json" ]; then
    npm install
else
    echo "⚠️  package.json not found"
fi

# 4. Rust formatting
echo "🦀 Setting up Rust formatter..."
rustup component add rustfmt

# 5. Python formatter
echo "🐍 Installing Python formatter..."
if ! command -v black &> /dev/null; then
    pip3 install black
else
    echo "✓ black already installed"
fi

# 6. Initialize lefthook
echo "🔗 Initializing Git hooks..."
lefthook install

echo ""
echo "✅ Setup complete!"
echo ""
echo "📋 Available commands:"
echo "  npm run format:all    - Format all files"
echo "  npm run textlint      - Check Japanese text"
echo "  npm run textlint:fix  - Fix Japanese text issues"
echo "  dprint fmt            - Format Markdown/JSON/TOML"
echo "  lefthook run pre-commit - Run pre-commit checks"
echo ""
echo "🎉 Ready to start writing with automatic formatting!"