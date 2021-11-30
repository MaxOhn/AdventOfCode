use std::ops::{Div, Mul, Rem};

/// greatest common divisor
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
/// least common multiple
pub fn lcm<N>(a: N, b: N) -> N
where
    N: Copy + Default + PartialEq + Div<Output = N> + Mul<Output = N> + Rem<Output = N>,
{
    a * b / gcd(a, b)
}
