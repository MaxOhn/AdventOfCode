#![feature(portable_simd)]

days! {
    day01,
    > day02,
}

mod solution;
mod util;

pub mod prelude {
    pub use super::{
        solution::{Solution, SolutionType},
        util::Parseable,
    };
}

#[macro_export]
macro_rules! days {
    ( $( $pre:ident ,)* > $day:ident, $( $post:ident ,)* ) => {
        pub mod $day;

        pub mod today {
            pub fn run() -> super::solution::Solution {
                let path = concat!("./inputs/", stringify!($day), ".txt");
                let file = std::fs::File::open(path).unwrap();

                // SAFETY: no :)
                let input = unsafe { memmap::Mmap::map(&file) }.unwrap();

                super::$day::run(&input)
            }
        }
    };
}
