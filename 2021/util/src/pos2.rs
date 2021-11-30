use std::{
    fmt::{Debug, Display, Formatter, Result},
    ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Rem, RemAssign, Sub,
        SubAssign,
    },
};

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

impl<T: PartialEq> PartialEq for Pos2<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y)
    }

    #[inline]
    fn ne(&self, other: &Self) -> bool {
        self.x.ne(&other.x) || self.y.ne(&other.y)
    }
}

impl<T: Eq> Eq for Pos2<T> {}

impl<T: Default> Default for Pos2<T> {
    #[inline]
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
        }
    }
}

impl<T: Copy> Copy for Pos2<T> {}

impl<T: Clone> Clone for Pos2<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

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
