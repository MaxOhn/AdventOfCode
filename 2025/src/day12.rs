use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> usize {
    struct Region {
        area: usize,
        quantities: Vec<usize>,
    }

    let mut lines = input.lines();

    let regions: Vec<_> = lines
        .by_ref()
        .rev()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (dim, quantities) = line.split_once(": ").unwrap();
            let (w, l) = dim.split_once('x').unwrap();

            let w: usize = w.parse().unwrap();
            let l: usize = l.parse().unwrap();

            let quantities = quantities
                .split(' ')
                .map(str::parse)
                .map(Result::unwrap)
                .collect();

            Region {
                area: w * l,
                quantities,
            }
        })
        .collect();

    let mut presents_size = Vec::new();

    while lines.next().is_some() {
        let mut size = 0;

        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }

            size += line.bytes().filter(|b| *b == b'#').count();
        }

        presents_size.push(size);
    }

    regions
        .iter()
        .filter(|region| {
            let needed: usize = region
                .quantities
                .iter()
                .enumerate()
                .map(|(i, count)| presents_size[i] * count)
                .sum();

            needed < region.area
        })
        .count()
}

fn part2(_input: &str) -> String {
    "x".to_owned()
}
