use criterion::*;

fn bench(c: &mut Criterion) {
    const INPUT: &[u8] = include_str!("../../day13/input").as_bytes();
    c.bench_function("part1 std", |b| {
        b.iter(|| aoc_ferris::day13::part1::stdhash(INPUT))
    });
    c.bench_function("part1 fx", |b| {
        b.iter(|| aoc_ferris::day13::part1::fxhash(INPUT))
    });
    c.bench_function("part2 branches", |b| {
        b.iter(|| aoc_ferris::day13::part2::with_branches(INPUT))
    });
    c.bench_function("part2 branchless", |b| {
        b.iter(|| aoc_ferris::day13::part2::without_branches(INPUT))
    });
    c.bench_function("part2 branchless abs", |b| {
        b.iter(|| aoc_ferris::day13::part2::without_branches_abs(INPUT))
    });
    c.bench_function("part2 branchless combined", |b| {
        b.iter(|| aoc_ferris::day13::part2::without_branches_combined(INPUT))
    });
    c.bench_function("part2 branchless combined pointers", |b| {
        b.iter(|| aoc_ferris::day13::part2::without_branches_combined_pointers(INPUT))
    });
    c.bench_function("part2 branched combined pointers", |b| {
        b.iter(|| aoc_ferris::day13::part2::branched_combined_pointers(INPUT))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
