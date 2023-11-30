use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    let mut p1 = 0;
    static mut SEATS: [bool; 1024] = [false; 1024];

    for line in input.lines() {
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

    Ok(Solution::new().part1(p1).part2(p2))
}
