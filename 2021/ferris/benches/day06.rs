use criterion::*;

fn bench(c: &mut Criterion) {
    const INPUT: &[u8] = include_str!("../../day06/input.txt").as_bytes();
    c.bench_function("no simd", |b| {
        b.iter(|| aoc_ferris::day06::part1::no_simd(INPUT))
    });
    c.bench_function("simd", |b| b.iter(|| aoc_ferris::day06::part1::simd(INPUT)));
    c.bench_function("part2", |b| b.iter(|| aoc_ferris::day06::part2::run(INPUT)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
