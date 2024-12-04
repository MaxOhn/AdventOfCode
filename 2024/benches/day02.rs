fn main() {
    divan::main();
}

const INPUT: &str = include_str!("../inputs/day02.txt");

#[divan::bench]
fn bruteforce() {
    aoc24::day02::part2_bruteforce(INPUT);
}

#[divan::bench]
fn recursive() {
    aoc24::day02::part2_recursive(INPUT);
}
