/// Parse numbers efficiently
pub trait Parse: Sized {
    fn parse(bytes: &[u8]) -> Self;
}

macro_rules! impl_parse_u {
    ($type:ty) => {
        impl Parse for $type {
            fn parse(bytes: &[u8]) -> Self {
                let mut n = 0;
                let mut i = 0;
                let mut j = bytes.len() - 1;

                while {
                    let c = unsafe { *bytes.get_unchecked(j) };
                    c == b'\n' || c == b'\r'
                } {
                    j -= 1;
                }

                while i <= j {
                    n = n * 10 + (unsafe { *bytes.get_unchecked(i) } & 0x0F) as $type;
                    i += 1;
                }

                n
            }
        }
    };
}

macro_rules! impl_parse_i {
    ($type:ty) => {
        impl Parse for $type {
            fn parse(bytes: &[u8]) -> Self {
                let (mut n, sig) = match unsafe { *bytes.get_unchecked(0) } {
                    b'-' => (0, -1),
                    other => ((other & 0x0F) as $type, 1),
                };

                let mut i = 1;
                let mut j = bytes.len() - 1;

                while {
                    let c = unsafe { *bytes.get_unchecked(j) };
                    c == b'\n' || c == b'\r'
                } {
                    j -= 1;
                }

                while i <= j {
                    n = n * 10 + (unsafe { *bytes.get_unchecked(i) } & 0x0F) as $type;
                    i += 1;
                }

                n * sig
            }
        }
    };
}

impl_parse_u!(u8);
impl_parse_u!(u16);
impl_parse_u!(u32);
impl_parse_u!(u64);
impl_parse_u!(usize);

impl_parse_i!(i8);
impl_parse_i!(i16);
impl_parse_i!(i32);
impl_parse_i!(i64);
impl_parse_i!(isize);

// ----- Custom hasher -----

pub struct NumHasherBuilder;

impl std::hash::BuildHasher for NumHasherBuilder {
    type Hasher = NumHasher;

    fn build_hasher(&self) -> Self::Hasher {
        NumHasher(0)
    }
}

pub struct NumHasher(u64);

impl std::hash::Hasher for NumHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        self.0 = unsafe { *(bytes.as_ptr() as *const u64) };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_u32() {
        let mut res: u32 = Parse::parse("3456789123".as_bytes());
        assert_eq!(res, 3_456_789_123);

        res = Parse::parse("3456789123\r\n".as_bytes());
        assert_eq!(res, 3_456_789_123);

        res = Parse::parse(
            "3456789123
"
            .as_bytes(),
        );
        assert_eq!(res, 3_456_789_123);
    }

    #[test]
    fn parse_i32_positive() {
        let mut res: i32 = Parse::parse("1456789123".as_bytes());
        assert_eq!(res, 1456789123);

        res = Parse::parse("1456789123\r\n".as_bytes());
        assert_eq!(res, 1456789123);

        res = Parse::parse(
            "1456789123
"
            .as_bytes(),
        );
        assert_eq!(res, 1456789123);
    }

    #[test]
    fn parse_i32_negative() {
        let mut res: i32 = Parse::parse("-1456789123".as_bytes());
        assert_eq!(res, -1456789123);

        res = Parse::parse("-1456789123\r\n".as_bytes());
        assert_eq!(res, -1456789123);

        res = Parse::parse(
            "-1456789123
"
            .as_bytes(),
        );
        assert_eq!(res, -1456789123);
    }
}
