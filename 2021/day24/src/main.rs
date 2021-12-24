use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

use util::Parse;

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
    let values = parse_values()?;
    let conditions = calculate_conditions(&values);

    let p1 = part1(&conditions);
    let p2 = part2(&conditions);
    let elapsed = start.elapsed();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Elapsed: {:?}", elapsed); // 120µs

    assert_eq!(p1, 93_499_629_698_999);
    assert_eq!(p2, 11_164_118_121_471);

    Ok(())
}

fn parse_values() -> Result<Vec<(isize, isize)>, Box<dyn Error>> {
    let file = File::open("./input")?;
    let mut input = BufReader::new(file);

    let mut line = String::new();
    let mut values = Vec::with_capacity(14);

    for _ in 0..14 {
        for _ in 0..5 {
            input.read_line(&mut line)?;
        }

        line.clear();
        input.read_line(&mut line)?;
        let add_x: isize = line.trim_end().as_bytes()[6..].parse();

        for _ in 0..9 {
            input.read_line(&mut line)?;
        }

        line.clear();
        input.read_line(&mut line)?;
        let add_y: isize = line.trim_end().as_bytes()[6..].parse();

        for _ in 0..2 {
            input.read_line(&mut line)?;
        }

        values.push((add_x, add_y));
    }

    Ok(values)
}

fn calculate_conditions(values: &[(isize, isize)]) -> Vec<(u32, u32, isize)> {
    let mut z = Vec::new();
    let mut conditions = Vec::new();

    for (w, (add_x, add_y)) in values.into_iter().enumerate() {
        if *add_x > 0 {
            z.push((w, add_y));
        } else if let Some((r, v)) = z.pop() {
            conditions.push((w as u32, r as u32, v + add_x));
        }
    }

    conditions
}

fn part1(conditions: &[(u32, u32, isize)]) -> isize {
    let mut answer = 99_999_999_999_999;

    for &(w, r, offset) in conditions {
        let (sign, shift) = if offset > 0 { (-1, r) } else { (1, w) };
        answer += sign * offset * 10_isize.pow(13 - shift);
    }

    answer
}

fn part2(conditions: &[(u32, u32, isize)]) -> isize {
    let mut answer = 11_111_111_111_111;

    for &(w, r, offset) in conditions {
        let (sign, shift) = if offset > 0 { (1, w) } else { (-1, r) };
        answer += sign * offset * 10_isize.pow(13 - shift);
    }

    answer
}
