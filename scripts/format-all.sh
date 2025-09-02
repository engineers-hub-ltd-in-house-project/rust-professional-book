#!/bin/bash
# 全ファイルを整形する統合スクリプト

set -e

echo "🔧 Starting comprehensive formatting..."

# 1. Markdown/JSON/TOML/YAML (dprint - 超高速)
echo "📝 Formatting Markdown/JSON/TOML/YAML with dprint..."
if command -v dprint &> /dev/null; then
    dprint fmt
else
    echo "⚠️  dprint not found. Install: cargo install dprint"
fi

# 2. Rust (rustfmt)
echo "🦀 Formatting Rust code..."
if command -v rustfmt &> /dev/null; then
    find code-examples -name "*.rs" -exec rustfmt --edition 2021 {} \;
else
    echo "⚠️  rustfmt not found. Install: rustup component add rustfmt"
fi

# 3. Python (black)
echo "🐍 Formatting Python code..."
if command -v black &> /dev/null; then
    find code-examples -name "*.py" -exec black -q {} \;
else
    echo "⚠️  black not found. Install: pip install black"
fi

# 4. Java (google-java-format)
echo "☕ Formatting Java code..."
if command -v google-java-format &> /dev/null; then
    find code-examples -name "*.java" -exec google-java-format -i {} \;
else
    echo "⚠️  google-java-format not found"
fi

# 5. C/C++ (clang-format)
echo "🔧 Formatting C/C++ code..."
if command -v clang-format &> /dev/null; then
    find code-examples \( -name "*.c" -o -name "*.cpp" -o -name "*.h" -o -name "*.hpp" \) -exec clang-format -i {} \;
else
    echo "⚠️  clang-format not found"
fi

# 6. 日本語文章チェック (textlint)
echo "🇯🇵 Checking Japanese text with textlint..."
if [ -f "package.json" ] && command -v npm &> /dev/null; then
    npm run textlint:fix
else
    echo "⚠️  textlint not configured. Run: npm install"
fi

echo "✅ Formatting complete!"
echo ""
echo "📊 Summary:"
echo "  - Markdown/Config files: dprint"
echo "  - Rust: rustfmt"
echo "  - Python: black"
echo "  - Java: google-java-format"
echo "  - C/C++: clang-format"
echo "  - Japanese text: textlint"