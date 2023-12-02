use std::{slice::Iter, str::FromStr};

use aoc_rust::Solution;
use eyre::{ContextCompat, Report, Result, WrapErr};

pub fn run(input: &str) -> Result<Solution> {
    let games = input
        .trim()
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<Game>>>()?;

    let p1 = part1(&games);
    let p2 = part2(&games);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(games: &[Game]) -> u32 {
    games
        .iter()
        .filter_map(|game| game.is_possible().then_some(game.id))
        .sum()
}

fn part2(games: &[Game]) -> u32 {
    games.iter().map(Game::powersum).sum()
}

struct Game {
    id: u32,
    subsets: Subsets,
}

impl Game {
    fn is_possible(&self) -> bool {
        self.subsets
            .iter()
            .flatten()
            .all(|cubes| match cubes.color {
                Color::Red => cubes.count <= 12,
                Color::Green => cubes.count <= 13,
                Color::Blue => cubes.count <= 14,
            })
    }

    fn powersum(&self) -> u32 {
        let (red, green, blue) =
            self.subsets
                .iter()
                .flatten()
                .fold((0, 0, 0), |(mut r, mut g, mut b), cubes| {
                    match cubes.color {
                        Color::Red => r = r.max(cubes.count),
                        Color::Green => g = g.max(cubes.count),
                        Color::Blue => b = b.max(cubes.count),
                    }

                    (r, g, b)
                });

        red * green * blue
    }
}

enum Color {
    Red,
    Green,
    Blue,
}

impl FromStr for Color {
    type Err = Report;

    fn from_str(color: &str) -> Result<Self, Self::Err> {
        let color = match color {
            "red" => Self::Red,
            "green" => Self::Green,
            "blue" => Self::Blue,
            _ => bail!("invalid color `{color}`"),
        };

        Ok(color)
    }
}

struct Cubes {
    count: u32,
    color: Color,
}

impl FromStr for Cubes {
    type Err = Report;

    fn from_str(cubes: &str) -> Result<Self, Self::Err> {
        let (count, color) = cubes.split_once(' ').wrap_err("missing whitespace")?;

        let count = count
            .parse()
            .map_err(|_| eyre!("incorrect count `{count}`"))?;

        let color = color.parse().wrap_err("incorrect color")?;

        Ok(Self { count, color })
    }
}

struct Reveal {
    cubes: Vec<Cubes>,
}

impl FromStr for Reveal {
    type Err = Report;

    fn from_str(reveal: &str) -> Result<Self, Self::Err> {
        reveal
            .split(", ")
            .map(str::parse)
            .collect::<Result<_>>()
            .wrap_err("invalid cubes")
            .map(|cubes| Self { cubes })
    }
}

impl<'a> IntoIterator for &'a Reveal {
    type Item = &'a Cubes;
    type IntoIter = Iter<'a, Cubes>;

    fn into_iter(self) -> Self::IntoIter {
        self.cubes.iter()
    }
}

struct Subsets {
    reveals: Vec<Reveal>,
}

impl Subsets {
    fn iter(&self) -> Iter<'_, Reveal> {
        self.reveals.iter()
    }
}

impl FromStr for Subsets {
    type Err = Report;

    fn from_str(subsets: &str) -> Result<Self, Self::Err> {
        subsets
            .split("; ")
            .map(str::parse)
            .collect::<Result<_>>()
            .wrap_err("invalid reveal")
            .map(|reveals| Self { reveals })
    }
}

impl FromStr for Game {
    type Err = Report;

    fn from_str(game: &str) -> Result<Self, Self::Err> {
        let suffix = game.strip_prefix("Game ").wrap_err("missing prefix")?;
        let (id, suffix) = suffix.split_once(": ").wrap_err("missing colon")?;
        let id = id.parse().map_err(|_| eyre!("incorrect id `{id}`"))?;
        let subsets = suffix.parse().wrap_err("invalid subsets")?;

        Ok(Self { id, subsets })
    }
}
