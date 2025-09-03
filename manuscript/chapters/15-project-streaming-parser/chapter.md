---
part: "第V部: 実践プロジェクト編"
page_count: 15
title: "実践プロジェクト: Zero-copyストリーミングパーサー"
---

# 第15章: プロジェクト3 - Zero-Copyストリーミングパーサー

## 学習目標
本プロジェクトを完了すると、以下が可能になります：
- [ ] ゼロコピー・パースの重要性と、それがパフォーマンスに与える影響を説明できる。
- [ ] パーサーコンビネータの基本的な考え方を理解できる。
- [ ] `nom`クレートを使い、宣言的なスタイルで堅牢かつ高性能なパーサーを構築できる。
- [ ] Rustのライフタイムとスライスが、いかにしてゼロコピー・パースを自然かつ安全に実現するかを理解できる。

---

## 15.1 プロジェクト概要：究極の効率を求めて

ネットワークプロトコルの解析、設定ファイルの読み込み、シリアライズ形式のデコードなど、プログラミングにおいて「ある形式のバイト列や文字列を、意味のあるデータ構造に変換する」という**パース（構文解析）**処理は至る所に存在します。

素朴なパーサー実装、例えば正規表現や文字列の`split`を多用するコードは、処理の過程で多数の小さな`String`オブジェクトを生成しがちです。入力データが大きくなると、この無数のメモリアロケーションとコピーが、深刻なパフォーマンスのボトルネックとなります。

このプロジェクトでは、この問題を解決するため、**ゼロコピー（Zero-Copy）**の原則に基づいた高性能なパーサーを構築します。ゼロコピーとは、パース処理中に不要なメモリアロケーションやコピーを一切行わず、パース結果はすべて元の入力バッファへの**スライス (`&str`)** として返す、という考え方です。Rustの所有権とライフタイムシステムは、このアプローチを驚くほど安全かつ自然に実現します。

この目的を達成するため、我々は**パーサーコンビネータ**という強力なパラダイムと、そのRustにおけるデファクトスタンダードな実装である**`nom`**クレートを利用します。

### 15.1.1 パーサーコンビネータとは？

パーサーコンビネータは、複雑なパーサーを構築するための関数型プログラミングのテクニックです。その思想は非常にシンプルです。

1.  **小さなパーサーを作る:** 「一つの英数字の連続（識別子）をパースする」「`=`という文字をパースする」「改行文字をパースする」といった、ごく基本的な要素をパースする小さな関数を多数用意する。
2.  **パーサーを組み合わせる:** それらの小さなパーサー関数を、まるでレゴブロックのように**組み合わせ（コンバイン）**て、より複雑な構造（「識別子`=`識別子`\n`」という一行）をパースする、新しい大きなパーサーを構築する。

このアプローチは、巨大で複雑な状態を持つ単一のパーサー関数を手で書くのに比べて、宣言的で、再利用性が高く、テストも容易であるという利点があります。

### 15.1.2 `nom`の基本

`nom`のパーサーは、基本的に`Fn(Input) -> IResult<Input, Output>`という形の関数です。`Input`はパース対象の入力（通常は`&str`や`&[u8]`）、`IResult`は`Result< (Input, Output), Err<Input> >`のエイリアスです。

-   **成功した場合:** `Ok((remaining_input, parsed_output))`を返す。つまり、パース後に残った入力と、パース結果のペア。
-   **失敗した場合:** `Err(...)`を返す。

この統一されたインターフェースにより、様々なパーサーを連鎖させたり、組み合わせたりすることが可能になります。

---

## 15.2 ハンズオン：`.env`形式のゼロコピー・パーサー

題材として、`KEY=VALUE`形式のシンプルな設定ファイル（`.env`ファイルのような形式）をパースするライブラリを構築します。完成版のコードは`code-examples/chapter-15/zero-copy-parser/`にあります。

### 15.2.1 プロジェクトのセットアップ

`nom`を依存関係に追加した、新しいライブラリプロジェクトを作成します。

```toml
[package]
name = "zero-copy-parser"
version = "0.1.0"
edition = "2021"

[dependencies]
nom = "7"
```

### 15.2.2 パーサーの実装 (`src/lib.rs`)

まず、`nom`の主要なコンポーネントをインポートします。

```rust
use nom::{
    IResult,
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, alphanumeric1, multispace0},
    combinator::recognize,
    sequence::{separated_pair, terminated},
    multi::many0,
};
use std::collections::HashMap;
```

**1. 基本的なパーサーの構築**

まず、キー（`KEY`）をパースするための小さなパーサーを定義します。
キーは英字で始まり、英数字が続くとします。

```rust
// キーとして有効な文字列（例: "API_KEY", "PORT"）を認識するパーサー
fn parse_key(input: &str) -> IResult<&str, &str> {
    recognize(
        nom::sequence::pair(
            alpha1,
            many0(alphanumeric1)
        )
    )(input)
}
```
`recognize`は、内側のパーサー（ここでは`alpha1`と`many0(alphanumeric1)`のペア）がマッチした入力部分全体を、一つのスライスとして返す便利なコンビネータです。

**2. パーサーの組み合わせ**

次に、`KEY=VALUE`という一行をパースするために、小さなパーサーを組み合わせていきます。

```rust
// `=` の前後にある空白を無視しつつ、`=`自体をパースする
fn parse_equals(input: &str) -> IResult<&str, &str> {
    let (input, _) = multispace0(input)?;
    let (input, eq) = tag("=")(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, eq))
}

// 行末までの全てを値として取得するパーサー
fn parse_value(input: &str) -> IResult<&str, &str> {
    take_while1(|c| c != '\n')(input)
}

// `KEY=VALUE` のペアをパースする
fn parse_pair(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(
        parse_key,      // 左側
        parse_equals,   // 区切り文字
        parse_value     // 右側
    )(input)
}
```
`separated_pair`は、`nom`の強力なコンビネータの一つで、3つのパーサーを順番に適用し、左右のパーサーの結果をタプルとして返します。

**3. 全体をパースするトップレベルパーサー**

最後に、複数行の`KEY=VALUE`ペアをパースし、`HashMap`に格納するトップレベルの関数を定義します。

```rust
// 複数のキーバリューペアをパースし、HashMapに集める
pub fn parse_env_file(input: &str) -> IResult<&str, HashMap<&str, &str>> {
    let (input, pairs) = many0(
        // 各行をパースし、改行で終了する
        terminated(parse_pair, multispace0)
    )(input)?;

    let map = pairs.into_iter().collect();
    Ok((input, map))
}
```
`many0`は、内側のパーサー（`terminated(parse_pair, multispace0)`）が0回以上連続で成功する限り適用し、その結果を`Vec`として返します。

### 15.2.3 テストと実行

このパーサーが正しく動作することをテストで確認します。

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "
PORT=8080
DATABASE_URL=postgres://...\n
# A comment\nAPI_KEY=secret
";
        let (_, map) = parse_env_file(input).unwrap();

        assert_eq!(map.get("PORT"), Some(&"8080"));
        assert_eq!(map.get("DATABASE_URL"), Some(&"postgres://..."));
        assert_eq!(map.get("API_KEY"), Some(&"secret"));
        assert_eq!(map.len(), 3);
    }
}
```
注目すべきは、`HashMap<&str, &str>`のキーもバリューも`&str`、つまり元の`input`文字列へのスライスである点です。パース処理中に`String`の確保は一切行われていません。これこそがゼロコピー・パースです。

---

## 15.3 まとめ

この最後のプロジェクトでは、Rustのパフォーマンスを極限まで引き出すための強力なパラダイムを学びました。

-   **パーサーコンビネータ (`nom`)** は、宣言的で再利用性の高い方法で、複雑なパーサーを安全に構築する手段を提供します。
-   **ゼロコピー**は、不要なメモリアロケーションとコピーを排除することで、データ集約的なタスクのパフォーマンスを劇的に改善するテクニックです。
-   **Rustの所有権とスライス**は、ゼロコピー・パースを言語レベルで自然かつ安全にサポートする、理想的な基盤です。

これにて、本書の主要な学習パートは全て終了です。第I部から第V部まで、理論的基礎から始まり、Rustの核心的な機能を学び、そして3つの実践的なプロジェクトを経験しました。あなたはもはや、単にRustの構文を知っているだけでなく、「なぜRustがそのように設計されているのか」を深く理解し、様々な課題に対して、安全で、並行で、高性能なソリューションを自信を持って構築できる、真のプロフェッショナルとなったはずです。