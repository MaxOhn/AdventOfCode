use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> u32 {
    let mut lines = input.lines();

    let dirs = lines.next().unwrap().bytes().cycle();

    let _ = lines.next().unwrap();

    let mut nodes = Vec::new();

    for line in lines {
        let (front, back) = line.split_once(" = ").unwrap();

        let (left, right) = back
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(", ")
            .unwrap();

        nodes.push((front, left, right));
    }

    let mut curr = "AAA";
    let mut steps = 0;

    for dir in dirs {
        steps += 1;
        let (_, left, right) = nodes.iter().find(|(from, ..)| *from == curr).unwrap();

        match dir {
            b'L' => curr = left,
            b'R' => curr = right,
            _ => panic!(),
        }

        if curr == "ZZZ" {
            break;
        }
    }

    steps
}

fn part2(input: &str) -> u64 {
    let mut lines = input.lines();

    let dirs = lines.next().unwrap().bytes().cycle();

    let _ = lines.next().unwrap();

    let mut nodes = Vec::new();

    for line in lines {
        let (front, back) = line.split_once(" = ").unwrap();

        let (left, right) = back
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(", ")
            .unwrap();

        nodes.push((front, left, right));
    }

    nodes.sort_unstable_by_key(|(from, ..)| *from);

    let mut currs: Vec<_> = nodes
        .iter()
        .filter_map(|(from, ..)| from.ends_with('A').then_some(*from))
        .collect();

    let mut until_z = vec![None; currs.len()];

    let mut steps = 0;

    for dir in dirs {
        steps += 1;
        let mut done = true;

        for (curr, until_z) in currs.iter_mut().zip(until_z.iter_mut()) {
            let idx = nodes
                .binary_search_by_key(curr, |(from, ..)| *from)
                .unwrap();

            let (_, left, right) = &nodes[idx];

            let next = match dir {
                b'L' => left,
                b'R' => right,
                _ => panic!(),
            };

            *curr = next;

            done &= if curr.ends_with('Z') {
                let _ = until_z.get_or_insert(steps);

                true
            } else {
                until_z.is_some()
            }
        }

        if done {
            break;
        }
    }

    until_z
        .into_iter()
        .flatten()
        .fold(1, |lcm, n| aoc_rust::util::numbers::lcm(lcm, n))
}
