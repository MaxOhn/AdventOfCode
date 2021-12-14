use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

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

    input.read_line(&mut line)?;
    let template = line.as_bytes().to_owned(); // contains \n but doesn't matter
    input.read_line(&mut line)?;
    line.clear();

    let mut pairs = Pairs::new();

    while input.read_line(&mut line)? != 0 {
        let (left, right) = line.trim_end().split_once(" -> ").unwrap();
        let left = left.as_bytes();

        pairs.insert((left[0], left[1]), right.as_bytes()[0]);
        line.clear();
    }

    println!("Setup: {:?}", start.elapsed()); // 410µs

    let start = Instant::now();
    let p1 = solve(&template, &pairs, 10);
    println!("Part 1: {} [{:?}]", p1, start.elapsed()); // 420µs

    let start = Instant::now();
    let p2 = solve(&template, &pairs, 40);
    println!("Part 2: {} [{:?}]", p2, start.elapsed()); // 1.9ms

    assert_eq!(p1, 2549);
    assert_eq!(p2, 2_516_901_104_210);

    Ok(())
}

fn solve(template: &[u8], pairs: &Pairs, depth: u8) -> usize {
    let mut counts = Counts::new();
    let mut cache = Cache::new();

    for (a, b) in template.iter().zip(template.iter().skip(1)) {
        *counts.entry(*a).or_default() += 1;
        recurse(*a, *b, depth, &pairs, &mut counts, &mut cache);
    }

    let (min, max) = counts
        .into_iter()
        .map(|(_, v)| v)
        .fold((usize::MAX, 0), |(min, max), count| {
            (min.min(count), max.max(count))
        });

    max - min
}

type Pairs = HashMap<(u8, u8), u8>;
type Counts = BTreeMap<u8, usize>; // faster than HashMap
type Cache = HashMap<(u8, u8, u8), Counts>;

fn recurse(a: u8, b: u8, depth: u8, pairs: &Pairs, total: &mut Counts, cache: &mut Cache) {
    let counts = match cache.get(&(a, b, depth)) {
        Some(counts) => counts,
        None => {
            let mut counts = Counts::new();

            if let Some(c) = pairs.get(&(a, b)).filter(|_| depth > 0).copied() {
                *counts.entry(c).or_default() += 1;

                recurse(a, c, depth - 1, pairs, &mut counts, cache);
                recurse(c, b, depth - 1, pairs, &mut counts, cache);
            }

            cache.entry((a, b, depth)).or_insert(counts)
        }
    };

    for (k, v) in counts {
        *total.entry(*k).or_default() += *v;
    }
}
