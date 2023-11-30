use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let mut grid = vec![[0_u8; 1000]; 1000];

    const BOTH_PARTS: u8 = 0b0001_0001;
    const PART2: u8 = 0b0000_0001;

    for line in input.lines() {
        let bytes = line.trim_end().as_bytes();

        let mut i = 0;
        let x1 = parse_num(bytes, &mut i, b',');
        i += 1;
        let y1 = parse_num(bytes, &mut i, b' ');
        i += 4;
        let x2 = parse_num(bytes, &mut i, b',');
        i += 1;

        let mut y2 = (bytes[i] & 0x0F) as usize;
        i += 1;

        while i < bytes.len() {
            y2 = y2 * 10 + (bytes[i] & 0x0F) as usize;
            i += 1
        }

        if x1 == x2 {
            let range = if y1 < y2 { y1..=y2 } else { y2..=y1 };

            for y in range {
                grid[y][x1] += BOTH_PARTS;
            }
        } else if y1 == y2 {
            let range = if x1 < x2 { x1..=x2 } else { x2..=x1 };

            for x in range {
                grid[y1][x] += BOTH_PARTS;
            }
        } else if (x1 > x2 && y1 > y2) || (x2 > x1 && y2 > y1) {
            let (x_range, y_range) = if x1 < x2 {
                (x1..=x2, y1..=y2)
            } else {
                (x2..=x1, y2..=y1)
            };

            for (x, y) in x_range.zip(y_range) {
                grid[y][x] += PART2;
            }
        } else {
            let x_range = if x1 < x2 { x1..=x2 } else { x2..=x1 };
            let y_range = if y1 < y2 { y1..=y2 } else { y2..=y1 };

            for (x, y) in x_range.rev().zip(y_range) {
                grid[y][x] += PART2;
            }
        }
    }

    let mut p1 = 0;
    let mut p2 = 0;

    for row in grid {
        for count in row {
            p1 += (count >= 0b0010_0000) as usize;
            p2 += ((count & 0b0000_1111) > 1) as usize;
        }
    }

    Ok(Solution::new().part1(p1).part2(p2))
}

fn parse_num(bytes: &[u8], i: &mut usize, delim: u8) -> usize {
    let mut n = (bytes[*i] & 0x0F) as usize;

    loop {
        *i += 1;

        if bytes[*i] == delim {
            return n;
        }

        n = n * 10 + (bytes[*i] & 0x0F) as usize;
    }
}
