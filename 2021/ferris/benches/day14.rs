use criterion::*;

fn bench(c: &mut Criterion) {
    const INPUT: &[u8] = include_str!("../../day14/input").as_bytes();
    c.bench_function("d14p1 simple map pairs", |b| {
        b.iter(|| aoc_ferris::day14::part1::simple(INPUT))
    });
    c.bench_function("d14p1 cached map pairs", |b| {
        b.iter(|| aoc_ferris::day14::part1::cached(INPUT))
    });
    c.bench_function("d14p1 cached array pairs", |b| {
        b.iter(|| aoc_ferris::day14::part1::array_pairs::run(INPUT))
    });
    c.bench_function("d14p1 cached array pairs counts", |b| {
        b.iter(|| aoc_ferris::day14::part1::array_pairs_counts::run(INPUT))
    });
    c.bench_function("d14p2 sync", |b| {
        b.iter(|| aoc_ferris::day14::part2::sync(INPUT))
    });
    c.bench_function("d14p2 parallel", |b| {
        b.iter(|| aoc_ferris::day14::part2::parallel(INPUT))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
