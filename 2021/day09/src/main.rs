use std::{
    cmp::Reverse,
    collections::{HashSet, VecDeque},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

use util::{Matrix, Pos2};

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        let mut e: &dyn Error = &*err;

        while let Some(src) = e.source() {
            eprintln!("  - caused by: {}", src);
            e = src;
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let file = File::open("./input")?;
    let mut input = BufReader::new(file);

    let mut line = String::new();
    let mut grid = Vec::new();
    let mut w = 0;

    while input.read_line(&mut line)? != 0 {
        {
            let line = line.trim_end().as_bytes();
            w = line.len();
            grid.extend(line.iter().map(|&b| b - b'0'));
        }

        line.clear();
    }

    let grid = Matrix::from_vec(grid, w);
    println!("Setup: {:?}", start.elapsed()); // 145µs

    let start = Instant::now();
    let (p1, _) = part1(&grid);
    println!("Part 1: {} [{:?}]", p1, start.elapsed()); // 130µs

    let start = Instant::now();
    let (_, lows) = part1(&grid);
    let p2 = part2(&grid, &lows);
    println!("Part 2: {} [{:?}]", p2, start.elapsed()); // 2.1ms

    assert_eq!(p1, 575);
    assert_eq!(p2, 1_019_700);

    Ok(())
}

fn part1(grid: &Matrix<u8>) -> (u32, Vec<Pos2<isize>>) {
    let mut answer = 0;
    let mut lows = Vec::new();

    let w = grid.width() as isize;
    let h = grid.height() as isize;

    for x in 0..w {
        'outer: for y in 0..h {
            let curr = grid[y as usize][x as usize];

            for i in [-1, 0, 1] {
                for j in [-1, 0, 1] {
                    if i == 0 && j == 0 {
                        continue;
                    }

                    let nx = x + i;
                    let ny = y + j;

                    if nx < 0 || ny < 0 || nx >= w || ny >= h {
                        continue;
                    }

                    if grid[ny as usize][nx as usize] <= curr {
                        continue 'outer;
                    }
                }
            }

            answer += curr as u32 + 1;
            lows.push(Pos2::new(x, y));
        }
    }

    (answer, lows)
}

fn part2(grid: &Matrix<u8>, lows: &[Pos2<isize>]) -> usize {
    let mut basins = Vec::with_capacity(lows.len());

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    let w = grid.width() as isize;
    let h = grid.height() as isize;

    for &low in lows {
        let mut basin = HashSet::new();

        visited.clear();
        queue.clear();
        queue.push_front(low);

        while let Some(curr) = queue.pop_back() {
            if grid[curr.y as usize][curr.x as usize] == 9 {
                continue;
            }

            basin.insert(curr);

            for i in [-1, 0, 1] {
                for j in [-1, 0, 1] {
                    if i == j || i == -j {
                        continue;
                    }

                    let n = curr + Pos2::new(i, j);

                    if n.x < 0 || n.y < 0 || n.x >= w || n.y >= h {
                        continue;
                    }

                    if visited.insert(n) {
                        queue.push_front(n);
                    }
                }
            }
        }

        basins.push(basin.len());
    }

    basins.sort_unstable_by_key(|&len| Reverse(len));
    basins.truncate(3);

    basins.iter().fold(1, |prod, len| prod * len)
}
