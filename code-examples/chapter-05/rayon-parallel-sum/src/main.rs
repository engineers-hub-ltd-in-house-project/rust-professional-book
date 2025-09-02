use rayon::prelude::*;
use std::time::Instant;

fn main() {
    let size = 10_000_000;
    let numbers: Vec<u64> = (0..size).collect();

    println!("--- 逐次処理による二乗和 ---");
    let start = Instant::now();
    let sequential_sum: u64 = numbers.iter().map(|&i| i * i).sum();
    let duration = start.elapsed();
    println!("結果: {}", sequential_sum);
    println!("所要時間: {:?}", duration);

    println!("\n--- 並列処理による二乗和 (Rayon使用) ---");
    let start = Instant::now();
    // .iter() を .par_iter() に変えるだけ！
    let parallel_sum: u64 = numbers.par_iter().map(|&i| i * i).sum();
    let duration = start.elapsed();
    println!("結果: {}", parallel_sum);
    println!("所要時間: {:?}", duration);

    assert_eq!(sequential_sum, parallel_sum);
}
