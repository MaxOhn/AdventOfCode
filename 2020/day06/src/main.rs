use std::hint::unreachable_unchecked;
use std::time::Instant;

#[cfg(not(feature = "functional"))]
fn main() {
    use std::io::{BufRead, BufReader};

    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    static mut QUESTIONS: [u8; 26] = [0; 26];

    let mut p1 = 0;
    let mut p2 = 0;
    let mut group_size = 0;

    while input
        .read_line(&mut line)
        .unwrap_or_else(|_| unsafe { unreachable_unchecked() })
        != 0
    {
        let bytes = line.trim_end().as_bytes();
        let mut i = 0;

        if bytes.is_empty() {
            while i != 26 {
                unsafe {
                    p1 += (*QUESTIONS.get_unchecked(i) > 0) as u16;
                    p2 += (*QUESTIONS.get_unchecked(i) == group_size) as u16;
                    *QUESTIONS.get_unchecked_mut(i) = 0;
                }
                i += 1;
            }
            group_size = 0;
        } else {
            while i != bytes.len() {
                unsafe {
                    *QUESTIONS.get_unchecked_mut((*bytes.get_unchecked(i) - b'a') as usize) += 1
                }
                i += 1;
            }
            group_size += 1;
            line.clear();
        }
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Elapsed: {:?}", start.elapsed()); // 611µs

    assert_eq!(p1, 6742);
    assert_eq!(p2, 3447);
}

#[cfg(feature = "functional")]
fn main() {
    use itertools::Itertools;
    use std::collections::HashSet;

    let start = Instant::now();
    let input =
        std::fs::read_to_string("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let p2: usize = input
        .split("\r\n\r\n")
        .map(|group| {
            group
                .split("\r\n")
                .fold1(|mut all, next| {
                    all.retain(|answer| next.contains(&answer));
                    all
                })
                .unwrap_or_else(|| unsafe { unreachable_unchecked() })
                .len()
        })
        .sum();

    println!("Part 2: {} [{:?}]", p2, start.elapsed());
}
