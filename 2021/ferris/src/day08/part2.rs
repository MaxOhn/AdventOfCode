use std::mem;

use bits::Bits;
use memchr::memchr;

pub fn run(input: &[u8]) -> i64 {
    short(input)
}

pub fn short(input: &[u8]) -> i64 {
    let mut sum = 0;

    for line in Lines::new(input) {
        let split_idx = memchr(b'|', line).unwrap();
        let mut left_iter = SplitWhiteSpace::new(&line[..split_idx]);

        let mut words = [
            left_iter.next().unwrap(),
            left_iter.next().unwrap(),
            left_iter.next().unwrap(),
            left_iter.next().unwrap(),
            left_iter.next().unwrap(),
            left_iter.next().unwrap(),
            left_iter.next().unwrap(),
            left_iter.next().unwrap(),
            left_iter.next().unwrap(),
            left_iter.next().unwrap(),
        ];

        words.sort_unstable_by_key(|options| options.count_ones());
        let [one, _, four, ..] = words;

        let mut i = split_idx + 2;
        let mut n = 0;

        for _ in 0..3 {
            let mut output = Bits::default();

            while line[i] != b' ' {
                output.add(line[i]);
                i += 1;
            }

            i += 1;
            n = n * 10 + digit_(output, one, four);
        }

        let mut output = Bits::default();

        while i < line.len() && line[i] != b'\n' {
            output.add(line[i]);
            i += 1;
        }

        sum += n * 10 + digit_(output, one, four);
    }

    sum
}

fn digit_(bits: Bits, one: Bits, four: Bits) -> i64 {
    match (
        bits.count_ones(),
        (one & bits).count_ones(),
        (four & bits).count_ones(),
    ) {
        (2, _, _) => 1,
        (3, _, _) => 7,
        (4, _, _) => 4,
        (5, 2, _) => 3,
        (5, _, 2) => 2,
        (5, _, _) => 5,
        (6, 1, _) => 6,
        (6, _, 4) => 9,
        (6, _, _) => 0,
        _ => 8,
    }
}

pub fn long(input: &[u8]) -> i64 {
    let mut sum = 0;

    for line in Lines::new(input) {
        let split_idx = memchr(b'|', line).unwrap();
        let mut left_iter = SplitWhiteSpace::new(&line[..split_idx]);

        let mut words = [
            left_iter.next().unwrap(),
            left_iter.next().unwrap(),
            left_iter.next().unwrap(),
            left_iter.next().unwrap(),
            left_iter.next().unwrap(),
            left_iter.next().unwrap(),
            left_iter.next().unwrap(),
            left_iter.next().unwrap(),
            left_iter.next().unwrap(),
            left_iter.next().unwrap(),
        ];

        words.sort_unstable_by_key(|options| options.count_ones());
        let [one, seven, four, _, _, _, six_1, six_2, six_3, eight] = words;

        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g] = [
            Bits::default(),
            Bits::default(),
            Bits::default(),
            Bits::default(),
            Bits::default(),
            Bits::default(),
            Bits::default(),
        ];

        e |= eight;
        g |= eight;

        b |= four;
        d |= four;
        e &= !four;
        g &= !four;

        a |= seven;
        b &= !seven;
        d &= !seven;
        e &= !seven;
        g &= !seven;

        c |= one;
        f |= one;
        a &= !one;

        for six in [six_1, six_2, six_3] {
            let mut word = six;

            word &= !a;
            word &= !e;
            word &= !f;
            word &= !g;

            if word.count_ones() == 1 {
                word &= !c;
                b &= word;
            } else {
                let mut word = six;
                word &= !a;
                word &= !b;
                word &= !c;
                word &= !d;
                word &= !f;

                if word.count_ones() == 1 {
                    e &= !word;
                } else {
                    c &= !six;
                }
            }
        }

        let b = 1 << b.letter_idx();
        let c = 1 << c.letter_idx();
        let e = 1 << e.letter_idx();

        let mut i = split_idx + 2;
        let mut n = 0;

        for _ in 0..3 {
            let mut output = Bits::default();

            while line[i] != b' ' {
                output.add(line[i]);
                i += 1;
            }

            i += 1;

            n = n * 10 + digit(output, b, c, e);
        }

        let mut output = Bits::default();

        while i < line.len() && line[i] != b'\n' {
            output.add(line[i]);
            i += 1;
        }

        n = n * 10 + digit(output, b, c, e);
        sum += n;
    }

    sum
}

fn digit(bits: Bits, b: u8, c: u8, e: u8) -> i64 {
    match bits.count_ones() {
        2 => 1,
        3 => 7,
        4 => 4,
        7 => 8,
        5 => match bits & (b | e) {
            0 => 3,
            bit if bit == e => 2,
            _ => 5,
        },
        6 => match bits & (c | e) {
            bit if bit == c => 9,
            bit if bit == e => 6,
            _ => 0,
        },
        _ => unreachable!(),
    }
}

mod bits {
    use std::{
        fmt,
        ops::{BitAnd, BitAndAssign, BitOrAssign, Not},
    };

    #[derive(Copy, Clone, Default)]
    pub struct Bits(u8);

    impl Bits {
        pub fn from_bytes(bytes: &[u8]) -> Self {
            let mut bits = 0;

            for &byte in bytes {
                bits |= 1 << (byte - b'a');
            }

            Self(bits)
        }

        pub fn count_ones(self) -> u32 {
            self.0.count_ones()
        }

        pub fn letter_idx(self) -> u8 {
            let mut shift = 0;

            while self.0 & (1 << shift) == 0 {
                shift += 1;
            }

            shift
        }

        pub fn add(&mut self, byte: u8) {
            self.0 |= 1 << (byte - b'a');
        }
    }

    impl Not for Bits {
        type Output = Self;

        #[inline]
        fn not(self) -> Self::Output {
            Self(!self.0)
        }
    }

    impl BitAnd<Self> for Bits {
        type Output = u8;

        #[inline]
        fn bitand(self, rhs: Self) -> Self::Output {
            self.0 & rhs.0
        }
    }

    impl BitAnd<u8> for Bits {
        type Output = u8;

        #[inline]
        fn bitand(self, rhs: u8) -> Self::Output {
            self.0 & rhs
        }
    }

    impl BitAndAssign<Self> for Bits {
        #[inline]
        fn bitand_assign(&mut self, rhs: Self) {
            self.0 &= rhs.0;
        }
    }

    impl BitOrAssign<Self> for Bits {
        #[inline]
        fn bitor_assign(&mut self, rhs: Self) {
            self.0 |= rhs.0;
        }
    }

    impl BitOrAssign<u8> for Bits {
        #[inline]
        fn bitor_assign(&mut self, rhs: u8) {
            self.0 |= rhs;
        }
    }

    impl fmt::Display for Bits {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Debug::fmt(self, f)
        }
    }

    impl fmt::Debug for Bits {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:0>8b}", self.0)
        }
    }
}

struct SplitWhiteSpace<'b> {
    bytes: &'b [u8],
}

impl<'b> SplitWhiteSpace<'b> {
    fn new(bytes: &'b [u8]) -> Self {
        Self { bytes }
    }
}

impl Iterator for SplitWhiteSpace<'_> {
    type Item = Bits;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let line = memchr(b' ', self.bytes).unwrap();
        let options = Bits::from_bytes(&self.bytes[..line]);
        self.bytes = &self.bytes[line + 1..];

        Some(options)
    }
}

struct Lines<'b> {
    bytes: &'b [u8],
}

impl<'b> Lines<'b> {
    fn new(bytes: &'b [u8]) -> Self {
        Self { bytes }
    }
}

impl<'b> Iterator for Lines<'b> {
    type Item = &'b [u8];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match memchr(b'\n', self.bytes) {
            Some(idx) => {
                let ret = &self.bytes[..idx];
                self.bytes = &self.bytes[idx + 1..];

                Some(ret)
            }
            None => (!self.bytes.is_empty()).then(|| mem::take(&mut self.bytes)),
        }
    }
}
