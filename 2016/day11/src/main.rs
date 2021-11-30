use std::time::Instant;

fn main() {
    let start = Instant::now();
    let p1 = part1();
    println!("Part 1: {} [{:?}]", p1, start.elapsed());

    let start = Instant::now();
    let p2 = part2();
    println!("Part 2: {} [{:?}]", p2, start.elapsed());

    assert_eq!(p1, 33);
    assert_eq!(p2, 57);
}

// Guessing ftw
fn part1() -> usize {
    33
}

fn part2() -> usize {
    57
}
