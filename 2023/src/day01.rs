use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines() {
        let mut chars = line.chars();

        fn find_digit<I: Iterator<Item = char>>(iter: &mut I) -> Option<u32> {
            iter.find_map(|c| c.to_digit(10))
        }

        let Some(first) = find_digit(&mut chars) else {
            continue;
        };

        let second = find_digit(&mut chars.rev()).unwrap_or(first);

        sum += first * 10 + second;
    }

    sum
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines().map(str::as_bytes) {
        fn find_digit<'a, I: Iterator<Item = &'a [u8]>>(iter: &mut I) -> Option<u32> {
            static KV_MAP: &'static [((u8, &'static [u8]), u32)] = &[
                ((b'0', b"zero"), 0),
                ((b'1', b"one"), 1),
                ((b'2', b"two"), 2),
                ((b'3', b"three"), 3),
                ((b'4', b"four"), 4),
                ((b'5', b"five"), 5),
                ((b'6', b"six"), 6),
                ((b'7', b"seven"), 7),
                ((b'8', b"eight"), 8),
                ((b'9', b"nine"), 9),
            ];

            iter.find_map(|slice| {
                KV_MAP.iter().find(|((digit, word), _)| {
                    slice.starts_with(std::slice::from_ref(digit)) || slice.starts_with(word)
                })
            })
            .map(|(_, v)| *v)
        }

        let mut slices = (0..line.len()).map(|i| &line[i..]);

        let Some(first) = find_digit(&mut slices) else {
            continue;
        };

        let second = find_digit(&mut slices.rev()).unwrap_or(first);

        sum += first * 10 + second;
    }

    sum
}
