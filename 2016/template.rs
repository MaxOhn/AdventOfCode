use std::time::Instant;

fn main() {
    let input = std::fs::read_to_string("./input").unwrap();

    let start = Instant::now();
    let p1 = part1(&input);
    println!("Part 1: {} [{:?}]", p1, start.elapsed());

    let start = Instant::now();
    let p2 = part2(&input);
    println!("Part 2: {} [{:?}]", p2, start.elapsed());

    assert_eq!(p1, 0);
    assert_eq!(p2, 0);
}

fn part1(input: &str) -> usize {
    0
}

fn part2(_input: &str) -> usize {
    0
}
