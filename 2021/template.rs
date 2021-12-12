use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        let mut e: &dyn Error = &*err;

        while let Some(src) = e.source() {
            eprintln!("  - caused by: {}", src);
            e = src;
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let file = File::open("./input")?;
    let mut input = BufReader::new(file);

    let mut line = String::new();
    let mut answer = 0;

    while input.read_line(&mut line)? != 0 {
        //
        line.clear();
    }

    println!("Answer: {} {:?}", answer, start.elapsed()); //

    // let start = Instant::now();
    // let p1 = part1();
    // println!("Part 1: {} [{:?}]", p1, start.elapsed()); //

    // let start = Instant::now();
    // let p2 = part2();
    // println!("Part 2: {} [{:?}]", p2, start.elapsed()); //

    // assert_eq!(p1, 0);
    // assert_eq!(p2, 0);

    Ok(())
}
