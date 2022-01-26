use criterion::{black_box, criterion_group, criterion_main, Criterion};
use benchmark::fib;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fib::fibonacci(black_box(20))));
}

pub fn criterion_benchmark30(c: &mut Criterion) {
    c.bench_function("fib 30", |b| b.iter(|| fib::fibonacci(black_box(30))));
}

criterion_group!(benches,
                 criterion_benchmark,
                 criterion_benchmark30);
criterion_main!(benches);
