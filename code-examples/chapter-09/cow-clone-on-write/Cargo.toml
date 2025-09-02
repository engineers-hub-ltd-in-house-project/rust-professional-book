use std::borrow::Cow;

// この関数は文字列を処理する。
// 絵文字が含まれていれば、それを削除して所有されたStringを返す。
// そうでなければ、元の文字列の借用されたスライスを返す。
fn remove_emojis<'a>(text: &'a str) -> Cow<'a, str> {
    if text.contains('😀') || text.contains('😂') {
        println!("  (絵文字を検出しました。クローンして変更します)");
        // 変更が必要な場合、データをクローンしてOwnedなCowを返す。
        Cow::Owned(text.replace('😀', "").replace('😂', ""))
    } else {
        println!("  (絵文字なし。借用されたものを返します)");
        // 変更が不要な場合、BorrowedなCowを返す（アロケーションなし）。
        Cow::Borrowed(text)
    }
}

fn main() {
    let s1 = "Hello 😀 World!";
    let s2 = "Just a plain string.";
    let s3 = "Another 😂 emoji here.";

    println!("'{}' を処理中", s1);
    let processed1 = remove_emojis(s1);
    println!("結果: '{}', 所有されているか: {}
", processed1, processed1.is_owned());

    println!("'{}' を処理中", s2);
    let processed2 = remove_emojis(s2);
    println!("結果: '{}', 所有されているか: {}
", processed2, processed2.is_owned());

    println!("'{}' を処理中", s3);
    let processed3 = remove_emojis(s3);
    println!("結果: '{}', 所有されているか: {}
", processed3, processed3.is_owned());

    // 所有されたCowは変更可能。
    let mut owned_cow: Cow<'static, str> = Cow::Owned(String::from("変更可能な文字列"));
    println!("初期の所有されたCow: {}", owned_cow);
    owned_cow.to_mut().push_str(" が追加されました。");
    println!("変更後の所有されたCow: {}", owned_cow);

    // 借用されたCowを変更しようとすると、自動的に自身をクローンする。
    let mut borrowed_cow: Cow<'static, str> = Cow::Borrowed("変更不可能な文字列");
    println!("初期の借用されたCow: {}", borrowed_cow);
    borrowed_cow.to_mut().push_str(" が追加されました。"); // これがクローンをトリガーする
    println!("変更後の借用されたCow: {}", borrowed_cow);
    println!("変更後に所有されたか？ {}", borrowed_cow.is_owned());
}
