use crate::{Error, Solution};
use std::str::FromStr;

pub fn solve(input: String) -> Result<Solution<usize, usize>, Error> {
    let p1 = part1(input.as_str());
    let p2 = part2(input.as_str());

    Ok(Solution::new(p1, p2))
}

fn part1(input: &str) -> usize {
    let mut shuffle: Vec<_> = input
        .lines()
        .map(Step::from_str)
        .map(Result::unwrap)
        .collect();

    const LEN: usize = 10_007;

    minimize_shuffle(&mut shuffle, LEN);

    let mut p1 = 2019;

    for step in shuffle {
        p1 = step.predict_next(p1, LEN);
    }

    p1
}

fn part2(input: &str) -> usize {
    let len = 119_315_717_514_047;
    let mut iters: usize = 101_741_582_076_661;

    let mut shuffle: Vec<_> = input
        .lines()
        .map(Step::from_str)
        .map(Result::unwrap)
        .collect();

    minimize_shuffle(&mut shuffle, len);

    let mut final_shuffle = Vec::with_capacity(6);

    // Basically add n many copies of the shuffle to itself with n
    // being the largest exponent of two that is smaller than `iters`
    while iters > 0 {
        let mut pow = if iters.is_power_of_two() {
            iters
        } else {
            iters.next_power_of_two() / 2
        };

        iters -= pow;

        let mut curr = shuffle.clone();

        while pow > 1 {
            curr.append(&mut curr.clone());
            minimize_shuffle(&mut curr, len);
            pow /= 2;
        }

        final_shuffle.append(&mut curr);
        minimize_shuffle(&mut final_shuffle, len);
    }

    let mut p2 = 2020;

    for &step in final_shuffle.iter().rev() {
        p2 = step.predict_prev(p2, len);
    }

    p2
}

// Swap & remove elements until the shuffle contains only one element per Step variant
fn minimize_shuffle(shuffle: &mut Vec<Step>, len: usize) {
    let mut changed = true;

    while changed {
        changed = false;
        let mut i = 0;

        while i < shuffle.len() - 1 {
            match (shuffle[i], shuffle[i + 1]) {
                // Simple cases
                (Step::NewStack, Step::NewStack) => {
                    shuffle.remove(i + 1);
                    shuffle.remove(i);
                    changed = true;
                    i = i.saturating_sub(1);
                }
                (Step::Cut(a), Step::Cut(b)) => {
                    shuffle.remove(i + 1);
                    shuffle[i] = Step::Cut((a + b) % len as isize);
                    changed = true;
                }
                (Step::Increment(a), Step::Increment(b)) => {
                    shuffle.remove(i + 1);
                    shuffle[i] = Step::Increment(usize::mul_mod(a, b, len));
                    changed = true;
                }

                // Cross cases
                (Step::Cut(a), Step::Increment(b)) => {
                    shuffle.swap(i, i + 1);
                    shuffle[i + 1] = Step::Cut(isize::mul_mod(a, b as isize, len as isize));
                    changed = true;
                    i += 1;
                }
                (Step::NewStack, Step::Increment(a)) => {
                    shuffle.swap(i, i + 1);
                    shuffle[i + 1] = Step::Cut((1 - a as isize) % len as isize);
                    shuffle.insert(i + 2, Step::NewStack);
                    changed = true;
                    i += 2;
                }
                (Step::NewStack, Step::Cut(a)) => {
                    shuffle.swap(i, i + 1);
                    shuffle[i] = Step::Cut((len as isize - a) % len as isize);
                    changed = true;
                    i += 1;
                }
                _ => i += 1,
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Step {
    NewStack,
    Cut(isize),
    Increment(usize),
}

impl FromStr for Step {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');

        let technique = match (split.next().unwrap(), split.next().unwrap()) {
            ("cut", n) => Self::Cut(n.parse().unwrap()),
            ("deal", "into") => Self::NewStack,
            ("deal", "with") => Self::Increment(split.nth(1).unwrap().parse().unwrap()),
            _ => return Err(()),
        };

        Ok(technique)
    }
}

impl Step {
    // No longer required, replaced by predict_next and predict_prev
    fn _apply(self, cards: &mut [usize], buf: &mut [usize]) {
        match self {
            Self::NewStack => cards.reverse(),
            Self::Cut(n) => {
                if n < 0 {
                    let mid = -n as usize % cards.len();
                    cards.rotate_right(mid);
                } else {
                    let mid = n as usize % cards.len();
                    cards.rotate_left(mid as usize);
                }
            }
            Self::Increment(n) => {
                for i in 0..cards.len() {
                    buf[(n * i) % cards.len()] = cards[i];
                }
                cards.swap_with_slice(buf);
            }
        }
    }

    // To which position will position `pos` be mapped after one shuffle
    fn predict_next(self, pos: usize, len: usize) -> usize {
        match self {
            Self::NewStack => len - pos - 1,
            Self::Increment(n) => (pos * n) % len,
            Self::Cut(n) => ((pos as isize - n + len as isize) as usize) % len,
        }
    }

    // What position is being mapped to position `pos` after one shuffle
    fn predict_prev(self, pos: usize, len: usize) -> usize {
        match self {
            Self::NewStack => len - pos - 1,
            Self::Increment(n) => linear_congruence(n, pos, len).unwrap(),
            Self::Cut(n) => (pos as isize + n + len as isize) as usize % len,
        }
    }
}

trait Ops {
    /// `(a * b) % m` without overflow
    fn mul_mod(a: Self, b: Self, m: Self) -> Self;
}

macro_rules! impl_ops {
    ($type:ty) => {
        impl Ops for $type {
            fn mul_mod(mut a: Self, mut b: Self, m: Self) -> Self {
                let mut res = 0;
                a %= m;

                while b > 0 {
                    if b % 2 == 1 {
                        res = (res + a) % m;
                    }

                    a = (a * 2) % m;
                    b /= 2;
                }

                res % m
            }
        }
    };
}

impl_ops!(isize);
impl_ops!(usize);

// Thanks https://www.youtube.com/watch?v=XoTEKjS61kI
fn linear_congruence(a: usize, b: usize, m: usize) -> Option<usize> {
    mod_inv(a, m).map(|i| usize::mul_mod(b, i, m))
}

fn mod_inv(x: usize, n: usize) -> Option<usize> {
    let (g, x, _) = egcd(x as isize, n as isize);
    if g == 1 {
        Some((x % n as isize + n as isize) as usize % n)
    } else {
        None
    }
}

fn egcd(a: isize, b: isize) -> (isize, isize, isize) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test22() {
        crate::util::tests::test_full_problem(22, solve, 4284, 96_797_432_275_571);
    }
}
