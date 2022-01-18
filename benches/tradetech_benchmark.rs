use criterion::{criterion_group, criterion_main, Criterion};
extern crate matcher;

use matcher::order;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut m = matcher::Matcher::new();
    c.bench_function("Matcher", |b| b.iter(|| m.proceed_record(order::Order::new(
        order::OrderType::Lim,
        order::Side::Buy,
        100,
        1,
        1))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
