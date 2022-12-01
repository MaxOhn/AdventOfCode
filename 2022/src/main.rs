days! {
    > day01,
}

#[macro_export]
macro_rules! days {
    ( $( $pre:ident ,)* > $day:ident, $( $post:ident ,)* ) => {
        mod $day;

        fn main() {
            let input = include_str!(concat!(r#"..\inputs\"#, stringify!($day), ".txt"));
            $day::run(input);
        }
    };
}
