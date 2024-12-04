fn main() {
    divan::main();
}

const INPUT: &str = include_str!("../inputs/day04.txt");

#[divan::bench]
fn naive(bencher: divan::Bencher) {
    bencher.with_inputs(|| INPUT).bench_refs(|input| {
        aoc24::day04::part1(input);
    });
}

#[divan::bench]
fn structured(bencher: divan::Bencher) {
    bencher.with_inputs(|| INPUT).bench_refs(|input| {
        aoc24::day04::part1_structured(input);
    });
}
