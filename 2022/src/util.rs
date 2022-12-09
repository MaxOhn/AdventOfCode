use std::hash::{BuildHasher, Hasher};

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
    ( $( $slice:ident ).+ [$idx:expr] ) => {
        unsafe { * $( $slice. )* get_unchecked($idx) }
    };
}

#[macro_export]
macro_rules! get_mut {
    ( $( $slice:ident ).+ [$idx:expr] ) => {
        unsafe { $( $slice. )* get_unchecked_mut($idx) }
    };
}

#[macro_export]
macro_rules! set {
    ( $slice:ident[$idx:expr] $op:tt $val:expr ) => {
        unsafe { *$slice.get_unchecked_mut($idx) $op $val }
    };
}

pub struct IntHasher;

impl BuildHasher for IntHasher {
    type Hasher = IntHash;

    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        IntHash(0)
    }
}

pub struct IntHash(u64);

#[rustfmt::skip]
impl Hasher for IntHash {
    fn write(&mut self, _: &[u8])     {}
    fn write_u128(&mut self, _: u128) {}
    fn write_i128(&mut self, _: i128) {}

    #[inline] fn write_u8(&mut self, n: u8)       { self.0 = u64::from(n) }
    #[inline] fn write_u16(&mut self, n: u16)     { self.0 = u64::from(n) }
    #[inline] fn write_u32(&mut self, n: u32)     { self.0 = u64::from(n) }
    #[inline] fn write_u64(&mut self, n: u64)     { self.0 = n            }
    #[inline] fn write_usize(&mut self, n: usize) { self.0 = n as u64     }

    #[inline] fn write_i8(&mut self, n: i8)       { self.0 = n as u64 }
    #[inline] fn write_i16(&mut self, n: i16)     { self.0 = n as u64 }
    #[inline] fn write_i32(&mut self, n: i32)     { self.0 = n as u64 }
    #[inline] fn write_i64(&mut self, n: i64)     { self.0 = n as u64 }
    #[inline] fn write_isize(&mut self, n: isize) { self.0 = n as u64 }

    #[inline] fn finish(&self) -> u64 { self.0 }
}
