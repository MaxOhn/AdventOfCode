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

    const LEN: usize = 12;

    let mut ones = [0; LEN];
    let mut count = 0;

    let mut lines: Vec<_> = Vec::with_capacity(1000);

    // Parse input
    while input.read_line(&mut line)? != 0 {
        count += 1;
        let mut num = 0;

        for (one, &byte) in ones.iter_mut().zip(line.as_bytes()) {
            *one += (byte == b'1') as usize;
            num = num * 2 + (byte == b'1') as u32;
        }

        lines.push(num);
        line.clear();
    }

    // Part 1
    let mut gamma = 0;
    let mut epsilon = 0;
    let half = count / 2;

    for amount in ones {
        gamma *= 2;
        epsilon *= 2;

        if amount > half {
            gamma += 1;
        } else if amount < half {
            epsilon += 1;
        }
    }

    let p1 = gamma * epsilon;
    let p1_elapsed = start.elapsed();

    // Use lines for co2 and clone for oxy
    let mut oxy_lines = lines.clone();
    let shift = LEN - 1;

    // Use ones from part 1 for first iteration
    let most_common = (ones[0] >= oxy_lines.len() / 2) as u32;
    oxy_lines.retain(|line| line >> shift == most_common);

    let least_common = (ones[0] < (lines.len() + lines.len() % 2) / 2) as u32;
    lines.retain(|line| line >> shift == least_common);

    // Do remaining iterations
    for i in 2..=LEN {
        let shift = LEN - i;
        let mask = 1 << shift;

        // Handle oxy
        if oxy_lines.len() > 1 {
            let mut oxy_ones = 0;

            for num in &oxy_lines {
                oxy_ones += (num & mask >= 1) as usize;
            }

            let most_common_bit =
                ((oxy_ones >= (oxy_lines.len() + oxy_lines.len() % 2) / 2) as u32) << shift;
            oxy_lines.retain(|line| line & mask == most_common_bit);
        }

        // Handle co2
        if lines.len() > 1 {
            let mut co2_ones = 0;

            for num in &lines {
                co2_ones += (num & mask > 1) as usize;
            }

            let least_common_bit =
                ((co2_ones < (lines.len() + lines.len() % 2) / 2) as u32) << shift;
            lines.retain(|line| line & mask == least_common_bit);
        }
    }

    let oxy = oxy_lines[0];
    let co2 = lines[0];
    let p2 = oxy * co2;
    let p2_elapsed = start.elapsed();

    println!("Part 1: {} [{:?}]", p1, p1_elapsed); // 154µs
    println!("Part 2: {} [{:?}]", p2, p2_elapsed); // 181µs

    assert_eq!(p1, 3_885_894);
    assert_eq!(p2, 4_375_225);

    Ok(())
}
