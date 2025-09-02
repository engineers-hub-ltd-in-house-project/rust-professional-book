use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dispatch_benchmark::{process_dynamic, process_static, ComponentA, ComponentB, Renderer};

fn dispatch_benchmark(c: &mut Criterion) {
    let comp_a = ComponentA { data: 10 };
    let comp_b = ComponentB { data: 20 };

    let mut group = c.benchmark_group("Static vs Dynamic Dispatch");

    // --- 静的ディスパッチのベンチマーク ---
    group.bench_function("Static Dispatch (ComponentA)", |b| {
        b.iter(|| process_static(black_box(&comp_a)))
    });
    group.bench_function("Static Dispatch (ComponentB)", |b| {
        b.iter(|| process_static(black_box(&comp_b)))
    });

    // --- 動的ディスパッチのベンチマーク ---
    // `as &dyn Renderer` でトレイトオブジェクトに変換する
    group.bench_function("Dynamic Dispatch (ComponentA)", |b| {
        b.iter(|| process_dynamic(black_box(&comp_a as &dyn Renderer)))
    });
    group.bench_function("Dynamic Dispatch (ComponentB)", |b| {
        b.iter(|| process_dynamic(black_box(&comp_b as &dyn Renderer)))
    });

    group.finish();
}

criterion_group!(benches, dispatch_benchmark);
criterion_main!(benches);
