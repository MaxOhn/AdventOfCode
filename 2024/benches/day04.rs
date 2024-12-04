fn main() {
    divan::main();
}

const INPUT: &str = include_str!("../inputs/day04.txt");

#[divan::bench]
fn naive() {
    aoc24::day04::part1(INPUT);
}

#[divan::bench]
fn structured() {
    aoc24::day04::part1_structured(INPUT);
}
