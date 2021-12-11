use std::{
    fmt::{Debug, Display, Formatter, Result},
    ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, RangeInclusive, Rem,
        RemAssign, Sub, SubAssign,
    },
};

#[derive(Copy, Clone, Default, Eq, Hash, PartialEq)]
pub struct Pos2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Pos2<T> {
    #[inline]
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Pos2<T>
where
    T: Copy + PartialOrd + Add<Output = T> + Sub<Output = T>,
{
    #[inline]
    /// Return the manhatten distance to another position.
    pub fn manhatten_dist(&self, other: &Self) -> T {
        let a = if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };

        let b = if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };

        a + b
    }
}

macro_rules! impl_pos2_decimal {
    ($($type:ty),*) => {
        $(
            impl Pos2<$type> {
                #[inline]
                /// Return the position's length.
                pub fn len(&self) -> $type {
                    self.x.hypot(self.y)
                }

                #[inline]
                /// Return the distance to another position.
                pub fn dist(&self, other: Self) -> $type {
                    (*self - other).len()
                }

                #[inline]
                /// Normalize the coordinates with respect to the vector's length.
                pub fn normalize(self) -> Self {
                    self / self.len()
                }
            }
        )*
    };
}

impl_pos2_decimal!(f32, f64);

impl<T: Debug> Debug for Pos2<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.debug_struct("Pos2")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

impl<T: Display> Display for Pos2<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl<T: Add<Output = T>> Add for Pos2<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Pos2<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<U: Copy, T: Mul<U, Output = T>> Mul<U> for Pos2<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: U) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<U: Copy, T: Div<U, Output = T>> Div<U> for Pos2<T> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: U) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<U: Copy, T: Rem<U, Output = T>> Rem<U> for Pos2<T> {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: U) -> Self::Output {
        Self {
            x: self.x % rhs,
            y: self.y % rhs,
        }
    }
}

impl<T: AddAssign> AddAssign for Pos2<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: SubAssign> SubAssign for Pos2<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<U: Copy, T: MulAssign<U>> MulAssign<U> for Pos2<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: U) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<U: Copy, T: DivAssign<U>> DivAssign<U> for Pos2<T> {
    #[inline]
    fn div_assign(&mut self, rhs: U) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<U: Copy, T: RemAssign<U>> RemAssign<U> for Pos2<T> {
    #[inline]
    fn rem_assign(&mut self, rhs: U) {
        self.x %= rhs;
        self.y %= rhs;
    }
}

macro_rules! impl_index {
    ($($ty:ty),+) => {
        $(
            impl<T> Index<$ty> for Pos2<T> {
                type Output = T;

                #[inline]
                fn index(&self, idx: $ty) -> &Self::Output {
                    if idx == 0 {
                        &self.x
                    } else if idx == 1 {
                        &self.y
                    } else {
                        panic!("index must be 0 or 1")
                    }
                }
            }

            impl<T> IndexMut<$ty> for Pos2<T> {
                #[inline]
                fn index_mut(&mut self, idx: $ty) -> &mut Self::Output {
                    if idx == 0 {
                        &mut self.x
                    } else if idx == 1 {
                        &mut self.y
                    } else {
                        panic!("index must be 0 or 1")
                    }
                }
            }
            )+
    };
}

impl_index!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

// Can be generic once `std::iter::Step` is stabilized
pub struct Pos2Iter {
    x_range_curr: RangeInclusive<usize>,
    x_range: RangeInclusive<usize>,
    y_range: RangeInclusive<usize>,
    y: Option<usize>,
}

impl Pos2Iter {
    #[inline]
    pub fn new(min_x: usize, max_x: usize, min_y: usize, max_y: usize) -> Self {
        let x_range = min_x..=max_x;
        let mut y_range = min_y..=max_y;

        Self {
            x_range_curr: x_range.clone(),
            y: y_range.next(),
            x_range,
            y_range,
        }
    }
}

impl Iterator for Pos2Iter {
    type Item = Pos2<usize>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (x, y) = match self.x_range_curr.next() {
            Some(x) => (x, self.y?),
            None => {
                self.x_range_curr = self.x_range.clone();
                self.y = self.y_range.next();

                (self.x_range_curr.next()?, self.y?)
            }
        };

        Some(Pos2::new(x, y))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter() {
        let mut iter = Pos2Iter::new(2, 4, 0, 1);

        assert_eq!(iter.next(), Some(Pos2::new(2, 0)));
        assert_eq!(iter.next(), Some(Pos2::new(3, 0)));
        assert_eq!(iter.next(), Some(Pos2::new(4, 0)));
        assert_eq!(iter.next(), Some(Pos2::new(2, 1)));
        assert_eq!(iter.next(), Some(Pos2::new(3, 1)));
        assert_eq!(iter.next(), Some(Pos2::new(4, 1)));
        assert_eq!(iter.next(), None);
    }
}
