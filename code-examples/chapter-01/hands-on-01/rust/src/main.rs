use std::time::Instant;

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn fibonacci_iterative(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    
    let mut prev = 0;
    let mut curr = 1;
    
    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    
    curr
}

fn main() {
    println!("Rust Performance Benchmark - Chapter 01");
    println!("=========================================");
    
    let test_values = vec![10, 20, 30, 35, 40];
    
    println!("\n再帰的フィボナッチ:");
    for n in &test_values {
        let start = Instant::now();
        let result = fibonacci(*n);
        let duration = start.elapsed();
        println!("  fib({}) = {} | 時間: {:?}", n, result, duration);
    }
    
    println!("\n反復的フィボナッチ:");
    for n in &test_values {
        let start = Instant::now();
        let result = fibonacci_iterative(*n);
        let duration = start.elapsed();
        println!("  fib({}) = {} | 時間: {:?}", n, result, duration);
    }
}