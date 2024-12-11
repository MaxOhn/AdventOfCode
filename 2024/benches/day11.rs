fn main() {
    divan::main();
}

const INPUT: &str = include_str!("../inputs/day11.txt").trim_ascii();

#[divan::bench(sample_count = 1000, sample_size = 10)]
fn part1_sequential() {
    aoc24::day11::part1_sequential(INPUT);
}

#[divan::bench(sample_count = 100, sample_size = 10)]
fn part2_sequential() {
    aoc24::day11::part2_sequential(INPUT);
}

#[divan::bench(sample_count = 1000, sample_size = 10)]
fn part1_parallel() {
    aoc24::day11::part1_parallel(INPUT);
}

#[divan::bench(sample_count = 100, sample_size = 10)]
fn part2_parallel() {
    aoc24::day11::part2_parallel(INPUT);
}
