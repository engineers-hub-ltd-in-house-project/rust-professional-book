fn main() {
    println!("--- 所有権のデモ: ムーブ ---");
    let s1 = String::from("hello");
    // s1の所有権はs2にムーブする。s1は以後無効になる。
    let s2 = s1;

    println!("s2の値: {}", s2);

    // 以下の行のコメントを外すと、s1の値がムーブされているため、コンパイルエラーになる。
    // 試してみましょう！
    // println!("s1はもうアクセスできない: {}", s1);

    println!("\n--- 所有権のデモ: クローン ---");
    let s3 = String::from("world");
    // ディープコピーを作るには、明示的にclone()を呼ぶ。
    let s4 = s3.clone();

    println!("s3 = {}, s4 = {}", s3, s4); // s3とs4の両方が有効。

    println!("\n--- 所有権のデモ: 関数 ---");
    let s5 = String::from("takes ownership");
    // s5は関数にムーブされ、ここでは無効になる。
    takes_ownership(s5);
    // println!("{}", s5); // コンパイルエラー！

    let x = 5;
    // i32はCopyトレイトを持つため、xはムーブされずに関数にコピーされる。
    makes_copy(x);
    println!("xはまだ有効: {}", x); // xはまだ有効。
}

fn takes_ownership(some_string: String) {
    println!("takes_ownershipの中: {}", some_string);
} // ここで`some_string`がスコープを抜け、`drop`が呼ばれる。

fn makes_copy(some_integer: i32) {
    println!("makes_copyの中: {}", some_integer);
} // ここで`some_integer`がスコープを抜ける。特別なことは何も起きない。
