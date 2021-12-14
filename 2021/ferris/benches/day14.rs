use criterion::*;

fn bench(c: &mut Criterion) {
    const INPUT: &[u8] = include_str!("../../day14/input").as_bytes();
    c.bench_function("part1 simple", |b| {
        b.iter(|| aoc_ferris::day14::part1::simple(INPUT))
    });
    c.bench_function("part1 cached", |b| {
        b.iter(|| aoc_ferris::day14::part1::cached(INPUT))
    });
    c.bench_function("part2 sync", |b| {
        b.iter(|| aoc_ferris::day14::part2::sync(INPUT))
    });
    c.bench_function("part2 parallel", |b| {
        b.iter(|| aoc_ferris::day14::part2::parallel(INPUT))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
