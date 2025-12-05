use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let input = input.trim();

    let p1 = part1(input);
    let p2 = part2(input);

    Ok(Solution::new().part1(p1).part2(p2))
}

fn part1(input: &str) -> usize {
    let mut lines = input.lines();

    let mut events = parse_ranges(lines.by_ref());

    let ids = lines.map(str::parse).flat_map(Result::ok).map(|id| Event {
        value: id,
        kind: EventKind::Query,
    });

    events.extend(ids);
    events.sort_unstable();

    events
        .iter()
        .scan(0, |active, event| {
            match event.kind {
                EventKind::Query => return Some(*active > 0),
                EventKind::Start => *active += 1,
                EventKind::End => *active -= 1,
            }

            Some(false)
        })
        .filter(|&b| b)
        .count()
}

fn part2(input: &str) -> u64 {
    let mut events = parse_ranges(input.lines());
    events.sort_unstable();

    let mut total = 0;
    let mut active = 0;
    let mut prev = 0;

    for event in events {
        if active > 0 {
            total += event.value - prev;
        }

        prev = event.value;

        match event.kind {
            EventKind::Start => active += 1,
            EventKind::End => active -= 1,
            EventKind::Query => unreachable!("only used in part 1"),
        }
    }

    total
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum EventKind {
    Start,
    Query,
    End,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Event {
    value: u64,
    kind: EventKind,
}

fn parse_ranges<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<Event> {
    let ranges = lines
        .take_while(|line| !line.is_empty())
        .flat_map(|line| line.split_once('-'))
        .flat_map(|(from, to)| from.parse::<u64>().ok().zip(to.parse::<u64>().ok()));

    ranges
        .flat_map(|(from, to)| {
            [
                Event {
                    value: from,
                    kind: EventKind::Start,
                },
                Event {
                    value: to + 1,
                    kind: EventKind::End,
                },
            ]
        })
        .collect()
}
