macro_rules! solution {
    ( $( $variant:ident($ty:ty) ,)* ) => {
        #[derive(Clone)]
        pub enum SolutionType {
            $( $variant($ty), )*
            Unsolved,
        }

        impl Display for SolutionType {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                match self {
                    $( Self::$variant(n) => write!(f, "{n}") ,)*
                    Self::Unsolved => f.write_str("TODO"),
                }
            }
        }

        impl Default for SolutionType {
            #[inline]
            fn default() -> Self {
                Self::Unsolved
            }
        }

        $(
            impl From<$ty> for SolutionType {
                #[inline]
                fn from(n: $ty) -> Self {
                    Self::$variant(n)
                }
            }

            impl PartialEq<$ty> for SolutionType {
                fn eq(&self, other: &$ty) -> bool {
                    if let Self::$variant(n) = self {
                        n == other
                    } else {
                        false
                    }
                }
            }
        )*
    };
}

use std::fmt::{Display, Formatter, Result as FmtResult};

solution! {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    Usize(usize),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    Isize(isize),
    F32(f32),
    F64(f64),
    String(String),
}

#[derive(Clone, Default)]
pub struct Solution {
    pub part1: SolutionType,
    pub part2: SolutionType,
}

impl Solution {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn part1<T>(self, part1: T) -> Self
    where
        SolutionType: From<T>,
    {
        Self {
            part1: part1.into(),
            part2: self.part2,
        }
    }

    pub fn part2<T>(self, part2: T) -> Self
    where
        SolutionType: From<T>,
    {
        Self {
            part1: self.part1,
            part2: part2.into(),
        }
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "Part 1: {}", self.part1)?;
        writeln!(f, "Part 2: {}", self.part2)?;

        Ok(())
    }
}
