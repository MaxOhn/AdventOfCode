use aoc_rust::Solution;
use eyre::Result;

pub fn run(input: &str) -> Result<Solution> {
    static mut QUESTIONS: [u8; 26] = [0; 26];

    let mut p1 = 0;
    let mut p2 = 0;
    let mut group_size = 0;

    for line in input.lines() {
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
        }
    }

    Ok(Solution::new().part1(p1).part2(p2))
}
