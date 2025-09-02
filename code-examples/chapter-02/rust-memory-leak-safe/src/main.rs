// この関数は、RAII (Resource Acquisition Is Initialization) パターンにより、
// どのようにリターンしてもメモリリークを防ぐ。
fn process_data(error_condition: bool) {
    // `Vec`が作られるとき、メモリが確保される (Resource Acquisition Is Initialization)
    let buffer = vec![0u8; 1024];
    println!("Buffer allocated. (Capacity: {})", buffer.capacity());

    // ... bufferを使った何らかの処理 ...
    println!("Processing data...");

    if error_condition {
        println!("An error occurred! Returning early.");
        // エラーが発生しても、この関数から抜ける際に`buffer`がスコープを抜ける。
        // `buffer`の`Drop`トレイトの実装が自動的に呼ばれ、メモリが解放される。
        // そのため、メモリリークは発生しない。
        return;
    }

    println!("Processing successful.");
    // 正常終了時も同様に、関数の終端で`buffer`がスコープを抜け、
    // `drop`が呼ばれてメモリが解放される。
}

fn main() {
    println!("--- Running normal case ---");
    process_data(false);

    println!("\n--- Running error case ---");
    process_data(true);

    println!("\nProgram finished. No memory leaks in either case.");
}
