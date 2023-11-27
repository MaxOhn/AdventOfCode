use std::ops::{Div, Mul, Rem};

/// Greatest common divisor
pub fn gcd<N>(a: N, b: N) -> N
where
    N: Copy + Default + PartialEq + Rem<Output = N>,
{
    if b == N::default() {
        a
    } else {
        gcd(b, a % b)
    }
}

#[inline]
/// Least common multiple
pub fn lcm<N>(a: N, b: N) -> N
where
    N: Copy + Default + PartialEq + Div<Output = N> + Mul<Output = N> + Rem<Output = N>,
{
    a * b / gcd(a, b)
}

/// Calculate `(b^p)%m`
pub fn mod_pow(b: i64, p: i64, m: i64) -> i64 {
    match p {
        0 => 1,
        e if e % 2 == 0 => mod_pow((b * b) % m, e / 2, m),
        _ => (b * mod_pow(b, p - 1, m)) % m,
    }
}
