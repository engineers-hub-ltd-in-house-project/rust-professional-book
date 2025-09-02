// この例は `std::simd` を使用するため、nightly Rust が必要です。
// 実行するには: `rustup override set nightly` の後 `cargo run` を実行してください。
#![feature(portable_simd)]

use std::simd::{f32x4, Simd};
use std::time::Instant;

const SIZE: usize = 1_000_000;

fn main() {
    let a: Vec<f32> = (0..SIZE).map(|i| i as f32).collect();
    let b: Vec<f32> = (0..SIZE).map(|i| (i * 2) as f32).collect();
    let mut c_scalar: Vec<f32> = vec![0.0; SIZE];
    let mut c_simd: Vec<f32> = vec![0.0; SIZE];

    println!("--- スカラー加算 ---");
    let start = Instant::now();
    for i in 0..SIZE {
        c_scalar[i] = a[i] + b[i];
    }
    let duration = start.elapsed();
    println!("時間: {:?}", duration);
    // println!("最初の10個のスカラー結果: {:?}", &c_scalar[0..10]);

    println!("\n--- SIMD加算 (f32x4) ---");
    let start = Instant::now();
    // 4要素ずつ処理 (f32x4 の場合)
    for i in (0..SIZE).step_by(4) {
        // 4つのf32をSIMDベクトルにロード
        let simd_a = f32x4::from_slice(&a[i..i+4]);
        let simd_b = f32x4::from_slice(&b[i..i+4]);

        // 4つの要素に対して同時に加算を実行
        let simd_c = simd_a + simd_b;

        // 結果をスライスにストア
        simd_c.write_to_slice(&mut c_simd[i..i+4]);
    }
    let duration = start.elapsed();
    println!("時間: {:?}", duration);
    // println!("最初の10個のSIMD結果: {:?}", &c_simd[0..10]);

    // 結果を検証 (SIZEが4の倍数の場合のみ)
    if SIZE % 4 == 0 {
        assert_eq!(c_scalar, c_simd);
        println!("\n結果が一致しました！");
    } else {
        println!("\n結果は完全に検証されていません (SIZEが4の倍数ではありません)。");
    }
}
