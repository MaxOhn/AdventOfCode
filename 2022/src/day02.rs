use aoc22::prelude::Solution;

pub fn run(input: &[u8]) -> Solution {
    let p1 = input
        .chunks(4)
        .fold(Results::default(), |results, chunk| match &chunk[..3] {
            [b'A', _, b'X'] => results.draw(1),
            [b'A', _, b'Y'] => results.win(2),
            [b'A', _, b'Z'] => results.loss(3),
            [b'B', _, b'X'] => results.loss(1),
            [b'B', _, b'Y'] => results.draw(2),
            [b'B', _, b'Z'] => results.win(3),
            [b'C', _, b'X'] => results.win(1),
            [b'C', _, b'Y'] => results.loss(2),
            [b'C', _, b'Z'] => results.draw(3),
            _ => unreachable!("{:?}", chunk),
        });

    let p2 = input
        .chunks(4)
        .fold(Results::default(), |results, chunk| match &chunk[..3] {
            [b'A', _, b'X'] => results.loss(3),
            [b'A', _, b'Y'] => results.draw(1),
            [b'A', _, b'Z'] => results.win(2),
            [b'B', _, b'X'] => results.loss(1),
            [b'B', _, b'Y'] => results.draw(2),
            [b'B', _, b'Z'] => results.win(3),
            [b'C', _, b'X'] => results.loss(2),
            [b'C', _, b'Y'] => results.draw(3),
            [b'C', _, b'Z'] => results.win(1),
            _ => unreachable!("{:?}", chunk),
        });

    Solution::new().part1(p1.sum()).part2(p2.sum())
}

#[derive(Default)]
struct Results {
    wins: u16,
    draws: u16,
    losses: u16,
}

impl Results {
    fn win(mut self, shape: u16) -> Self {
        self.wins += shape + 6;

        self
    }

    fn draw(mut self, shape: u16) -> Self {
        self.draws += shape + 3;

        self
    }

    fn loss(mut self, shape: u16) -> Self {
        self.losses += shape;

        self
    }

    fn sum(self) -> u16 {
        self.wins + self.draws + self.losses
    }
}
