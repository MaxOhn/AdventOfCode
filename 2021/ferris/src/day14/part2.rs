use memchr::{memchr, memchr_iter};
use rayon::{iter::ParallelIterator, slice::ParallelSlice};
use rustc_hash::FxHashMap as HashMap;

pub fn run(input: &[u8]) -> i64 {
    sync(input)
}

pub fn sync(input: &[u8]) -> i64 {
    let (template, pairs) = parse_input(input);

    let mut counts = Counts::default();
    let mut cache = Cache::default();

    for window in template.windows(2) {
        *counts.entry(window[0]).or_default() += 1;
        recurse(window[0], window[1], 40, &pairs, &mut counts, &mut cache);
    }

    *counts.entry(*template.last().unwrap()).or_default() += 1;

    eval(counts)
}

pub fn parallel(input: &[u8]) -> i64 {
    let (template, pairs) = parse_input(input);

    let map_op = |cache: &mut Cache, window: &[u8]| {
        let mut counts = Counts::default();
        *counts.entry(window[0]).or_default() += 1;
        recurse(window[0], window[1], 40, &pairs, &mut counts, cache);

        counts
    };

    let reduce_op = |mut counts: Counts, curr: Counts| {
        curr.into_iter()
            .for_each(|(c, n)| *counts.entry(c).or_default() += n);

        counts
    };

    let mut counts = template
        .par_windows(2)
        .map_init(Cache::default, map_op)
        .reduce(Counts::default, reduce_op);

    *counts.entry(*template.last().unwrap()).or_default() += 1;

    eval(counts)
}

#[inline(always)]
fn parse_input(input: &[u8]) -> (&[u8], Pairs) {
    let new_line = memchr(b'\n', input).unwrap();

    let pairs: Pairs = memchr_iter(b'-', input)
        .map(|i| ((input[i - 3], input[i - 2]), input[i + 3]))
        .collect();

    (&input[..new_line], pairs)
}

#[inline(always)]
fn eval(counts: Counts) -> i64 {
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

fn recurse(a: u8, b: u8, depth: u8, pairs: &Pairs, total: &mut Counts, cache: &mut Cache) {
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

                recurse(a, c, depth - 1, pairs, &mut counts, cache);
                recurse(c, b, depth - 1, pairs, &mut counts, cache);

                let counts = cache.entry((a, b, depth)).or_insert(counts);

                for (k, v) in counts {
                    *total.entry(*k).or_default() += *v;
                }
            }
        }
    }
}
