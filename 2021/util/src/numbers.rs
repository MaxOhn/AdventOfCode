use std::ops::{Div, Mul, Rem};

/// greatest common divisor
pub fn gcd<N>(a: N, b: N) -> N
where
    N: Rem<Output = N> + Copy + Default + Eq,
{
    if b == N::default() {
        a
    } else {
        gcd(b, a % b)
    }
}

#[inline]
/// least common multiple
pub fn lcm<N>(a: N, b: N) -> N
where
    N: Mul<Output = N> + Div<Output = N> + Rem<Output = N> + Copy + Default + Eq,
{
    a * b / gcd(a, b)
}
