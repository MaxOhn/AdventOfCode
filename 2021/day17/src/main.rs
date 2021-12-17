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
    input.read_line(&mut line)?;

    let (x, y) = &line[13..].trim_end().split_once(", ").unwrap();
    let (x_min, x_max) = x[2..].split_once("..").unwrap();
    let (y_min, y_max) = y[2..].split_once("..").unwrap();

    let x_min: i32 = x_min.parse().unwrap();
    let x_max: i32 = x_max.parse().unwrap();
    let y_min: i32 = y_min.parse().unwrap();
    let y_max: i32 = y_max.parse().unwrap();

    let mut p1 = 0;
    let mut p2 = 0;

    let lower_x = ((1.0 + 8.0 * x_min as f64).sqrt() / 2.0 - 0.5).ceil() as i32;

    for vx_init in lower_x..=x_max {
        for vy_init in y_min..=-y_min {
            let mut x = 0;
            let mut y = 0;

            let mut vx = vx_init;
            let mut vy = vy_init;

            let mut valid = false;
            let mut max_y = 0;

            while !(y + vy < y_min || (x < x_min && vx <= 0) || (x > x_max && vx >= 0)) {
                x += vx;
                y += vy;

                vx -= (vx > 0) as i32;
                vy -= 1;
                valid |= (x_min..=x_max).contains(&x) && (y_min..=y_max).contains(&y);
                max_y = max_y.max(y);
            }

            if valid {
                p1 = p1.max(max_y);
                p2 += 1;
            }
        }
    }

    let elapsed = start.elapsed();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Elapsed: {:?}", elapsed); // 432µs

    assert_eq!(p1, 3916);
    assert_eq!(p2, 2986);

    Ok(())
}
