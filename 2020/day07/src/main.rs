use std::collections::HashMap;
use std::hint::unreachable_unchecked;
use std::io::{BufRead, BufReader};
use std::time::Instant;
use util::NumHasherBuilder;

type Bags = HashMap<u16, Vec<(u8, u16)>, NumHasherBuilder>;
type CachePart1 = HashMap<u16, bool>;
type CachePart2 = HashMap<u16, u32>;

const MY_BAG: &str = "iny go"; // always omit the first and last two characters

fn main() {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut names = HashMap::with_capacity(594);
    names.insert(MY_BAG.to_owned(), 0);
    let mut id = 1;

    let mut bags = HashMap::with_capacity_and_hasher(590, NumHasherBuilder);

    while input
        .read_line(&mut line)
        .unwrap_or_else(|_| unsafe { unreachable_unchecked() })
        != 0
    {
        let bytes = line.as_bytes();
        let i = bag_end_idx(bytes) - 2;

        if unsafe { *bytes.get_unchecked(i + 16) } == b'n' {
            line.clear();
            continue;
        }

        let outer = if let Some(bag) = names.get(unsafe { line.get_unchecked(2..i) }) {
            *bag
        } else {
            names.insert(unsafe { line.get_unchecked(2..i) }.to_owned(), id);
            id += 1;

            id - 1
        };

        for inner in unsafe { line.get_unchecked(i + 16..) }.split(", ") {
            let bytes = inner.as_bytes();
            let n = unsafe { *bytes.get_unchecked(0) } - b'0';
            let i = bag_end_idx(unsafe { bytes.get_unchecked(4..) }) + 2;

            let inner = if let Some(bag) = names.get(unsafe { inner.get_unchecked(4..i) }) {
                (n, *bag)
            } else {
                names.insert(unsafe { inner.get_unchecked(4..i).to_owned() }, id);
                id += 1;

                (n, id - 1)
            };

            bags.entry(outer)
                .or_insert_with(|| Vec::with_capacity(2))
                .push(inner);
        }

        line.clear();
    }

    println!("Setup: {:?}", start.elapsed()); // 850µs

    let start = Instant::now();
    let mut cache = HashMap::with_capacity(256);

    let p1 = bags
        .keys()
        .filter(|&&bag| contains_recursive(bag, &bags, &mut cache))
        .count()
        - 1; // my bag does not contain itself

    println!("Part 1: {} [{:?}]", p1, start.elapsed()); // 110µs

    let start = Instant::now();
    let mut cache = HashMap::with_capacity(64);

    let p2: u32 = bags
        .get(&0)
        .unwrap_or_else(|| unsafe { unreachable_unchecked() })
        .iter()
        .map(|(amount, bag)| *amount as u32 * count_recursive(*bag, &bags, &mut cache))
        .sum();

    println!("Part 2: {} [{:?}]", p2, start.elapsed()); // 4µs

    assert_eq!(p1, 161);
    assert_eq!(p2, 30_899);
}

fn contains_recursive(bag: u16, bags: &Bags, cache: &mut CachePart1) -> bool {
    let inner = if let Some(value) = cache.get(&bag) {
        return *value;
    } else if bag == 0 {
        return true;
    } else if let Some(bag) = bags.get(&bag) {
        bag
    } else {
        return false;
    };

    for (_, bag) in inner {
        if contains_recursive(*bag, bags, cache) {
            cache.insert(*bag, true);
            return true;
        }
    }

    cache.insert(bag, false);
    false
}

fn count_recursive(bag: u16, bags: &Bags, cache: &mut CachePart2) -> u32 {
    if let Some(value) = cache.get(&bag) {
        return *value;
    }

    let mut count = 1;

    if let Some(inner) = bags.get(&bag) {
        for (amount, bag) in inner {
            count += *amount as u32 * count_recursive(*bag, bags, cache);
        }
    }

    cache.insert(bag, count);
    count
}

fn bag_end_idx(bag: &[u8]) -> usize {
    let mut i = 1;
    let mut found_first = false;

    loop {
        if unsafe { *bag.get_unchecked(i) } == b' ' {
            if found_first {
                return i;
            }

            found_first = true;
        }

        i += 1;
    }
}
