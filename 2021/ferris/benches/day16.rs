use criterion::*;

fn bench(c: &mut Criterion) {
    const INPUT: &[u8] = include_str!("../../day16/input.txt").as_bytes();
    c.bench_function("d16p1 vec", |b| {
        b.iter(|| aoc_ferris::day16::part1::with_vec::run(INPUT))
    });
    c.bench_function("d16p1 iter", |b| {
        b.iter(|| aoc_ferris::day16::part1::with_iter::run(INPUT))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
