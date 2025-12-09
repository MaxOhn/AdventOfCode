use std::{cmp, collections::HashSet};

use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> u64 {
    let tiles = Tile::parse_all(input);

    let mut max = 0;

    for i in 0..tiles.len() - 1 {
        for j in i + 1..tiles.len() {
            let a = tiles[i];
            let b = tiles[j];

            let area = Tile::area(a, b);

            if area > max {
                max = area;
            }
        }
    }

    max
}

fn part2(input: &str) -> u64 {
    let tiles = Tile::parse_all(input);

    let mut max_x = 0;
    let mut max_y = 0;

    for tile in tiles.iter() {
        max_x = max_x.max(tile.x);
        max_y = max_y.max(tile.y);
    }

    let mut edges = HashSet::new();

    for tile in tiles.iter() {
        let mut found_x = false;
        let mut found_y = false;

        for i in 1.. {
            let nx1 = tile.x - i;
            let nx2 = tile.x + i;

            let ny1 = tile.y - i;
            let ny2 = tile.y + i;

            if nx1 >= 0 && !found_x {
                let nt = Tile::new(nx1, tile.y);

                if tiles.contains(&nt) {
                    edges.insert((*tile, nt));
                    found_x = true;
                }
            }

            if nx2 <= max_x && !found_x {
                let nt = Tile::new(nx2, tile.y);

                if tiles.contains(&nt) {
                    edges.insert((*tile, nt));
                    found_x = true;
                }
            }

            if ny1 >= 0 && !found_y {
                let nt = Tile::new(tile.x, ny1);

                if tiles.contains(&nt) {
                    edges.insert((*tile, nt));
                    found_y = true;
                }
            }

            if ny2 <= max_y && !found_y {
                let nt = Tile::new(tile.x, ny2);

                if tiles.contains(&nt) {
                    edges.insert((*tile, nt));
                    found_y = true;
                }
            }

            if (found_x && found_y) || (nx1 < 0 && nx2 > max_x && ny1 < 0 && ny2 > max_y) {
                break;
            }
        }
    }

    let mut max = 0;

    for i in 0..tiles.len() - 1 {
        for j in i + 1..tiles.len() {
            let a = tiles[i];
            let b = tiles[j];

            if a.x == b.x || a.y == b.y {
                continue;
            }

            let area = Tile::area(a, b);

            if area <= max {
                continue;
            }

            // Check if any edge intersects with the rectangle within the
            // rectangle built by two tiles. If so, discard it.
            let top_left = Tile::new(cmp::min(a.x, b.x) + 1, cmp::min(a.y, b.y) + 1);
            let top_right = Tile::new(cmp::max(a.x, b.x) - 1, cmp::min(a.y, b.y) + 1);
            let bot_left = Tile::new(cmp::min(a.x, b.x) + 1, cmp::max(a.y, b.y) - 1);
            let bot_right = Tile::new(cmp::max(a.x, b.x) - 1, cmp::max(a.y, b.y) - 1);

            let rect = [top_left, top_right, bot_left, bot_right];

            let found_intersecting_edge = edges.iter().any(|(ea, eb)| {
                fn is_within(tile: &Tile, rect: &[Tile; 4]) -> bool {
                    let [top_left, top_right, bot_left, _] = rect;

                    tile.x >= top_left.x
                        && tile.x <= top_right.x
                        && tile.y >= top_left.y
                        && tile.y <= bot_left.y
                }

                // "counter clock wise algorithm" to check whether lines intersect
                fn line_segments_intersect(p1: Tile, p2: Tile, p3: Tile, p4: Tile) -> bool {
                    fn ccw(a: Tile, b: Tile, c: Tile) -> bool {
                        (c.y - a.y) * (b.x - a.x) > (b.y - a.y) * (c.x - a.x)
                    }

                    ccw(p1, p3, p4) != ccw(p2, p3, p4) && ccw(p1, p2, p3) != ccw(p1, p2, p4)
                }

                if is_within(ea, &rect) || is_within(eb, &rect) {
                    return true;
                }

                let rect_edges = [
                    (top_left, top_right),
                    (top_right, bot_right),
                    (bot_right, bot_left),
                    (bot_left, top_left),
                ];

                for (ra, rb) in rect_edges {
                    if line_segments_intersect(ra, rb, *ea, *eb) {
                        return true;
                    }
                }

                false
            });

            if !found_intersecting_edge {
                max = area;
            }
        }
    }

    max
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Tile {
    x: i64,
    y: i64,
}

impl Tile {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn parse_all(input: &str) -> Vec<Self> {
        input
            .lines()
            .map(|line| {
                let (x, y) = line.split_once(',').unwrap();

                Tile {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                }
            })
            .collect()
    }

    fn area(a: Self, b: Self) -> u64 {
        (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1)
    }
}
