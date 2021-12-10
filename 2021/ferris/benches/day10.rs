use criterion::*;

fn bench(c: &mut Criterion) {
    const INPUT: &[u8] = include_str!("../../day10/input").as_bytes();
    c.bench_function("arrayvec", |b| {
        b.iter(|| aoc_ferris::day10::part1::with_arrayvec(INPUT))
    });
    c.bench_function("vec", |b| {
        b.iter(|| aoc_ferris::day10::part1::with_vec(INPUT))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
