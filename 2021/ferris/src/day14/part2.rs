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
        counts[(window[0] - b'A') as usize] += 1;
        recurse(
            window[0] - b'A',
            window[1] - b'A',
            40,
            &pairs,
            &mut counts,
            &mut cache,
        );
    }

    counts[(*template.last().unwrap() - b'A') as usize] += 1;

    eval(counts)
}

pub fn parallel(input: &[u8]) -> i64 {
    let (template, pairs) = parse_input(input);

    let map_op = |cache: &mut Cache, window: &[u8]| {
        let mut counts = Counts::default();
        counts[(window[0] - b'A') as usize] += 1;
        recurse(
            window[0] - b'A',
            window[1] - b'A',
            40,
            &pairs,
            &mut counts,
            cache,
        );

        counts
    };

    let reduce_op = |mut counts: Counts, curr: Counts| {
        counts
            .iter_mut()
            .zip(curr)
            .for_each(|(count, n)| *count += n);

        counts
    };

    let mut counts = template
        .par_windows(2)
        .map_init(Cache::default, map_op)
        .reduce(Counts::default, reduce_op);

    counts[(*template.last().unwrap() - b'A') as usize] += 1;

    eval(counts)
}

#[inline(always)]
fn parse_input(input: &[u8]) -> (&[u8], Pairs) {
    let new_line = memchr(b'\n', input).unwrap();

    let mut pairs = Pairs::new();
    memchr_iter(b'-', input).for_each(|i| pairs.insert(input[i - 3], input[i - 2], input[i + 3]));

    (&input[..new_line], pairs)
}

#[inline(always)]
fn eval(counts: Counts) -> i64 {
    let (min, max) = counts.into_iter().fold((i64::MAX, 0), |(min, max), count| {
        (if count > 0 { min.min(count) } else { min }, max.max(count))
    });

    max - min
}

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
                let mut counts = Counts::default();
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
