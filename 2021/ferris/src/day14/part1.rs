use memchr::{memchr, memchr_iter};
use rustc_hash::FxHashMap as HashMap;

pub fn run(input: &[u8]) -> i64 {
    cached(input)
}

pub fn simple(input: &[u8]) -> i64 {
    let new_line = memchr(b'\n', input).unwrap();

    let pairs: Pairs = memchr_iter(b'-', input)
        .map(|i| ((input[i - 3], input[i - 2]), input[i + 3]))
        .collect();

    let template = &input[..new_line];
    let mut counts = Counts::default();

    for (a, b) in template.iter().zip(template.iter().skip(1)) {
        *counts.entry(*a).or_default() += 1;
        recurse(*a, *b, 10, &pairs, &mut counts);
    }

    *counts.entry(input[new_line - 1]).or_default() += 1;

    let (min, max) = counts
        .into_iter()
        .map(|(_, v)| v)
        .fold((i64::MAX, 0), |(min, max), count| {
            (min.min(count), max.max(count))
        });

    max - min
}

pub fn cached(input: &[u8]) -> i64 {
    let new_line = memchr(b'\n', input).unwrap();

    let pairs: Pairs = memchr_iter(b'-', input)
        .map(|i| ((input[i - 3], input[i - 2]), input[i + 3]))
        .collect();

    let template = &input[..new_line];
    let mut counts = Counts::default();
    let mut cache = Cache::default();

    for (a, b) in template.iter().zip(template.iter().skip(1)) {
        *counts.entry(*a).or_default() += 1;
        recurse_cached(*a, *b, 10, &pairs, &mut counts, &mut cache);
    }

    *counts.entry(input[new_line - 1]).or_default() += 1;

    let (min, max) = counts
        .into_iter()
        .map(|(_, v)| v)
        .fold((i64::MAX, 0), |(min, max), count| {
            (min.min(count), max.max(count))
        });

    max - min
}

type Pairs = HashMap<(u8, u8), u8>;
type Counts = HashMap<u8, i64>;
type Cache = HashMap<(u8, u8, u8), Counts>;

fn recurse(a: u8, b: u8, depth: u8, pairs: &Pairs, counts: &mut Counts) {
    if let Some(c) = pairs.get(&(a, b)).filter(|_| depth > 0).copied() {
        *counts.entry(c).or_default() += 1;

        recurse(a, c, depth - 1, pairs, counts);
        recurse(c, b, depth - 1, pairs, counts);
    }
}

fn recurse_cached(a: u8, b: u8, depth: u8, pairs: &Pairs, total: &mut Counts, cache: &mut Cache) {
    match cache.get(&(a, b, depth)) {
        Some(counts) => {
            for (k, v) in counts {
                *total.entry(*k).or_default() += *v;
            }
        }
        None => {
            if let Some(c) = pairs.get(&(a, b)).filter(|_| depth > 0).copied() {
                let mut counts = Counts::default();
                *counts.entry(c).or_default() += 1;

                recurse_cached(a, c, depth - 1, pairs, &mut counts, cache);
                recurse_cached(c, b, depth - 1, pairs, &mut counts, cache);

                let counts = cache.entry((a, b, depth)).or_insert(counts);

                for (k, v) in counts {
                    *total.entry(*k).or_default() += *v;
                }
            }
        }
    }
}
