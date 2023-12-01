use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_1::{part1, part2};

fn bench_p1(c: &mut Criterion) {
    c.bench_function(
        format!("{} - part 1", env!("CARGO_PKG_NAME")).as_str(),
        |b| {
            b.iter(|| part1(black_box(include_str!("../input.txt"))));
        },
    );
}

fn bench_p2(c: &mut Criterion) {
    c.bench_function(
        format!("{} - part 2", env!("CARGO_PKG_NAME")).as_str(),
        |b| {
            b.iter(|| part2(black_box(include_str!("../input.txt"))));
        },
    );
}

criterion_group!(benches, bench_p1, bench_p2);
criterion_main!(benches);
