#[macro_export]
macro_rules! get {
    ($slice:expr, $idx:expr) => {
        unsafe { *$slice.get_unchecked($idx) }
    };
}
#[macro_export]
macro_rules! get_ref {
    ($slice:expr, $idx:expr) => {
        unsafe { $slice.get_unchecked($idx) }
    };
}

#[macro_export]
macro_rules! get_mut {
    ($slice:expr, $idx:expr) => {
        $slice.get_unchecked_mut($idx)
    };
}

#[macro_export]
macro_rules! set {
    ($slice:expr, $idx:expr, $val:expr) => {
        unsafe { *get_mut!($slice, $idx) = $val }
    };
}

#[macro_export]
macro_rules! swap {
    ($a:expr, $b:expr) => {
        std::mem::swap($a, $b)
    };
}

#[macro_export]
macro_rules! swap_elems {
    ($slice:expr, $i:expr, $j:expr) => {
        unsafe { std::ptr::swap(get_mut!($slice, $i), get_mut!($slice, $j)) }
    };
}

pub trait Parse {
    fn parse(bytes: &[u8]) -> Self;
}

macro_rules! impl_parse_u {
    ($type:ty) => {
        impl Parse for $type {
            fn parse(bytes: &[u8]) -> Self {
                let mut n = 0;
                let mut i = 0;
                let mut j = bytes.len() - 1;

                while get!(bytes, j) == b'\n' {
                    j -= 1;
                }

                while i <= j {
                    n = n * 10 + (get!(bytes, i) & 0x0F) as $type;
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
                let (mut n, sig) = match get!(bytes, 0) {
                    b'-' => (0, -1),
                    other => ((other & 0x0F) as $type, 1),
                };

                let mut i = 1;
                let mut j = bytes.len() - 1;

                while get!(bytes, j) == b'\n' {
                    j -= 1;
                }

                while i <= j {
                    n = n * 10 + (get!(bytes, i) & 0x0F) as $type;
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
