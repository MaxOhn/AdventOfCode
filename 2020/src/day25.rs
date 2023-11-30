use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let mut lines = input.lines();

    let card_key: usize = lines.next().unwrap().parse().unwrap();
    let door_key: usize = lines.next().unwrap().parse().unwrap();

    let mut val = 1;
    let mut encryption = 1;
    let subject_number = 7;

    while val != card_key {
        val = (subject_number * val) % 20_201_227;
        encryption = (encryption * door_key) % 20_201_227;
    }

    Ok(Solution::new().part1(encryption))
}
