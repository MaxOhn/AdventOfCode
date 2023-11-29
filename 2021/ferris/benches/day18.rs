use criterion::*;

fn bench(c: &mut Criterion) {
    const INPUT: &[u8] = include_str!("../../day18/input.txt").as_bytes();
    c.bench_function("d18p1 double box", |b| {
        b.iter(|| aoc_ferris::day18::part1::double_box::run(INPUT))
    });
    c.bench_function("d18p1 single box", |b| {
        b.iter(|| aoc_ferris::day18::part1::single_box::run(INPUT))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
