use criterion::*;

#[allow(unused)]
fn day02(c: &mut Criterion) {
    const INPUT: &[u8] = include_str!("../inputs/day02.txt").as_bytes();
    c.bench_function("naive", |b| b.iter(|| aoc22::day02::part1_naive(INPUT)));
    c.bench_function("const lookup", |b| {
        b.iter(|| aoc22::day02::part1_const_lookup(INPUT))
    });

    #[cfg(feature = "nightly")]
    {
        c.bench_function("simd", |b| b.iter(|| aoc22::day02::part1_simd(INPUT)));
        c.bench_function("simd+rayon", |b| {
            b.iter(|| aoc22::day02::part1_simd_rayon(INPUT))
        });
    }
}

#[allow(unused)]
fn day06(c: &mut Criterion) {
    const INPUT: &str = include_str!("../inputs/day06.txt");
    c.bench_function("array", |b| {
        b.iter(|| aoc22::day06::solve_with_array(INPUT))
    });
    c.bench_function("bitflags", |b| {
        b.iter(|| aoc22::day06::solve_with_bitflags(INPUT))
    });
    c.bench_function("dynamic", |b| {
        b.iter(|| aoc22::day06::solve_with_dynamic(INPUT))
    });
}

#[allow(unused)]
fn day12(c: &mut Criterion) {
    const INPUT: &str = include_str!("../inputs/day12.txt");
    c.bench_function("Dijkstra", |b| b.iter(|| aoc22::day12::run_dijkstra(INPUT)));
    c.bench_function("A*", |b| b.iter(|| aoc22::day12::run_a_star(INPUT)));
}

criterion_group!(benches, day12);
criterion_main!(benches);
