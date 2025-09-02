use benchmark_driven_example::{
    format_user_list_idiomatic, format_user_list_naive, format_user_list_single_string,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn formatting_benchmark(c: &mut Criterion) {
    // ベンチマーク用のデータを作成
    let users: Vec<(u32, String)> = (0..1000).map(|i| (i, format!("User{}", i))).collect();
    // ベンチマーク関数は`&str`を期待するため、変換する
    let users_str: Vec<(u32, &str)> = users.iter().map(|(id, name)| (*id, name.as_str())).collect();

    let mut group = c.benchmark_group("Formatting User Lists");

    // ベースラインとなる素朴な実装を計測
    group.bench_function("naive_loop", |b| {
        b.iter(|| format_user_list_naive(black_box(&users_str)))
    });

    // イテレータを使った慣用的な実装を計測
    group.bench_function("idiomatic_map", |b| {
        b.iter(|| format_user_list_idiomatic(black_box(&users_str)))
    });

    // アロケーションを削減した最適化版を計測
    group.bench_function("single_string_allocation", |b| {
        b.iter(|| format_user_list_single_string(black_box(&users_str)))
    });

    group.finish();
}

criterion_group!(benches, formatting_benchmark);
criterion_main!(benches);
