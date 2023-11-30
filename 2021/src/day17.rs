use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let (x, y) = &input[13..].trim_end().split_once(", ").unwrap();
    let (x_min, x_max) = x[2..].split_once("..").unwrap();
    let (y_min, y_max) = y[2..].split_once("..").unwrap();

    let x_min: i32 = x_min.parse()?;
    let x_max: i32 = x_max.parse()?;
    let y_min: i32 = y_min.parse()?;
    let y_max: i32 = y_max.parse()?;

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

    Ok(Solution::new().part1(p1).part2(p2))
}
