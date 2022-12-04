#![feature(portable_simd)]

#[macro_use]
mod util;

mod solution;

modules! {
    day01,
    day02,
    day03,
    > day04,
}

pub mod prelude {
    pub use super::{solution::Solution, util::Parseable};
}

#[macro_export]
macro_rules! modules {
    ( $( $pre:ident ,)* > $current:ident, $( $post:ident ,)* ) => {
        $( pub mod $pre; )*
        pub mod $current;
        $( pub mod $post; )*

        #[cfg(not(target_arch = "wasm32"))]
        pub mod current {
            pub fn run() -> super::prelude::Solution {
                let path = concat!("./inputs/", stringify!($current), ".txt");
                let file = std::fs::File::open(path).unwrap();

                let mmap = unsafe { memmap::Mmap::map(&file) }.unwrap();
                let input = unsafe { std::str::from_utf8_unchecked(&mmap) };

                super::$current::run(input)
            }
        }
    }
}
