use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2_recursive(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            line.split_ascii_whitespace()
                .map(str::parse::<i8>)
                .map(Result::unwrap)
                .is_safe()
        })
        .count()
}

pub fn part2_recursive(input: &str) -> usize {
    let mut buf = Vec::new();

    input
        .lines()
        .filter(|line| {
            let iter = line
                .split_ascii_whitespace()
                .map(str::parse::<i8>)
                .map(Result::unwrap);

            buf.clear();
            buf.extend(iter);

            is_safe(&buf)
        })
        .count()
}

fn is_safe(report: &[i8]) -> bool {
    let [first, second, rest @ ..] = report else {
        return true;
    };

    if let [third, rest @ ..] = rest {
        let state_a = State {
            ascending: *second < *third,
            removed: true,
        };

        let is_safe = is_safe_recursive(*third, rest, state_a);

        if within_range(*second, *third) && is_safe {
            return true;
        }

        let state_b = State {
            ascending: *first < *third,
            removed: true,
        };

        if within_range(*first, *third)
            && ((is_safe && state_a.ascending == state_b.ascending)
                || is_safe_recursive(*third, rest, state_b))
        {
            return true;
        }
    }

    let state = State {
        ascending: *first < *second,
        removed: false,
    };

    within_range(*first, *second) && is_safe_recursive(*second, rest, state)
}

#[derive(Copy, Clone)]
struct State {
    ascending: bool,
    removed: bool,
}

fn is_safe_recursive(curr: i8, remaining: &[i8], state: State) -> bool {
    let [next, rest @ ..] = remaining else {
        return state.removed;
    };

    if !state.removed {
        let mut state = state;
        state.removed = true;

        if is_safe_recursive(curr, rest, state) {
            return true;
        }
    }

    within_range(curr, *next)
        && ((state.ascending && curr < *next) || (!state.ascending && curr > *next))
        && is_safe_recursive(*next, rest, state)
}

fn within_range(a: i8, b: i8) -> bool {
    (1..=3).contains(&(a - b).abs())
}

pub fn part2_bruteforce(input: &str) -> usize {
    let mut buf = Vec::new();

    input
        .lines()
        .filter(|line| {
            let iter = line
                .split_ascii_whitespace()
                .map(str::parse::<i8>)
                .map(Result::unwrap);

            buf.clear();
            buf.extend(iter);

            (0..buf.len()).any(|i| {
                buf.iter()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, level)| *level)
                    .is_safe()
            })
        })
        .count()
}

trait Report {
    fn is_safe(self) -> bool;
}

impl<I: Iterator<Item = i8>> Report for I {
    fn is_safe(mut self) -> bool {
        fn check_remaining<I: Iterator<Item = i8>>(
            iter: I,
            init: i8,
            cmp: fn(&i8, &i8) -> bool,
        ) -> bool {
            iter.scan(init, |prev, curr| {
                let is_safe = cmp(prev, &curr) && within_range(*prev, curr);
                *prev = curr;

                Some(is_safe)
            })
            .all(std::convert::identity)
        }

        let (Some(first), Some(second)) = (self.next(), self.next()) else {
            return true;
        };

        let cmp = if first < second { i8::lt } else { i8::gt };

        within_range(first, second) && check_remaining(self, second, cmp)
    }
}
