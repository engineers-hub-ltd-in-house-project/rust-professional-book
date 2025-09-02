use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aos_soa_benchmark::{PointAoS, PointsSoA, sum_x_aos, sum_x_soa};

const DATA_SIZE: usize = 1_000_000;

fn data_layout_benchmark(c: &mut Criterion) {
    // AoSデータを準備
    let aos_data: Vec<PointAoS> = (0..DATA_SIZE)
        .map(|i| PointAoS {
            x: i as f64,
            y: (i * 2) as f64,
            z: (i * 3) as f64,
        })
        .collect();

    // SoAデータを準備
    let mut soa_data = PointsSoA::new(DATA_SIZE);
    for i in 0..DATA_SIZE {
        soa_data.x[i] = i as f64;
        soa_data.y[i] = (i * 2) as f64;
        soa_data.z[i] = (i * 3) as f64;
    }

    let mut group = c.benchmark_group("データレイアウト (AoS vs SoA)");

    group.bench_function("sum_x_aos", |b| {
        b.iter(|| sum_x_aos(black_box(&aos_data)))
    });

    group.bench_function("sum_x_soa", |b| {
        b.iter(|| sum_x_soa(black_box(&soa_data)))
    });

    group.finish();
}

criterion_group!(benches, data_layout_benchmark);
criterion_main!(benches);
