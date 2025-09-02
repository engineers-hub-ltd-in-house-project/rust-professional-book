fn main() {
    // `Vec`はヒープにデータを確保する
    let data = vec![1, 2, 3];

    // `data`の所有権を`data2`にムーブする
    let data2 = data;

    // この時点で、`data`変数はもはや無効。
    // `data2`が唯一の所有者となる。
    // これにより、同じデータを指す有効な変数が複数存在することがなくなり、
    // 誤って二度解放する可能性が原理的に排除される。

    // 以下の行のコメントを外すと、コンパイルエラーになる。
    // `data`は所有権を失っているため、もはや使えない。
    //
    // error[E0382]: borrow of moved value: `data`
    //   --> src/main.rs:17:22
    //    |
    // 5  |     let data = vec![1, 2, 3];
    //    |         ---- move occurs because `data` has type `Vec<i32>`, which does not implement the `Copy` trait
    // ...
    // 7  |     let data2 = data;
    //    |                 ---- value moved here
    // ...
    // 17 |     println!("{:?}", data);
    //    |                      ^^^^ value borrowed here after move
    
    // println!("{:?}", data);

    // `data2`がスコープを抜けるときに、`data2`が所有するベクタのメモリが
    // 一度だけ、自動的に解放される。
    println!("data2 is the owner. It will be dropped automatically at the end of the scope.");
}
