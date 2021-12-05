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
    let mut grid = vec![[0; 1000]; 1000];

    while input.read_line(&mut line)? != 0 {
        let mut split = line.split(" -> ");
        let mut first = split.next().unwrap().split(',');
        let mut second = split.next().unwrap().split(',');

        let x1: usize = first.next().unwrap().parse().unwrap();
        let y1: usize = first.next().unwrap().parse().unwrap();
        let x2: usize = second.next().unwrap().parse().unwrap();
        let y2: usize = second.next().unwrap().trim_end().parse().unwrap();

        if x1 == x2 {
            let min = y1.min(y2);
            let max = y1.max(y2);

            for y in min..=max {
                grid[y][x1] += 1;
            }
        } else if y1 == y2 {
            let min = x1.min(x2);
            let max = x1.max(x2);

            for x in min..=max {
                grid[y1][x] += 1;
            }
        } else {
            if (x1 > x2 && y1 > y2) || (x2 > x1 && y2 > y1) {
                let x_min = x1.min(x2);
                let x_max = x1.max(x2);
                let y_min = y1.min(y2);
                let y_max = y1.max(y2);

                for (x, y) in (x_min..=x_max).zip(y_min..=y_max) {
                    grid[y][x] += 1;
                }
            } else {
                let x_min = x1.min(x2);
                let x_max = x1.max(x2);
                let y_min = y1.min(y2);
                let y_max = y1.max(y2);

                for (x, y) in (x_min..=x_max).rev().zip(y_min..=y_max) {
                    grid[y][x] += 1;
                }
            }
        }

        line.clear();
    }

    let mut answer = 0;

    for row in grid {
        for count in row {
            answer += (count > 1) as usize;
        }
    }

    let elapsed = start.elapsed();
    println!("Answer: {} [{:?}]", answer, elapsed);

    Ok(())
}
