#![cfg_attr(feature = "nightly", feature(portable_simd))]

#[macro_use]
extern crate eyre;

#[macro_use]
mod util;

mod solution;

modules! {
    day01,
    day02,
    day03,
    day04,
    day05,
    day06,
    day07,
    day08,
    day09,
    day10,
    day11,
    day12,
    day13,
    day14,
    day15,
    day16,
    day17,
    > day18,
}

pub mod prelude {
    pub use super::{solution::Solution, util::Parseable};

    pub use eyre::{ContextCompat, Report, Result, WrapErr};
}

#[macro_export]
macro_rules! modules {
    ( $( $pre:ident ,)* ) => {
        compile_error!("One day must be prefixed with `> `")
    };
    ( $( $pre:ident ,)* > $current:ident, $( $post:ident ,)* ) => {
        $( pub mod $pre; )*
        pub mod $current;
        $( pub mod $post; )*

        #[cfg(not(target_arch = "wasm32"))]
        pub mod current {
            use super::prelude::*;

            pub fn run() -> Result<Solution> {
                let path = concat!("./inputs/", stringify!($current), ".txt");
                let file = std::fs::File::open(path)
                    .wrap_err_with(|| format!("failed to open file at path `{path}`"))?;

                let mmap = unsafe { memmap::Mmap::map(&file) }.wrap_err("failed to memory map")?;
                let input = unsafe { std::str::from_utf8_unchecked(&mmap) };

                super::$current::run(input)
                    .wrap_err(concat!("failed to run day ", stringify!($current)))
            }
        }
    }
}
