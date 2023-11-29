use criterion::*;

fn bench(c: &mut Criterion) {
    const INPUT: &[u8] = include_str!("../../day08/input.txt").as_bytes();
    c.bench_function("short", |b| {
        b.iter(|| aoc_ferris::day08::part2::short(INPUT))
    });
    c.bench_function("long", |b| b.iter(|| aoc_ferris::day08::part2::long(INPUT)));
}

criterion_group!(benches, bench);
criterion_main!(benches);
