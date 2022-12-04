fn main() {
    let start = std::time::Instant::now();
    let solution = aoc22::current::run();
    let elapsed = start.elapsed();

    print!("{solution}");
    println!("Elapsed: {elapsed:?}");
}
