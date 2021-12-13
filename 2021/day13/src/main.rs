use std::{
    collections::HashSet,
    error::Error,
    fmt::Write,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

use util::{Parse, Pos2};

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
    let mut dots: Vec<Pos2<usize>> = Vec::new();

    while input.read_line(&mut line)? != 1 {
        let (left, right) = line.trim_end().split_once(',').unwrap();

        let left = left.as_bytes().parse();
        let right = right.as_bytes().parse();
        let pos = Pos2 { x: left, y: right };

        dots.push(pos);
        line.clear();
    }

    let p1 = {
        line.clear();
        input.read_line(&mut line)?;
        let line_ = line.trim_end().as_bytes();
        let axis = line_[11];
        let n = (&line_[13..]).parse();
        fold(axis, n, &mut dots);
        line.clear();

        dots.iter().collect::<HashSet<_>>().len()
    };

    let p1_elapsed = start.elapsed();

    while input.read_line(&mut line)? != 0 {
        let line_ = line.trim_end().as_bytes();

        let axis = line_[11];
        let n = (&line_[13..]).parse();
        fold(axis, n, &mut dots);

        line.clear();
    }

    let p2 = print(&dots);
    let p2_elapsed = start.elapsed();

    println!("Part 1: {} [{:?}]", p1, p1_elapsed); // 714µs
    println!("Part 2: [{:?}]\n{}", p2_elapsed, p2); // 767µs

    assert_eq!(p1, 716);

    Ok(())
}

fn fold(axis: u8, n: usize, dots: &mut [Pos2<usize>]) {
    if axis == b'x' {
        for dot in dots.iter_mut() {
            if dot.x > n {
                dot.x = n - (dot.x - n);
            }
        }
    } else {
        for dot in dots.iter_mut() {
            if dot.y > n {
                dot.y = n - (dot.y - n);
            }
        }
    }
}

fn print(dots: &[Pos2<usize>]) -> String {
    let mut max_x = 0;
    let mut max_y = 0;

    for dot in dots {
        max_x = max_x.max(dot.x);
        max_y = max_y.max(dot.y);
    }

    let mut grid = vec![vec![b'.'; max_x + 1]; max_y + 1];
    let mut ret = String::with_capacity(grid.len() + max_y);

    for dot in dots {
        grid[dot.y as usize][dot.x] = b'#';
    }

    for row in grid {
        let _ = writeln!(ret, "{}", std::str::from_utf8(&row).unwrap());
    }

    ret.pop();

    ret
}
