use criterion::*;

fn bench(c: &mut Criterion) {
    const INPUT: &[u8] = include_str!("../../day13/input").as_bytes();
    c.bench_function("part1 std", |b| {
        b.iter(|| aoc_ferris::day13::part1::stdhash(INPUT))
    });
    c.bench_function("part1 fx", |b| {
        b.iter(|| aoc_ferris::day13::part1::fxhash(INPUT))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
