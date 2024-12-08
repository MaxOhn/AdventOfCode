fn main() {
    divan::main();
}

const INPUT: &str = include_str!("../inputs/day07.txt").trim_ascii();

#[divan::bench(sample_count = 100)]
fn part1_recursive() {
    aoc24::day07::part1_recursive(INPUT);
}

#[divan::bench(sample_count = 100)]
fn part1_dynamic() {
    aoc24::day07::part1_dynamic(INPUT);
}

#[divan::bench(sample_count = 100)]
fn part2_recursive() {
    aoc24::day07::part2_recursive(INPUT);
}

#[divan::bench(sample_count = 100)]
fn part2_dynamic() {
    aoc24::day07::part2_dynamic(INPUT);
}
