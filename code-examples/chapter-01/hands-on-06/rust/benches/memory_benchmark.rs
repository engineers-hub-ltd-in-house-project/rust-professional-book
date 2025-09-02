use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use performance_comparison::*;

fn memory_operations_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_operations");
    
    // 様々なサイズでベンチマーク
    for size in [1_000, 10_000, 100_000].iter() {
        group.bench_with_input(
            BenchmarkId::new("rust_vec", size), 
            size, 
            |b, &size| {
                b.iter(|| {
                    let mut vec = Vec::with_capacity(size);
                    for i in 0..size {
                        vec.push(black_box(i));
                    }
                    vec
                });
            }
        );
        
        group.bench_with_input(
            BenchmarkId::new("rust_processing", size),
            size,
            |b, &size| {
                // 事前準備
                let data: Vec<i32> = (0..size).collect();
                
                b.iter(|| {
                    let result: i32 = data.iter()
                        .filter(|&&x| x % 2 == 0)
                        .map(|&x| x * 2)
                        .sum();
                    black_box(result)
                });
            }
        );
    }
    
    group.finish();
}

criterion_group!(benches, memory_operations_benchmark);
criterion_main!(benches);