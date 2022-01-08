use criterion::{criterion_group, criterion_main, Criterion};
use matcher::matcher::Matcher;

pub fn criterion_benchmark(c: &mut Criterion) {
    let _m = Matcher{};
    let i = 0;
    c.bench_function("Matcher", |b| b.iter(|| _m.proceed_record()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
