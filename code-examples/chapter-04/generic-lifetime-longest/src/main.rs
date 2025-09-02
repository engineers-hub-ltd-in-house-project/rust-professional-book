// ここでのライフタイム注釈 `<'a>` は、コンパイラが返される参照が
// `x`と`y`のどちらになるか判断できないために必要となる。
// 両方に同じライフタイム`'a`を与えることで、我々はコンパイラに
// 「返される参照は、二つの入力参照のうち、短い方のライフタイムと同じ期間だけ有効である」
// と教えている。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz"; // string2は文字列リテラルで、'staticライフタイムを持つ

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);

    // このブロックは、ライフタイムがどのように有効性を強制するかを示す
    let string3 = String::from("long string is long");
    let result2;
    {
        let string4 = String::from("xyz");
        // `result2`は、string3とstring4のうち短い方のライフタイムを持つ。
        // この場合、`string4`のライフタイムとなる。
        result2 = longest(string3.as_str(), string4.as_str());
        println!("内側のスコープでは、longestは '{}'", result2);
    } // ここで`string4`がdropされる。`result2`のライフタイムもここで終わる。

    // 以下の行のコメントを外すと、`result2`のライフタイム（`string4`に紐づく）が
    // 終了しているため、コンパイルエラーになる。
    // error[E0597]: `string4` does not live long enough
    // println!("外側のスコープでは、result2はもはや有効ではない: {}", result2);
}
