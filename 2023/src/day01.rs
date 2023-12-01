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
        fn find_digit<I: Iterator<Item = char>>(mut iter: I) -> Option<u32> {
            iter.find_map(|c| c.to_digit(10))
        }

        let mut iter = line.chars();

        let Some(first) = find_digit(&mut iter) else {
            continue;
        };

        let second = find_digit(iter.rev()).unwrap_or(first);

        sum += first * 10 + second;
    }

    sum
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;

    for line in input.lines().map(str::as_bytes) {
        fn find_digit<'a, I: Iterator<Item = &'a [u8]>>(mut iter: I) -> Option<u32> {
            static KV_MAP: &'static [(u8, &'static [u8])] = &[
                (b'0', b"zero"),
                (b'1', b"one"),
                (b'2', b"two"),
                (b'3', b"three"),
                (b'4', b"four"),
                (b'5', b"five"),
                (b'6', b"six"),
                (b'7', b"seven"),
                (b'8', b"eight"),
                (b'9', b"nine"),
            ];

            iter.find_map(|slice| {
                KV_MAP.iter().find(|(digit, word)| {
                    slice.starts_with(std::slice::from_ref(digit)) || slice.starts_with(word)
                })
            })
            .map(|(d, _)| (*d - b'0') as u32)
        }

        let mut iter = (0..line.len()).map(|i| &line[i..]);

        let Some(first) = find_digit(&mut iter) else {
            continue;
        };

        let second = find_digit(iter.rev()).unwrap_or(first);

        sum += first * 10 + second;
    }

    sum
}
