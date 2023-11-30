use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/day01.txt").unwrap();
    let solution = aoc20::day01::run(&input).unwrap();
    println!("{solution}");
}
