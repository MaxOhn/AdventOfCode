use std::{fs::read_to_string, time::Instant};

fn main() {
    let input = read_to_string("./input.txt").unwrap();

    let start = Instant::now();
    let solution = day07::run(&input);
    let elapsed = start.elapsed();

    print!("{}", solution.unwrap());
    println!("Elapsed: {elapsed:?}");
}
