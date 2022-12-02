use criterion::*;

fn day02(c: &mut Criterion) {
    const INPUT: &[u8] = include_str!("../inputs/day02.txt").as_bytes();
    c.bench_function("naive", |b| b.iter(|| aoc22::day02::part1_naive(INPUT)));
    c.bench_function("simd", |b| b.iter(|| aoc22::day02::part1_simd(INPUT)));
    c.bench_function("simd+rayon", |b| {
        b.iter(|| aoc22::day02::part1_simd_rayon(INPUT))
    });
}

criterion_group!(benches, day02);
criterion_main!(benches);
