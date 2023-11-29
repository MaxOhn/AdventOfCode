use criterion::*;

fn bench(c: &mut Criterion) {
    const INPUT: &[u8] = include_str!("../../day07/input.txt").as_bytes();
    c.bench_function("part1_median", |b| {
        b.iter(|| aoc_ferris::day07::part1::median(INPUT))
    });
    c.bench_function("part1_regular", |b| {
        b.iter(|| aoc_ferris::day07::part1::regular(INPUT))
    });
    c.bench_function("part2_average", |b| {
        b.iter(|| aoc_ferris::day07::part2::average(INPUT))
    });
    c.bench_function("part2_regular", |b| {
        b.iter(|| aoc_ferris::day07::part2::regular(INPUT))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
