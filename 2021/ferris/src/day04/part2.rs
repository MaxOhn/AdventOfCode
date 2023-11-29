#![feature(core_intrinsics, portable_simd)]

use std::intrinsics::unlikely;

use std::simd::{u8x64, Simd};

pub fn run(input: &[u8]) -> i64 {
    let new_line = memchr::memchr(b'\n', input).unwrap();

    let mut bingos: Vec<_> = input[new_line..]
        .chunks_exact(76)
        .map(parse_board)
        .map(|field| Bingo { field, marked: 0 })
        .collect();

    let mut numbers = NumberIter::new(&input[..new_line]);

    for n in &mut numbers {
        let mut finished = false;

        for bingo in bingos.iter_mut() {
            finished |= bingo.mark(n);
        }

        if finished {
            bingos.retain(|bingo| !bingo.is_done());

            if unlikely(bingos.len() == 1) {
                break;
            }
        }
    }

    let bingo = &mut bingos[0];

    for n in numbers {
        if bingo.mark(n) {
            return bingo.sum() * n as i64;
        }
    }

    unreachable!()
}

fn parse_board(slice: &[u8]) -> [u8; 25] {
    let row = unsafe {
        [
            *slice.get_unchecked(2),
            *slice.get_unchecked(3),
            *slice.get_unchecked(5),
            *slice.get_unchecked(6),
            *slice.get_unchecked(8),
            *slice.get_unchecked(9),
            *slice.get_unchecked(11),
            *slice.get_unchecked(12),
            *slice.get_unchecked(14),
            *slice.get_unchecked(15),
            *slice.get_unchecked(17),
            *slice.get_unchecked(18),
            *slice.get_unchecked(20),
            *slice.get_unchecked(21),
            *slice.get_unchecked(23),
            *slice.get_unchecked(24),
            *slice.get_unchecked(26),
            *slice.get_unchecked(27),
            *slice.get_unchecked(29),
            *slice.get_unchecked(30),
            *slice.get_unchecked(32),
            *slice.get_unchecked(33),
            *slice.get_unchecked(35),
            *slice.get_unchecked(36),
            *slice.get_unchecked(38),
            *slice.get_unchecked(39),
            *slice.get_unchecked(41),
            *slice.get_unchecked(42),
            *slice.get_unchecked(44),
            *slice.get_unchecked(45),
            *slice.get_unchecked(47),
            *slice.get_unchecked(48),
            *slice.get_unchecked(50),
            *slice.get_unchecked(51),
            *slice.get_unchecked(53),
            *slice.get_unchecked(54),
            *slice.get_unchecked(56),
            *slice.get_unchecked(57),
            *slice.get_unchecked(59),
            *slice.get_unchecked(60),
            *slice.get_unchecked(62),
            *slice.get_unchecked(63),
            *slice.get_unchecked(65),
            *slice.get_unchecked(66),
            *slice.get_unchecked(68),
            *slice.get_unchecked(69),
            *slice.get_unchecked(71),
            *slice.get_unchecked(72),
            *slice.get_unchecked(74),
            *slice.get_unchecked(75),
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ]
    };

    let simd = u8x64::from_array(row) - Simd::splat(b'0');

    let [a1, a2, b1, b2, c1, c2, d1, d2, e1, e2, f1, f2, g1, g2, h1, h2, i1, i2, j1, j2, k1, k2, l1, l2, m1, m2, n1, n2, o1, o2, p1, p2, q1, q2, r1, r2, s1, s2, t1, t2, u1, u2, v1, v2, w1, w2, x1, x2, y1, y2, ..] =
        simd.to_array();

    // * Note: b' '.wrapping_sub(b'0') = 240
    [
        (a1 != 240) as u8 * a1 * 10 + a2,
        (b1 != 240) as u8 * b1 * 10 + b2,
        (c1 != 240) as u8 * c1 * 10 + c2,
        (d1 != 240) as u8 * d1 * 10 + d2,
        (e1 != 240) as u8 * e1 * 10 + e2,
        (f1 != 240) as u8 * f1 * 10 + f2,
        (g1 != 240) as u8 * g1 * 10 + g2,
        (h1 != 240) as u8 * h1 * 10 + h2,
        (i1 != 240) as u8 * i1 * 10 + i2,
        (j1 != 240) as u8 * j1 * 10 + j2,
        (k1 != 240) as u8 * k1 * 10 + k2,
        (l1 != 240) as u8 * l1 * 10 + l2,
        (m1 != 240) as u8 * m1 * 10 + m2,
        (n1 != 240) as u8 * n1 * 10 + n2,
        (o1 != 240) as u8 * o1 * 10 + o2,
        (p1 != 240) as u8 * p1 * 10 + p2,
        (q1 != 240) as u8 * q1 * 10 + q2,
        (r1 != 240) as u8 * r1 * 10 + r2,
        (s1 != 240) as u8 * s1 * 10 + s2,
        (t1 != 240) as u8 * t1 * 10 + t2,
        (u1 != 240) as u8 * u1 * 10 + u2,
        (v1 != 240) as u8 * v1 * 10 + v2,
        (w1 != 240) as u8 * w1 * 10 + w2,
        (x1 != 240) as u8 * x1 * 10 + x2,
        (y1 != 240) as u8 * y1 * 10 + y2,
    ]
}

struct Bingo {
    field: [u8; 25],
    marked: u32,
}

#[allow(clippy::unusual_byte_groupings)]
const DONE: [u32; 10] = [
    0b11111_00000_00000_00000_00000,
    0b00000_11111_00000_00000_00000,
    0b00000_00000_11111_00000_00000,
    0b00000_00000_00000_11111_00000,
    0b00000_00000_00000_00000_11111,
    0b10000_10000_10000_10000_10000,
    0b01000_01000_01000_01000_01000,
    0b00100_00100_00100_00100_00100,
    0b00010_00010_00010_00010_00010,
    0b00001_00001_00001_00001_00001,
];

impl Bingo {
    fn mark(&mut self, n: u8) -> bool {
        if let Some(idx) = self.field.iter().position(|&elem| elem == n) {
            self.marked |= 1 << idx;

            self.is_done()
        } else {
            false
        }
    }

    fn is_done(&self) -> bool {
        DONE.iter().any(|&mask| self.marked & mask == mask)
    }

    fn sum(&self) -> i64 {
        self.field
            .iter()
            .enumerate()
            .filter_map(|(i, n)| (self.marked & (1 << i) == 0).then(|| *n as i64))
            .sum()
    }
}

struct NumberIter<'a> {
    slice: &'a [u8],
}

impl<'a> NumberIter<'a> {
    fn new(slice: &'a [u8]) -> Self {
        Self { slice }
    }
}

impl Iterator for NumberIter<'_> {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (n, i) = match self.slice {
            [a, b',', ..] | [a] => (*a - b'0', 2),
            [a, b, ..] => (10 * (*a - b'0') + *b - b'0', 3),
            [] => return None,
        };

        self.slice = self.slice.get(i..).unwrap_or_default();

        Some(n)
    }
}
