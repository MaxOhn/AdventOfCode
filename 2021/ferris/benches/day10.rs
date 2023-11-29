use criterion::*;

fn bench(c: &mut Criterion) {
    const INPUT: &[u8] = include_str!("../../day10/input.txt").as_bytes();
    c.bench_function("part1 arrayvec", |b| {
        b.iter(|| aoc_ferris::day10::part1::with_arrayvec(INPUT))
    });
    c.bench_function("part1 vec", |b| {
        b.iter(|| aoc_ferris::day10::part1::with_vec(INPUT))
    });
    c.bench_function("part2 arrayvec", |b| {
        b.iter(|| aoc_ferris::day10::part2::with_arrayvec(INPUT))
    });
    c.bench_function("part2 vec drain", |b| {
        b.iter(|| aoc_ferris::day10::part2::with_vec_drain(INPUT))
    });
    c.bench_function("part2 vec clear", |b| {
        b.iter(|| aoc_ferris::day10::part2::with_vec_clear(INPUT))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
