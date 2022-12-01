use std::ops::Add;

pub fn run(input: &str) {
    let [p1, b, c] = input
        .split("\n\n")
        .map(|group| {
            group
                .split('\n')
                .map(str::parse)
                .map(Result::unwrap)
                .fold(0, <i32 as Add>::add)
        })
        .fold([0, 0, 0], |mut top3, sum| {
            if let Some(pos) = top3.iter().position(|n| *n < sum) {
                top3[pos..].rotate_right(1);
                top3[pos] = sum;
            }

            top3
        });

    let p2 = p1 + b + c;

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}
