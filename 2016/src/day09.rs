use aoc_rust::Solution;

pub fn run(input: &str) -> eyre::Result<Solution> {
    let input = input.as_bytes();

    Ok(Solution::new().part1(part1(input)).part2(part2(input)))
}

pub fn part1(bytes: &[u8]) -> usize {
    let mut i = 0;
    let mut len = 0;

    while i < bytes.len() {
        match get!(bytes, i) {
            b' ' => i += 1,
            b'(' => match parse_marker(get_ref!(bytes, i + 1..)) {
                Some((size, reps, j)) => {
                    i += j + 2;
                    let spaces = bytecount::count(get_ref!(bytes, i..i + size), b' ');
                    len += reps * (size - spaces);
                    i += size;
                }
                None => {
                    len += 1;
                    i += 1
                }
            },
            _ => {
                len += 1;
                i += 1;
            }
        }
    }

    len
}

pub fn part2(bytes: &[u8]) -> usize {
    let mut i = 0;
    let mut len = 0;

    while i < bytes.len() {
        match get!(bytes, i) {
            b' ' => i += 1,
            b'(' => match parse_marker(get_ref!(bytes, i + 1..)) {
                Some((size, reps, j)) => {
                    i += j + 2;
                    let decompressed = part2(get_ref!(bytes, i..i + size));
                    len += reps * decompressed;
                    i += size;
                }
                None => {
                    len += 1;
                    i += 1
                }
            },
            _ => {
                len += 1;
                i += 1;
            }
        }
    }

    len
}

fn parse_marker(bytes: &[u8]) -> Option<(usize, usize, usize)> {
    let mut i = 0;
    let mut a = None;
    let mut b = None;

    loop {
        match get!(bytes, i) {
            n if n.is_ascii_digit() => {
                let val = a.get_or_insert(0);
                *val = *val * 10 + (n & 0xF) as usize;
            }
            b'x' => swap!(&mut a, &mut b),
            b')' => return Some((b?, a?, i)),
            _ => return None,
        }

        i += 1;
    }
}
