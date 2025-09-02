// この構造体は参照を保持するため、その定義にライフタイム注釈が必要となる。
// この注釈は、`ImportantExcerpt`のインスタンスが、その`part`フィールドに
// 保持している参照よりも長く生存できないことをコンパイラに伝える。
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // 構造体のメソッド
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    // `first_sentence`への参照を保持する構造体のインスタンスを作成
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    i.announce_and_return_part("I have an important excerpt!");

    // 以下のコードは、Rustが提供する安全性を実証する
    let excerpt_part;
    {
        let short_lived_string = String::from("This is a short-lived string.");
        let excerpt = ImportantExcerpt {
            part: short_lived_string.as_str(),
        };
        excerpt_part = excerpt.part;
    } // ここで`short_lived_string`がdropされる

    // もしここで`excerpt_part`を使おうとすると、コンパイラはエラーを出す。
    // なぜなら、それは既にdropされたデータを参照しているからである。
    // 以下の行のコメントを外すと、コンパイルエラーが発生する。
    // error[E0597]: `short_lived_string` does not live long enough
    // println!("Dangling part: {}", excerpt_part);
}
