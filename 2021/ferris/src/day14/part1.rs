use memchr::{memchr, memchr_iter};
use rustc_hash::FxHashMap as HashMap;

pub fn run(input: &[u8]) -> i64 {
    array_pairs_counts::run(input)
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

pub mod array_pairs {
    use memchr::{memchr, memchr_iter};
    use rustc_hash::FxHashMap as HashMap;

    struct Pairs([u8; 26 * 26]);

    impl Pairs {
        fn new() -> Self {
            Self([0; 26 * 26])
        }

        #[inline(always)]
        fn insert(&mut self, a: u8, b: u8, c: u8) {
            let a = (a - b'A') as usize;
            let b = (b - b'A') as usize;

            self.0[a * 26 + b] = c - b'A';
        }

        #[inline(always)]
        fn get(&self, a: u8, b: u8) -> u8 {
            self.0[a as usize * 26 + b as usize]
        }
    }

    pub fn run(input: &[u8]) -> i64 {
        let new_line = memchr(b'\n', input).unwrap();

        let mut pairs = Pairs::new();

        memchr_iter(b'-', input)
            .for_each(|i| pairs.insert(input[i - 3], input[i - 2], input[i + 3]));

        let template = &input[..new_line];
        let mut counts = Counts::default();
        let mut cache = Cache::default();

        for (a, b) in template.iter().zip(template.iter().skip(1)) {
            *counts.entry(*a - b'A').or_default() += 1;
            recurse(*a - b'A', *b - b'A', 10, &pairs, &mut counts, &mut cache);
        }

        *counts.entry(input[new_line - 1] - b'A').or_default() += 1;

        let (min, max) = counts
            .into_iter()
            .map(|(_, v)| v)
            .fold((i64::MAX, 0), |(min, max), count| {
                (min.min(count), max.max(count))
            });

        max - min
    }

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
                if depth > 0 {
                    let c = pairs.get(a, b);
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
}

pub mod array_pairs_counts {
    use memchr::{memchr, memchr_iter};
    use rustc_hash::FxHashMap as HashMap;

    struct Pairs([u8; 26 * 26]);

    impl Pairs {
        #[inline(always)]
        fn new() -> Self {
            Self([0; 26 * 26])
        }

        #[inline(always)]
        fn insert(&mut self, a: u8, b: u8, c: u8) {
            let a = (a - b'A') as usize;
            let b = (b - b'A') as usize;

            self.0[a * 26 + b] = c - b'A';
        }

        #[inline(always)]
        fn get(&self, a: u8, b: u8) -> u8 {
            self.0[a as usize * 26 + b as usize]
        }
    }

    pub fn run(input: &[u8]) -> i64 {
        let new_line = memchr(b'\n', input).unwrap();

        let mut pairs = Pairs::new();

        memchr_iter(b'-', input)
            .for_each(|i| pairs.insert(input[i - 3], input[i - 2], input[i + 3]));

        let template = &input[..new_line];
        let mut counts = [0; 26];
        let mut cache = Cache::default();

        for (a, b) in template.iter().zip(template.iter().skip(1)) {
            counts[(*a - b'A') as usize] += 1;
            recurse(*a - b'A', *b - b'A', 10, &pairs, &mut counts, &mut cache);
        }

        counts[(input[new_line - 1] - b'A') as usize] += 1;

        let (min, max) = counts.into_iter().fold((i64::MAX, 0), |(min, max), count| {
            (if count > 0 { min.min(count) } else { min }, max.max(count))
        });

        max - min
    }

    type Counts = [i64; 26];
    type Cache = HashMap<(u8, u8, u8), Counts>;

    fn recurse(a: u8, b: u8, depth: u8, pairs: &Pairs, total: &mut Counts, cache: &mut Cache) {
        match cache.get(&(a, b, depth)) {
            Some(counts) => {
                for (total, curr) in total.iter_mut().zip(counts) {
                    *total += *curr;
                }
            }
            None => {
                if depth > 0 {
                    let c = pairs.get(a, b);
                    let mut counts = [0; 26];
                    counts[c as usize] += 1;

                    recurse(a, c, depth - 1, pairs, &mut counts, cache);
                    recurse(c, b, depth - 1, pairs, &mut counts, cache);

                    let counts = cache.entry((a, b, depth)).or_insert(counts);

                    for (total, curr) in total.iter_mut().zip(counts) {
                        *total += *curr;
                    }
                }
            }
        }
    }
}
