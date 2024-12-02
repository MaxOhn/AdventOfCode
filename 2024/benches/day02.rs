fn main() {
    divan::main();
}

const INPUT: &str = include_str!("../inputs/day02.txt");

#[divan::bench]
fn bruteforce(bencher: divan::Bencher) {
    bencher.with_inputs(|| INPUT).bench_refs(|input| {
        aoc24::day02::part2_bruteforce(input);
    });
}

#[divan::bench]
fn recursive(bencher: divan::Bencher) {
    bencher.with_inputs(|| INPUT).bench_refs(|input| {
        aoc24::day02::part2_recursive(input);
    });
}
