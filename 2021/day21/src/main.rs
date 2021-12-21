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
    println!("Part 1: {} [{:?}]", p1, start.elapsed()); //

    let start = Instant::now();
    let p2 = part2(player1, player2);
    println!("Part 2: {} [{:?}]", p2, start.elapsed()); //

    assert_eq!(p1, 908_595);
    assert_eq!(p2, 91_559_198_282_731);

    Ok(())
}

fn part1(mut p1: Player, mut p2: Player) -> u32 {
    const LIMIT: u32 = 1000;

    let mut die = Die::new();

    loop {
        let roll1 = die.roll();
        let roll2 = die.roll();
        let roll3 = die.roll();
        p1.forward(roll1 + roll2 + roll3);

        if p1.score >= LIMIT {
            return p2.score * die.rolls;
        }

        let roll1 = die.roll();
        let roll2 = die.roll();
        let roll3 = die.roll();
        p2.forward(roll1 + roll2 + roll3);

        if p2.score >= LIMIT {
            return p1.score * die.rolls;
        }
    }
}

type Cache = HashMap<([Player; 2], usize), [usize; 2]>;

fn part2(p1: Player, p2: Player) -> usize {
    let mut cache = Cache::new();
    let wins = roll_first([p1, p2], 0, &mut cache);

    wins[0].max(wins[1])
}

const LIMIT: u32 = 21;

fn roll_first(players: [Player; 2], player_idx: usize, cache: &mut Cache) -> [usize; 2] {
    if let Some(wins) = cache.get(&(players, player_idx)) {
        return *wins;
    }

    let init_players = players;
    let mut wins = [0, 0];

    for next_roll in 1..=3 {
        roll(players, player_idx, next_roll, 1, &mut wins, cache);
    }

    cache.insert((init_players, player_idx), wins);

    wins
}

fn roll(
    mut players: [Player; 2],
    player_idx: usize,
    sum: u32,
    idx: u32,
    wins: &mut [usize; 2],
    cache: &mut Cache,
) {
    if idx == 3 {
        players[player_idx].forward(sum);

        return if players[player_idx].score >= LIMIT {
            wins[player_idx] += 1;
        } else {
            let new_wins = roll_first(players, 1 - player_idx, cache);
            wins[0] += new_wins[0];
            wins[1] += new_wins[1];
        };
    } else {
        for next_roll in 1..=3 {
            roll(players, player_idx, sum + next_roll, idx + 1, wins, cache);
        }
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Player {
    pos: i16,
    score: u32,
}

impl Player {
    fn new(pos: u8) -> Self {
        Self {
            pos: pos as i16 - 1,
            score: 0,
        }
    }

    fn forward(&mut self, num: u32) {
        self.pos += num as i16;
        self.pos %= 10;
        self.score += (self.pos + 1) as u32;
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
