use aoc22::prelude::Solution;

pub fn run(input: &[u8]) -> Solution {
    let p1 = input
        .chunks(4)
        .fold(RockPaperScissors::default(), |rps, chunk| {
            match &chunk[..3] {
                [b'A', _, b'X'] => rps.draw(1),
                [b'A', _, b'Y'] => rps.win(2),
                [b'A', _, b'Z'] => rps.loss(3),
                [b'B', _, b'X'] => rps.loss(1),
                [b'B', _, b'Y'] => rps.draw(2),
                [b'B', _, b'Z'] => rps.win(3),
                [b'C', _, b'X'] => rps.win(1),
                [b'C', _, b'Y'] => rps.loss(2),
                [b'C', _, b'Z'] => rps.draw(3),
                _ => unreachable!("{:?}", chunk),
            }
        });

    let p2 = input
        .chunks(4)
        .fold(RockPaperScissors::default(), |rps, chunk| {
            match &chunk[..3] {
                [b'A', _, b'X'] => rps.loss(3),
                [b'A', _, b'Y'] => rps.draw(1),
                [b'A', _, b'Z'] => rps.win(2),
                [b'B', _, b'X'] => rps.loss(1),
                [b'B', _, b'Y'] => rps.draw(2),
                [b'B', _, b'Z'] => rps.win(3),
                [b'C', _, b'X'] => rps.loss(2),
                [b'C', _, b'Y'] => rps.draw(3),
                [b'C', _, b'Z'] => rps.win(1),
                _ => unreachable!("{:?}", chunk),
            }
        });

    Solution::new().part1(p1.score).part2(p2.score)
}

#[derive(Default)]
struct RockPaperScissors {
    score: u16,
}

impl RockPaperScissors {
    fn win(mut self, shape: u16) -> Self {
        self.score += shape + 6;

        self
    }

    fn draw(mut self, shape: u16) -> Self {
        self.score += shape + 3;

        self
    }

    fn loss(mut self, shape: u16) -> Self {
        self.score += shape;

        self
    }
}
