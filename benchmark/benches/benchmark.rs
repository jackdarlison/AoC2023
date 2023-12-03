use criterion::{Criterion, criterion_group, criterion_main};

fn days() -> Vec<(&'static str, fn())> {
    vec![
        ("day01a", day01a::main),
        ("day01b", day01b::main),
        ("day02a", day02a::main),
        ("day02b", day02b::main),
        ("day03a", day03a::main),
        ("day03b", day03b::main),
    ]
}

fn bench(c: &mut Criterion) {
   days().iter().for_each(|(name, f)| {
    c.bench_function(name, |b| b.iter(|| f));
   })
}

criterion_group!(benches, bench);
criterion_main!(benches);