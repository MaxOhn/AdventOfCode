use aoc_rust::Solution;
use eyre::Report;

use self::{
    image::{Image, Pixel},
    rule::{Rule23, Rule34, Rules},
};

pub fn run(input: &str) -> Result<Solution, Report> {
    let rules: Rules = input.trim().parse()?;

    let p1 = part1(&rules)?;
    let p2 = part2(&rules)?;

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(rules: &Rules) -> Result<usize, Report> {
    solve(rules, 5)
}

fn part2(rules: &Rules) -> Result<usize, Report> {
    solve(rules, 18)
}

fn solve(rules: &Rules, iters: usize) -> Result<usize, Report> {
    let mut image = Image::default();

    for _ in 0..iters {
        image.iterate(rules)?;
    }

    Ok(image.count_on())
}

mod rule {
    use std::{mem, ops::Index, str::FromStr};

    use eyre::{ContextCompat, Report, WrapErr};

    use super::Pixel;

    pub struct Rules {
        pub even: Vec<Rule23>,
        pub odd: Vec<Rule34>,
    }

    impl FromStr for Rules {
        type Err = Report;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut even = Vec::new();
            let mut odd = Vec::new();

            for line in s.lines() {
                match line.parse()? {
                    Rule::Rule23(rule) => even.push(rule),
                    Rule::Rule34(rule) => odd.push(rule),
                }
            }

            Ok(Self { even, odd })
        }
    }

    #[derive(PartialEq, Eq)]
    pub struct Square<const N: usize>([[Pixel; N]; N]);

    impl Square<2> {
        fn flip(&mut self) {
            let [a, b] = &mut self.0;
            a.swap(0, 1);
            b.swap(0, 1);
        }

        fn rotate(&mut self) {
            let [a, b] = &mut self.0;
            a.swap(0, 1);
            b.swap(0, 1);
            mem::swap(&mut a[1], &mut b[0]);
        }
    }

    impl Square<3> {
        fn flip(&mut self) {
            let [a, b, c] = &mut self.0;
            a.swap(0, 2);
            b.swap(0, 2);
            c.swap(0, 2);
        }

        fn rotate(&mut self) {
            let [a, b, c] = &mut self.0;
            a.swap(0, 2);
            c.swap(0, 2);
            mem::swap(&mut a[2], &mut c[0]);
            mem::swap(&mut a[1], &mut b[0]);
            mem::swap(&mut c[1], &mut b[2]);
            mem::swap(&mut a[1], &mut c[1]);
        }
    }

    impl<const N: usize> From<[[Pixel; N]; N]> for Square<N> {
        fn from(square: [[Pixel; N]; N]) -> Self {
            Self(square)
        }
    }

    impl<const N: usize> Index<usize> for Square<N> {
        type Output = [Pixel; N];

        fn index(&self, index: usize) -> &Self::Output {
            self.0.index(index)
        }
    }

    pub struct Rule23 {
        pub from: Square<2>,
        pub to: Square<3>,
    }

    impl Rule23 {
        pub fn matches(&self, mut square: Square<2>) -> bool {
            if self.rotate_search(&mut square) {
                return true;
            }

            square.flip();

            self.rotate_search(&mut square)
        }

        fn rotate_search(&self, square: &mut Square<2>) -> bool {
            for _ in 0..4 {
                if &self.from == square {
                    return true;
                }

                square.rotate();
            }

            false
        }
    }

    pub struct Rule34 {
        pub from: Square<3>,
        pub to: Square<4>,
    }

    impl Rule34 {
        pub fn matches(&self, mut square: Square<3>) -> bool {
            if self.rotate_search(&mut square) {
                return true;
            }

            square.flip();

            self.rotate_search(&mut square)
        }

        fn rotate_search(&self, square: &mut Square<3>) -> bool {
            for _ in 0..4 {
                if &self.from == square {
                    return true;
                }

                square.rotate();
            }

            false
        }
    }

    enum Rule {
        Rule23(Rule23),
        Rule34(Rule34),
    }

    impl FromStr for Rule {
        type Err = Report;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (from, to) = s.split_once(" => ").wrap_err("missing arrow")?;

            let (a, b) = from.split_once('/').wrap_err("missing slash")?;

            let rule = match b.split_once('/') {
                Some((b, c)) => {
                    let mut split = to.split('/');

                    let (Some(w), Some(x), Some(y), Some(z)) =
                        (split.next(), split.next(), split.next(), split.next())
                    else {
                        eyre::bail!("invalid rule")
                    };

                    let from1: [u8; 3] = a.as_bytes().try_into().wrap_err("invalid rule")?;
                    let from2: [u8; 3] = b.as_bytes().try_into().wrap_err("invalid rule")?;
                    let from3: [u8; 3] = c.as_bytes().try_into().wrap_err("invalid rule")?;
                    let to1: [u8; 4] = w.as_bytes().try_into().wrap_err("invalid rule")?;
                    let to2: [u8; 4] = x.as_bytes().try_into().wrap_err("invalid rule")?;
                    let to3: [u8; 4] = y.as_bytes().try_into().wrap_err("invalid rule")?;
                    let to4: [u8; 4] = z.as_bytes().try_into().wrap_err("invalid rule")?;

                    Rule::Rule34(Rule34 {
                        from: [from1, from2, from3].into(),
                        to: [to1, to2, to3, to4].into(),
                    })
                }
                None => {
                    let mut split = to.split('/');

                    let (Some(x), Some(y), Some(z)) = (split.next(), split.next(), split.next())
                    else {
                        eyre::bail!("invalid rule")
                    };

                    let from1: [u8; 2] = a.as_bytes().try_into().wrap_err("invalid rule")?;
                    let from2: [u8; 2] = b.as_bytes().try_into().wrap_err("invalid rule")?;
                    let to1: [u8; 3] = x.as_bytes().try_into().wrap_err("invalid rule")?;
                    let to2: [u8; 3] = y.as_bytes().try_into().wrap_err("invalid rule")?;
                    let to3: [u8; 3] = z.as_bytes().try_into().wrap_err("invalid rule")?;

                    Rule::Rule23(Rule23 {
                        from: [from1, from2].into(),
                        to: [to1, to2, to3].into(),
                    })
                }
            };

            Ok(rule)
        }
    }
}

mod image {
    use std::convert;

    use eyre::Result;

    use super::{Rule23, Rule34, Rules};

    pub struct Image {
        size: usize,
        image: Vec<Vec<Pixel>>,
    }

    impl Image {
        pub fn count_on(&self) -> usize {
            self.image
                .iter()
                .flat_map(convert::identity)
                .filter(|&&pixel| pixel == b'#')
                .count()
        }

        pub fn iterate(&mut self, rules: &Rules) -> Result<()> {
            if self.size % 2 == 0 {
                self.iterate_even(&rules.even)
            } else {
                self.iterate_odd(&rules.odd)
            }
        }

        fn iterate_even(&mut self, rules: &[Rule23]) -> Result<()> {
            let w = self.size;

            let mut next = vec![vec![b'0'; self.size / 2 * 3]; self.size / 2 * 3];

            for y in (0..w - 1).step_by(2) {
                for x in (0..w - 1).step_by(2) {
                    let window = [
                        [self.image[y][x], self.image[y][x + 1]],
                        [self.image[y + 1][x], self.image[y + 1][x + 1]],
                    ];

                    let Some(rule) = rules.iter().find(|rule| rule.matches(window.into())) else {
                        eyre::bail!("no matching rule")
                    };

                    let dy = y / 2 * 3;
                    let dx = x / 2 * 3;

                    next[dy][dx..][..3].copy_from_slice(&rule.to[0]);
                    next[dy + 1][dx..][..3].copy_from_slice(&rule.to[1]);
                    next[dy + 2][dx..][..3].copy_from_slice(&rule.to[2]);
                }
            }

            self.size /= 2;
            self.size *= 3;
            self.image = next;

            Ok(())
        }

        fn iterate_odd(&mut self, rules: &[Rule34]) -> Result<()> {
            let w = self.size;
            let mut next = vec![vec![b'0'; self.size / 3 * 4]; self.size / 3 * 4];

            for y in (0..w - 2).step_by(3) {
                for x in (0..w - 2).step_by(3) {
                    let window = [
                        [self.image[y][x], self.image[y][x + 1], self.image[y][x + 2]],
                        [
                            self.image[y + 1][x],
                            self.image[y + 1][x + 1],
                            self.image[y + 1][x + 2],
                        ],
                        [
                            self.image[y + 2][x],
                            self.image[y + 2][x + 1],
                            self.image[y + 2][x + 2],
                        ],
                    ];

                    let Some(rule) = rules.iter().find(|rule| rule.matches(window.into())) else {
                        eyre::bail!("no matching rule")
                    };

                    let dy = y / 3 * 4;
                    let dx = x / 3 * 4;

                    next[dy][dx..][..4].copy_from_slice(&rule.to[0]);
                    next[dy + 1][dx..][..4].copy_from_slice(&rule.to[1]);
                    next[dy + 2][dx..][..4].copy_from_slice(&rule.to[2]);
                    next[dy + 3][dx..][..4].copy_from_slice(&rule.to[3]);
                }
            }

            self.size /= 3;
            self.size *= 4;
            self.image = next;

            Ok(())
        }
    }

    impl Default for Image {
        fn default() -> Self {
            let size = 3;

            let image = ".#.\n..#\n###"
                .lines()
                .map(|line| line.bytes().collect())
                .collect();

            Self { size, image }
        }
    }

    pub type Pixel = u8;
}
