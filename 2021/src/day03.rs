use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    const LEN: usize = 12;

    let mut ones = [0; LEN];
    let mut count = 0;

    let mut lines: Vec<_> = Vec::with_capacity(1000);

    // Parse input
    for line in input.lines() {
        count += 1;
        let mut num = 0;

        for (one, &byte) in ones.iter_mut().zip(line.as_bytes()) {
            *one += (byte == b'1') as usize;
            num = num * 2 + (byte == b'1') as u32;
        }

        lines.push(num);
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

    Ok(Solution::new().part1(p1).part2(p2))
}
