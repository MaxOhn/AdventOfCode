use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> u64 {
    let mut sum = 0;

    for range in input.split(',') {
        let (first, last) = range.split_once('-').unwrap();
        let first: u64 = first.parse().unwrap();
        let last: u64 = last.parse().unwrap();

        for n in first..=last {
            fn invalid(n: u64) -> bool {
                let mut digits = 0;
                let mut m = n;

                while m > 0 {
                    digits += 1;
                    m /= 10;
                }

                if digits % 2 == 1 {
                    return false;
                }

                let mut m = n;
                let factor = 10_u64.pow(digits / 2);

                m /= factor;

                let reconstruct = m + factor * m;

                n == reconstruct
            }

            if invalid(n) {
                sum += n;
            }
        }
    }

    sum
}

fn part2(input: &str) -> u64 {
    let mut sum = 0;

    for range in input.split(',') {
        let (first, last) = range.split_once('-').unwrap();
        let first: u64 = first.parse().unwrap();
        let last: u64 = last.parse().unwrap();

        for n in first..=last {
            fn invalid(n: u64) -> bool {
                let mut factor = 10;

                while factor < n {
                    let m = n % factor;

                    if m == n % (factor / 10) {
                        factor *= 10;

                        continue;
                    }

                    let mut reconstruct = m;

                    while reconstruct < n {
                        reconstruct *= factor;
                        reconstruct += m;

                        if reconstruct == n {
                            return true;
                        }
                    }

                    factor *= 10;
                }

                false
            }

            if invalid(n) {
                sum += n;
            }
        }
    }

    sum
}
