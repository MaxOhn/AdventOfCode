macro_rules! parse {
    ( UINT: $( $ty:ty ),* ) => {
        $(
            impl Parseable for $ty {
                #[inline]
                fn parse(bytes: &[u8]) -> Self {
                    bytes
                        .iter()
                        .fold(0, |sum, &byte| 10 * sum + (byte & 0xF) as $ty)
                }
            }
        )*
    };
    ( INT: $( $ty:ty ),* ) => {
        $(
            impl Parseable for $ty {
                #[inline]
                fn parse(bytes: &[u8]) -> Self {
                    let (sig, skip) = if bytes.first() == Some(&b'-') {
                        (-1, 1)
                    } else {
                        (1, 0)
                    };

                    let n = bytes
                        .iter()
                        .skip(skip)
                        .fold(0, |sum, &byte| 10 * sum + (byte & 0xF) as $ty);

                    sig * n
                }
            }
        )*
    };
}

parse!(UINT: u8, u16, u32, u64, usize);
parse!(INT: i8, i16, i32, i64, isize);

pub trait Parseable {
    fn parse(bytes: &[u8]) -> Self;
}

#[macro_export]
macro_rules! get {
    ( $slice:ident[$idx:expr] ) => {
        unsafe { *$slice.get_unchecked($idx) }
    };
}

#[macro_export]
macro_rules! get_mut {
    ( $slice:ident[$idx:expr] ) => {
        unsafe { $slice.get_unchecked_mut($idx) }
    };
}

#[macro_export]
macro_rules! set {
    ( $slice:ident[$idx:expr] $op:tt $val:expr ) => {
        unsafe { *$slice.get_unchecked_mut($idx) $op $val }
    };
}
