use std::time::Instant;

fn benchmark_rust() {
    let sizes = vec![1_000, 10_000, 100_000];
    
    for size in sizes {
        let start = Instant::now();
        
        // データ生成
        let data: Vec<i32> = (0..size).collect();
        
        // 処理: 偶数のみフィルタ → 2倍 → 合計
        let result: i32 = data.iter()
            .filter(|&&x| x % 2 == 0)
            .map(|&x| x * 2)
            .sum();
        
        let elapsed = start.elapsed();
        let elapsed_ms = elapsed.as_secs_f64() * 1000.0;
        
        println!("Size: {}, Result: {}, Time: {:.2} ms", 
            size, result, elapsed_ms);
    }
}

fn main() {
    println!("=== Rust Benchmark ===");
    benchmark_rust();
}