fn main() {
    divan::main();
}

const INPUT: &str = include_str!("../inputs/day05.txt").trim_ascii();

#[divan::bench(sample_count = 1000, sample_size = 10)]
fn part1() {
    aoc24::day05::part1(INPUT);
}

#[divan::bench(sample_count = 1000, sample_size = 10)]
fn part2() {
    aoc24::day05::part2(INPUT);
}
