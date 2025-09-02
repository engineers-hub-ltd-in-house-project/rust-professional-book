// この関数の返り値の型は借用された値を含んでいますが、
// どの値から借用されたものかを示すライフタイムがありません。
// 
// このコードは、意図通りコンパイルに失敗します。

fn get_dangling_reference() -> &i32 {
    let local_var = 42;
    &local_var // 現在の関数が所有するデータへの参照を返そうとしている
} // ここで `local_var` はスコープを抜け、そのメモリは解放される

fn main() {
    let reference_to_nothing = get_dangling_reference();

    // もしこのコードがコンパイルを通過してしまえば、`reference_to_nothing`は
    // C言語の例と同じく、ダングリングリファレンスになってしまう。
    // しかし、借用検査器がそれを防いでくれます！
    //
    // 試してみると、以下のようなコンパイルエラーが表示されます：
    // error[E0106]: missing lifetime specifier
    //  --> src/main.rs:5:33
    //   |
    // 5 | fn get_dangling_reference() -> &i32 {
    //   |                                 ^ expected named lifetime parameter
    //   |
    //   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
    //   = help: consider using the `'static` lifetime
}
