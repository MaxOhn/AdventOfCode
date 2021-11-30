use std::hint::unreachable_unchecked;
use std::io::{BufRead, BufReader};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let file =
        std::fs::File::open("./input").unwrap_or_else(|_| unsafe { unreachable_unchecked() });
    let mut input = BufReader::new(file);

    let mut line = String::new();

    let mut p1 = 0;
    static mut SEATS: [bool; 1024] = [false; 1024];

    while input
        .read_line(&mut line)
        .unwrap_or_else(|_| unsafe { unreachable_unchecked() })
        != 0
    {
        let bytes = line.as_bytes();

        let mut row = 0;
        let mut pow = 64;

        let mut i = 0;
        while i != 7 {
            row += (unsafe { *bytes.get_unchecked(i) } == b'B') as usize * pow;
            pow /= 2;
            i += 1;
        }

        let mut col = 0;
        let mut pow = 4;

        while i != 10 {
            col += (unsafe { *bytes.get_unchecked(i) } == b'R') as usize * pow;
            pow /= 2;
            i += 1;
        }

        let id = 8 * row + col;

        p1 = p1.max(id);
        unsafe { *SEATS.get_unchecked_mut(id) = true }

        line.clear();
    }

    let mut p2 = 1;
    loop {
        if unsafe { *SEATS.get_unchecked(p2) } {
            p2 += 1;
        } else if unsafe { *SEATS.get_unchecked(p2 + 1) } {
            if unsafe { *SEATS.get_unchecked(p2 - 1) } {
                break;
            } else {
                p2 += 2;
            }
        } else {
            p2 += 3;
        }
    }

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
    println!("Elapsed: {:?}", start.elapsed()); // 730µs

    assert_eq!(p1, 922);
    assert_eq!(p2, 747);
}
