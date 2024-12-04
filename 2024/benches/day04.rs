fn main() {
    divan::main();
}

const INPUT: &str = include_str!("../inputs/day04.txt");

#[divan::bench(sample_count = 1000)]
fn naive() {
    aoc24::day04::part1(INPUT);
}

#[divan::bench(sample_count = 1000)]
fn structured() {
    aoc24::day04::part1_structured(INPUT);
}
