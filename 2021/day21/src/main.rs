use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        let mut e: &dyn Error = &*err;

        while let Some(src) = e.source() {
            eprintln!("  - caused by: {}", src);
            e = src;
        }
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input")?;
    let mut input = BufReader::new(file);

    let mut line = String::new();

    input.read_line(&mut line)?;
    let pos1 = *line.trim_end().as_bytes().last().unwrap() & 0x0F;
    line.clear();

    input.read_line(&mut line)?;
    let pos2 = *line.trim_end().as_bytes().last().unwrap() & 0x0F;
    line.clear();

    let player1 = Player::new(pos1);
    let player2 = Player::new(pos2);

    let start = Instant::now();
    let p1 = part1(player1, player2);
    println!("Part 1: {} [{:?}]", p1, start.elapsed()); // 1.1µs

    let start = Instant::now();
    let p2 = part2(player1, player2);
    println!("Part 2: {} [{:?}]", p2, start.elapsed()); // 12ms

    assert_eq!(p1, 908_595);
    assert_eq!(p2, 91_559_198_282_731);

    Ok(())
}

fn part1(mut p1: Player, mut p2: Player) -> u32 {
    const LIMIT: u32 = 1000;

    let mut die = Die::new();

    loop {
        if p1.forward(die.roll() + die.roll() + die.roll()) >= LIMIT {
            return p2.score * die.rolls;
        }

        if p2.forward(die.roll() + die.roll() + die.roll()) >= LIMIT {
            return p1.score * die.rolls;
        }
    }
}

type Cache = HashMap<([Player; 2], usize), [usize; 2]>;

fn part2(p1: Player, p2: Player) -> usize {
    let mut cache = Cache::new();
    let wins = roll([p1, p2], 0, &mut cache);

    wins[0].max(wins[1])
}

const LIMIT: u32 = 21;
const POSSIBS: [(u32, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn roll(players: [Player; 2], player: usize, cache: &mut Cache) -> [usize; 2] {
    if let Some(wins) = cache.get(&(players, player)) {
        return *wins;
    }

    let mut total_wins = [0, 0];

    for (sum, possibs) in POSSIBS {
        let mut players = players;

        if players[player].forward(sum) >= LIMIT {
            total_wins[player] += possibs;
        } else {
            let wins = roll(players, 1 - player, cache);

            for (total, win) in total_wins.iter_mut().zip(wins) {
                *total += win * possibs;
            }
        }
    }

    *cache.entry((players, player)).or_insert(total_wins)
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Player {
    pos: u32,
    score: u32,
}

impl Player {
    fn new(pos: u8) -> Self {
        Self {
            pos: pos as u32,
            score: 0,
        }
    }

    fn forward(&mut self, num: u32) -> u32 {
        self.pos = ((self.pos + num - 1) % 10) + 1;
        self.score += self.pos;

        self.score
    }
}

struct Die {
    curr: u32,
    rolls: u32,
}

impl Die {
    fn new() -> Self {
        Self { curr: 0, rolls: 0 }
    }

    fn roll(&mut self) -> u32 {
        self.curr += 1;
        self.rolls += 1;

        if self.curr > 100 {
            self.curr = 1;
        }

        self.curr
    }
}
