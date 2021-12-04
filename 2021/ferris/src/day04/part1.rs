#![feature(core_intrinsics, portable_simd)]

use std::intrinsics::unlikely;

use core_simd::{u8x64, Mask, Simd};

pub fn run(input: &[u8]) -> i64 {
    let new_line = memchr::memchr(b'\n', input).unwrap();

    let mut bingos: Vec<_> = input[new_line..]
        .chunks_exact(76)
        .map(parse_board)
        .map(|field| Bingo { field })
        .collect();

    for n in NumberIter::new(&input[..new_line]) {
        for bingo in bingos.iter_mut() {
            if unlikely(bingo.mark(n)) {
                return bingo.sum() * n as i64;
            }
        }
    }

    unreachable!()
}

fn parse_board(slice: &[u8]) -> [Num; 25] {
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

    let mask = Mask::from_array([
        true, false, true, false, true, false, true, false, true, false, true, false, true, false,
        true, false, true, false, true, false, true, false, true, false, true, false, true, false,
        true, false, true, false, true, false, true, false, true, false, true, false, true, false,
        true, false, true, false, true, false, true, false, false, false, false, false, false,
        false, false, false, false, false, false, false, false, false,
    ]);

    let simd = mask.select(simd * Simd::splat(10), simd);
    let [a1, a2, b1, b2, c1, c2, d1, d2, e1, e2, f1, f2, g1, g2, h1, h2, i1, i2, j1, j2, k1, k2, l1, l2, m1, m2, n1, n2, o1, o2, p1, p2, q1, q2, r1, r2, s1, s2, t1, t2, u1, u2, v1, v2, w1, w2, x1, x2, y1, y2, ..] =
        simd.to_array();

    // * Note: b' '.wrapping_sub(b'0').wrapping_mul(10) = 96
    [
        Num((a1 != 96) as u8 * a1 + a2),
        Num((b1 != 96) as u8 * b1 + b2),
        Num((c1 != 96) as u8 * c1 + c2),
        Num((d1 != 96) as u8 * d1 + d2),
        Num((e1 != 96) as u8 * e1 + e2),
        Num((f1 != 96) as u8 * f1 + f2),
        Num((g1 != 96) as u8 * g1 + g2),
        Num((h1 != 96) as u8 * h1 + h2),
        Num((i1 != 96) as u8 * i1 + i2),
        Num((j1 != 96) as u8 * j1 + j2),
        Num((k1 != 96) as u8 * k1 + k2),
        Num((l1 != 96) as u8 * l1 + l2),
        Num((m1 != 96) as u8 * m1 + m2),
        Num((n1 != 96) as u8 * n1 + n2),
        Num((o1 != 96) as u8 * o1 + o2),
        Num((p1 != 96) as u8 * p1 + p2),
        Num((q1 != 96) as u8 * q1 + q2),
        Num((r1 != 96) as u8 * r1 + r2),
        Num((s1 != 96) as u8 * s1 + s2),
        Num((t1 != 96) as u8 * t1 + t2),
        Num((u1 != 96) as u8 * u1 + u2),
        Num((v1 != 96) as u8 * v1 + v2),
        Num((w1 != 96) as u8 * w1 + w2),
        Num((x1 != 96) as u8 * x1 + x2),
        Num((y1 != 96) as u8 * y1 + y2),
    ]
}

#[derive(Copy, Clone, Debug)]
struct Num(u8);

impl Num {
    const MARK: u8 = 0b1000_0000;

    fn is_marked(self) -> bool {
        self.0 >= Self::MARK
    }

    /// Returns whether it was newly marked
    fn mark(&mut self) -> bool {
        let was_marked = self.is_marked();
        self.0 |= Self::MARK;

        !was_marked
    }
}

struct Bingo {
    field: [Num; 25],
}

impl Bingo {
    /// Returns whether the board is now done
    fn mark(&mut self, n: u8) -> bool {
        let marked = self
            .field
            .iter_mut()
            .filter(|num| num.0 == n)
            .fold(false, |marked, elem| marked | elem.mark());

        if marked {
            self.is_done()
        } else {
            false
        }
    }

    fn is_done(&self) -> bool {
        for row in self.field.chunks_exact(5) {
            if row.iter().copied().all(Num::is_marked) {
                return true;
            }
        }

        'outer: for col in 0..5 {
            for row in 0..5 {
                if !self.field[row * 5 + col].is_marked() {
                    continue 'outer;
                }
            }

            return true;
        }

        false
    }

    fn sum(&self) -> i64 {
        self.field
            .iter()
            .filter(|n| !n.is_marked())
            .map(|n| n.0 as i64)
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
