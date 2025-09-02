// 1. トレイトで共通の振る舞いを定義する
pub trait Summary {
    // 著者の名前を返すメソッド
    fn summarize_author(&self) -> String;

    // デフォルト実装を持つメソッド
    fn summarize(&self) -> String {
        format!("(Read more from {})", self.summarize_author())
    }
}

// 2. この振る舞いを持たせたい型を定義する
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// 3. 各々の型にトレイトを実装する
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    // デフォルト実装をオーバーライドすることも可能
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // こちらはデフォルトの`summarize`メソッドをそのまま使う
}

// 4. ジェネリックな関数でトレイトを制約として使う
// この関数は、`Summary`トレイトを実装したあらゆる型を受け取れる
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    println!("--- ジェネリックなnotify関数を呼び出し ---");
    notify(&tweet);
    notify(&article);

    println!("\n--- メソッドを直接呼び出し ---");
    println!("New tweet: {}", tweet.summarize());
    println!("New article: {}", article.summarize());
}
