/// Parse numbers efficiently
pub trait Parse<N> {
    fn parse(&self) -> N;
}

macro_rules! impl_parse_u {
    ($($uint:ty),*) => {
        $(
            impl Parse<$uint> for &[u8] {
                #[inline]
                fn parse(&self) -> $uint {
                    let mut n = 0;
                    let mut i = 0;
                    let mut j = self.len() - 1;

                    while {
                        let c = unsafe { *self.get_unchecked(j) };
                        c == b'\n' || c == b'\r'
                    } {
                        j -= 1;
                    }

                    while i <= j {
                        n = n * 10 + (unsafe { *self.get_unchecked(i) } & 0x0F) as $uint;
                        i += 1;
                    }

                    n
                }
            }
        )*
    };
}

macro_rules! impl_parse_i {
    ($($int:ty),*) => {
        $(
            impl Parse<$int> for &[u8] {
                #[inline]
                fn parse(&self) -> $int {
                    let (mut n, sig) = match unsafe { *self.get_unchecked(0) } {
                        b'-' => (0, -1),
                        other => ((other & 0x0F) as $int, 1),
                    };

                    let mut i = 1;
                    let mut j = self.len() - 1;

                    while {
                        let c = unsafe { *self.get_unchecked(j) };
                        c == b'\n' || c == b'\r'
                    } {
                        j -= 1;
                    }

                    while i <= j {
                        n = n * 10 + (unsafe { *self.get_unchecked(i) } & 0x0F) as $int;
                        i += 1;
                    }

                    n * sig
                }
            }
        )*
    };
}

impl_parse_u!(u8, u16, u32, u64, usize);
impl_parse_i!(i8, i16, i32, i64, isize);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_u32() {
        let mut res: u32 = "3456789123".as_bytes().parse();
        assert_eq!(res, 3_456_789_123);

        res = "3456789123\r\n".as_bytes().parse();
        assert_eq!(res, 3_456_789_123);

        res = "3456789123
"
        .as_bytes()
        .parse();
        assert_eq!(res, 3_456_789_123);
    }

    #[test]
    fn parse_i32_positive() {
        let mut res: i32 = "1456789123".as_bytes().parse();
        assert_eq!(res, 1456789123);

        res = "1456789123\r\n".as_bytes().parse();
        assert_eq!(res, 1456789123);

        res = "1456789123
"
        .as_bytes()
        .parse();
        assert_eq!(res, 1456789123);
    }

    #[test]
    fn parse_i32_negative() {
        let mut res: i32 = "-1456789123".as_bytes().parse();
        assert_eq!(res, -1456789123);

        res = "-1456789123\r\n".as_bytes().parse();
        assert_eq!(res, -1456789123);

        res = "-1456789123
"
        .as_bytes()
        .parse();
        assert_eq!(res, -1456789123);
    }
}
