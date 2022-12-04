fn main() {
    let start = std::time::Instant::now();

    match aoc22::current::run() {
        Ok(solution) => {
            let elapsed = start.elapsed();
            print!("{solution}");
            println!("Elapsed: {elapsed:?}");
        }
        Err(err) => eprintln!("{err:?}"),
    }
}
