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
    let mut stack = Vec::new();
    let mut p1 = 0;
    let mut scores = Vec::new();

    while input.read_line(&mut line)? != 0 {
        stack.clear();
        let before = p1;

        for c in line.trim_end().chars() {
            match c {
                '(' | '[' | '{' | '<' => stack.push(c),
                ')' => p1 += (stack.pop() != Some('(')) as usize * 3,
                ']' => p1 += (stack.pop() != Some('[')) as usize * 57,
                '}' => p1 += (stack.pop() != Some('{')) as usize * 1197,
                '>' => p1 += (stack.pop() != Some('<')) as usize * 25_137,
                _ => unreachable!("invalid {}", c),
            }
        }

        if p1 == before {
            let mut score = 0_u64;

            for &c in stack.iter().rev() {
                score *= 5;

                score += match c {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => unreachable!(),
                };
            }

            scores.push(score);
        }

        line.clear();
    }

    scores.sort_unstable();
    let p2 = scores[scores.len() / 2];
    let elapsed = start.elapsed();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Elapsed: {:?}", elapsed); // 170µs

    Ok(())
}
