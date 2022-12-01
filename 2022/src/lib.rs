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
        mod $day;

        fn main() {
            let start = std::time::Instant::now();
            let path = concat!("./inputs/", stringify!($day), ".txt");
            let file = std::fs::File::open(path).unwrap();

            // SAFETY: no :)
            let input = unsafe { memmap::Mmap::map(&file) }.unwrap();

            let solution = $day::run(&input);
            let elapsed = start.elapsed();

            print!("{solution}");
            println!("Elapsed: {elapsed:?}");
        }
    };
}
