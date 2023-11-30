use std::{
    fmt::{Debug, Display, Formatter, Result},
    ops::{
        Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Rem, RemAssign, Sub,
        SubAssign,
    },
};

#[derive(Copy, Clone, Default, Eq, Hash, PartialEq)]
pub struct Pos3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Pos3<T> {
    #[inline]
    #[allow(unused)]
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T> Pos3<T>
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

        let c = if self.z > other.z {
            self.z - other.z
        } else {
            other.z - self.z
        };

        a + b + c
    }
}

macro_rules! impl_pos2_decimal {
    ($($type:ty),*) => {
        $(
            impl Pos3<$type> {
                #[inline]
                #[allow(unused)]
                /// Return the position's length.
                pub fn len(&self) -> $type {
                    (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
                }

                #[inline]
                #[allow(unused)]
                /// Return the distance to another position.
                pub fn dist(&self, other: Self) -> $type {
                    (*self - other).len()
                }

                #[inline]
                #[allow(unused)]
                /// Normalize the coordinates with respect to the vector's length.
                pub fn normalize(self) -> Self {
                    self / self.len()
                }
            }
        )*
    };
}

impl_pos2_decimal!(f32, f64);

impl<T: Debug> Debug for Pos3<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.debug_struct("Pos3")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}

impl<T: Display> Display for Pos3<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl<T: Add<Output = T>> Add for Pos3<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Pos3<T> {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<U: Copy, T: Mul<U, Output = T>> Mul<U> for Pos3<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: U) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<U: Copy, T: Div<U, Output = T>> Div<U> for Pos3<T> {
    type Output = Self;

    #[inline]
    fn div(self, rhs: U) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<U: Copy, T: Rem<U, Output = T>> Rem<U> for Pos3<T> {
    type Output = Self;

    #[inline]
    fn rem(self, rhs: U) -> Self::Output {
        Self {
            x: self.x % rhs,
            y: self.y % rhs,
            z: self.z % rhs,
        }
    }
}

impl<T: AddAssign> AddAssign for Pos3<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: SubAssign> SubAssign for Pos3<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<U: Copy, T: MulAssign<U>> MulAssign<U> for Pos3<T> {
    #[inline]
    fn mul_assign(&mut self, rhs: U) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl<U: Copy, T: DivAssign<U>> DivAssign<U> for Pos3<T> {
    #[inline]
    fn div_assign(&mut self, rhs: U) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl<U: Copy, T: RemAssign<U>> RemAssign<U> for Pos3<T> {
    #[inline]
    fn rem_assign(&mut self, rhs: U) {
        self.x %= rhs;
        self.y %= rhs;
        self.z %= rhs;
    }
}

macro_rules! impl_index {
        ($($ty:ty),+) => {
            $(
                impl<T> Index<$ty> for Pos3<T> {
                    type Output = T;

                    #[inline]
                    fn index(&self, idx: $ty) -> &Self::Output {
                        if idx == 0 {
                            &self.x
                        } else if idx == 1 {
                            &self.y
                        } else if idx == 2 {
                            &self.z
                        } else {
                            panic!("index must be 0, 1, or 2")
                        }
                    }
                }

                impl<T> IndexMut<$ty> for Pos3<T> {
                    #[inline]
                    fn index_mut(&mut self, idx: $ty) -> &mut Self::Output {
                        if idx == 0 {
                            &mut self.x
                        } else if idx == 1 {
                            &mut self.y
                        } else if idx == 2 {
                            &mut self.z
                        } else {
                            panic!("index must be 0, 1, or 2")
                        }
                    }
                }
            )+
        };
    }

impl_index!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
