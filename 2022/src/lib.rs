#[macro_export]
macro_rules! days {
    ( $( $pre:ident ,)* > $day:ident, $( $post:ident ,)* ) => {
        mod $day;

        fn main() {
            let path = concat!("./inputs/", stringify!($day), ".txt");
            let file = std::fs::File::open(path).unwrap();

            // SAFETY: no :)
            let mmap = unsafe { memmap::Mmap::map(&file) }.unwrap();
            let input = unsafe { std::str::from_utf8_unchecked(&mmap) };

            $day::run(input);
        }
    };
}
